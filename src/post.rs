use actix_web::web::{Form, Json};
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::post;
use json::JsonValue;
use log::info;

use crate::post_data::*;
use crate::types::*;

pub async fn test() -> HttpResponse {
    HttpResponse::Ok().body("Worked!")
}

pub async fn generate_winning_post_sector_challenge(
    _req: HttpRequest,
    data: Json<GenerateWinningPostSectorChallengeData>,
) -> HttpResponse {
    info!("generate_winning_post_sector_challenge");

    let r = post::generate_winning_post_sector_challenge(
        data.proof_type,
        &data.randomness,
        data.sector_set_len,
        data.prover_id,
    );

    HttpResponse::Ok().json(r.map_err(|_| "Error"))
}

pub async fn generate_winning_post(_req: HttpRequest, form: Form<GenerateWinningPostData>) -> HttpResponse {
    info!("generate_winning_post");

    let r = post::generate_winning_post(&form.randomness, &form.replicas.as_object(), form.prover_id);

    HttpResponse::Ok().json(r.map_err(|_| "Error"))
}

pub async fn verify_winning_post(_req: HttpRequest, data: Json<VerifyWinningPostData>) -> HttpResponse {
    info!("verify_winning_post");

    let r = post::verify_winning_post(
        &data.randomness,
        data.proof.as_bytes(),
        &data.replicas.as_object(),
        data.prover_id,
    );

    HttpResponse::Ok().json(r.map_err(|_| "Error"))
}
