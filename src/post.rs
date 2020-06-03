use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::post;
use log::trace;

use crate::post_data::*;

pub async fn generate_winning_post_sector_challenge(
    _req: HttpRequest,
    data: Json<GenerateWinningPostSectorChallengeData>,
) -> HttpResponse {
    trace!("generate_winning_post_sector_challenge: {:?}", data);

    let r = post::generate_winning_post_sector_challenge(
        data.proof_type,
        &data.randomness,
        data.sector_set_len,
        data.prover_id,
    );

    let response = r.map_err(|e| format!("{:?}", e));
    trace!("generate_winning_post_sector_challenge finish: {:?}", response);
    HttpResponse::Ok().json(response)
}

pub async fn generate_winning_post(_req: HttpRequest, data: Json<GenerateWinningPostData>) -> HttpResponse {
    trace!("generate_winning_post: {:?}", data);

    let r = post::generate_winning_post(&data.randomness, &data.replicas.as_object(), data.prover_id);

    let response = r.map_err(|e| format!("{:?}", e));
    trace!("generate_winning_post finish: {:?}", response);
    HttpResponse::Ok().json(response)
}

pub async fn verify_winning_post(_req: HttpRequest, data: Json<VerifyWinningPostData>) -> HttpResponse {
    trace!("verify_winning_post: {:?}", data);

    let r = post::verify_winning_post(
        &data.randomness,
        &data.proof,
        &data.replicas.as_object(),
        data.prover_id,
    );

    let response = r.map_err(|e| format!("{:?}", e));
    trace!("verify_winning_post finish: {:?}", response);
    HttpResponse::Ok().json(response)
}

pub async fn generate_window_post(_req: HttpRequest, data: Json<GenerateWindowPostData>) -> HttpResponse {
    trace!("generate_window_post: {:?}", data);

    let r = post::generate_window_post(&data.randomness, &data.replicas.as_object(), data.prover_id);

    let response = r.map_err(|e| format!("{:?}", e));
    trace!("generate_window_post finish: {:?}", response);
    HttpResponse::Ok().json(response)
}

pub async fn verify_window_post(_req: HttpRequest, data: Json<VerifyWindowPostData>) -> HttpResponse {
    trace!("verify_window_post: {:?}", data);

    let proofs: Vec<_> = data.proof.iter().map(|(x, y)| (*x, y.as_slice())).collect();

    let r = post::verify_window_post(
        &data.randomness,
        proofs.as_slice(),
        &data.replicas.as_object(),
        data.prover_id,
    );

    let response = r.map_err(|e| format!("{:?}", e));
    trace!("verify_window_post finish: {:?}", response);
    HttpResponse::Ok().json(response)
}
