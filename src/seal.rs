use std::fs::OpenOptions;
use std::io::Result;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::{seal, PieceInfo};
use log::trace;
use serde_json::json;

use crate::polling::*;
use crate::seal_data::*;
use crate::types::WebPieceInfo;

pub async fn clear_cache(_req: HttpRequest, data: Json<ClearCacheData>) -> HttpResponse {
    trace!("clear_cache");

    let r = seal::clear_cache(data.sector_size, Path::new(&data.cache_path));

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn seal_pre_commit_phase1(data: Json<SealPreCommitPhase1Data>) -> HttpResponse {
    trace!("seal_pre_commit_phase1");

    let piece_infos: Vec<PieceInfo> = data.piece_infos.iter().map(|x| x.as_object()).collect();

    let r = seal::seal_pre_commit_phase1(
        data.registered_proof,
        &data.cache_path,
        &data.in_path,
        &data.out_path,
        data.prover_id,
        data.sector_id,
        data.ticket,
        &piece_infos[..],
    );

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn seal_pre_commit_phase2(data: Json<SealPreCommitPhase2Data>) -> HttpResponse {
    trace!("seal_pre_commit_phase2");

    let r = seal::seal_pre_commit_phase2(data.phase1_output.clone(), &data.cache_path, &data.out_path);

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn compute_comm_d(data: Json<ComputeCommDData>) -> HttpResponse {
    trace!("compute_comm_d");

    let piece_infos: Vec<PieceInfo> = data.piece_infos.iter().map(|x| x.as_object()).collect();

    let r = seal::compute_comm_d(data.registered_proof, &piece_infos[..]);

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn seal_commit_phase1(state: Data<Arc<Mutex<ServState>>>, data: Json<SealCommitPhase1Data>) -> HttpResponse {
    trace!("seal_commit_phase1");

    let (tx, rx) = channel();
    let handle: JoinHandle<()> = thread::spawn(move || {
        let piece_infos: Vec<PieceInfo> = data.piece_infos.iter().map(|x| x.as_object()).collect();

        let r = seal::seal_commit_phase1(
            &data.cache_path,
            &data.replica_path,
            data.prover_id,
            data.sector_id,
            data.ticket,
            data.seed,
            data.pre_commit.clone(),
            &piece_infos[..],
        );

        tx.send(json!(r.map_err(|e| format!("{:?}", e)))).unwrap();
    });

    let response = state.lock().unwrap().enqueue(handle, rx);
    HttpResponse::Ok().json(response)
}

pub async fn seal_commit_phase2(data: Json<SealCommitPhase2Data>) -> HttpResponse {
    trace!("seal_commit_phase2");

    let r = seal::seal_commit_phase2(data.phase1_output.clone(), data.prover_id, data.sector_id);

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn verify_seal(data: Json<VerifySealData>) -> HttpResponse {
    trace!("verify_seal");

    let r = seal::verify_seal(
        data.registered_proof,
        data.comm_r_in,
        data.comm_d_in,
        data.prover_id,
        data.sector_id,
        data.ticket,
        data.seed,
        data.proof_vec.as_slice(),
    );

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn verify_batch_seal(data: Json<VerifyBatchSealData>) -> HttpResponse {
    trace!("verify_batch_seal");

    let proof_vecs: Vec<_> = data.proof_vecs.iter().map(|x| x.as_slice()).collect();

    let r = seal::verify_batch_seal(
        data.registered_proof,
        data.comm_r_ins.as_slice(),
        data.comm_d_ins.as_slice(),
        data.prover_ids.as_slice(),
        data.sector_ids.as_slice(),
        data.tickets.as_slice(),
        data.seeds.as_slice(),
        proof_vecs.as_slice(),
    );

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn get_unsealed_range(data: Json<GetUnsealedRangeData>) -> HttpResponse {
    trace!("get_unsealed_range");

    let r = seal::get_unsealed_range(
        data.registered_proof,
        &data.cache_path,
        &data.sealed_path,
        &data.output_path,
        data.prover_id,
        data.sector_id,
        data.comm_d,
        data.ticket,
        data.offset,
        data.num_bytes,
    );

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn generate_piece_commitment(data: Json<GeneratePieceCommitmentData>) -> Result<HttpResponse> {
    trace!("generate_piece_commitment");

    let source = OpenOptions::new().read(true).open(&data.source)?;
    let r = seal::generate_piece_commitment(data.registered_proof, source, data.piece_size);

    Ok(HttpResponse::Ok().json(r.map(|x| WebPieceInfo::from_object(x)).map_err(|e| format!("{:?}", e))))
}

pub async fn add_piece(data: Json<AddPieceData>) -> Result<HttpResponse> {
    trace!("add_piece");

    let source = OpenOptions::new().read(true).open(&data.source)?;
    let target = OpenOptions::new().write(true).open(&data.target)?;
    let r = seal::add_piece(
        data.registered_proof,
        source,
        target,
        data.piece_size,
        &data.piece_lengths[..],
    );

    Ok(HttpResponse::Ok().json(
        r.map(|(x, y)| AddPieceOutput::from_object((x, y)))
            .map_err(|e| format!("{:?}", e)),
    ))
}

pub async fn write_and_preprocess(data: Json<WriteAndPreprocessData>) -> Result<HttpResponse> {
    trace!("write_and_preprocess");

    let source = OpenOptions::new().read(true).open(&data.source)?;
    let target = OpenOptions::new().write(true).open(&data.target)?;
    let r = seal::write_and_preprocess(data.registered_proof, source, target, data.piece_size);

    Ok(HttpResponse::Ok().json(
        r.map(|(x, y)| WriteAndPreprocessOutput::from_object((x, y)))
            .map_err(|e| format!("{:?}", e)),
    ))
}
