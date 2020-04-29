use std::path::Path;

use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::{seal, PieceInfo};
use log::trace;

use crate::seal_data::*;

pub async fn clear_cache(_req: HttpRequest, data: Json<ClearCacheData>) -> HttpResponse {
    trace!("clear_cache");

    let r = seal::clear_cache(data.sector_size, Path::new(&data.cache_path));

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}

pub async fn seal_pre_commit_phase1(data: Json<SealPreCommitPhase1Data>) -> HttpResponse {
    trace!("seal_pre_commit_phase1");

    let piece_info: Vec<PieceInfo> = data.piece_infos.iter().map(|x| x.as_object()).collect();

    let r = seal::seal_pre_commit_phase1(
        data.registered_proof,
        &data.cache_path,
        &data.in_path,
        &data.out_path,
        data.prover_id,
        data.sector_id,
        data.ticket,
        &piece_info[..],
    );

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}
