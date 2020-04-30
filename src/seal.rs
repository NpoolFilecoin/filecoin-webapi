use crate::seal_data::*;

use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::{seal, PieceInfo};
use log::trace;
use std::path::Path;

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

pub async fn seal_commit_phase1(data: Json<SealCommitPhase1Data>) -> HttpResponse {
    trace!("seal_commit_phase1");

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

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
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
