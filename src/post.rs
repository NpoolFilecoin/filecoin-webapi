use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::post;
use json::JsonValue;
use log::info;
use serde::Deserialize;

use crate::post_data::*;
use crate::types::*;

pub async fn test() -> HttpResponse {
    HttpResponse::Ok().body("Worked!")
}

#[derive(Deserialize, Clone, Debug)]
pub enum TestEnum {
    A,
    B,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TestData {
    a: u64,
    e: TestEnum,
}

pub async fn post_test(_req: HttpRequest, data: Json<TestData>) -> HttpResponse {
    info!("post_test");
    HttpResponse::Ok().body(format!("{:?}", data))
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

pub async fn generate_winning_post(_req: HttpRequest, data: Json<GenerateWinningPostData>) -> HttpResponse {
    info!("generate_winning_post");

    // HttpResponse::Ok().body(format!("{:?}", data))
    let r = post::generate_winning_post(&data.randomness, &data.replicas.as_object(), data.prover_id);

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
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

pub async fn generate_window_post(_req: HttpRequest, data: Json<GenerateWindowPostData>) -> HttpResponse {
    info!("generate_window_post");

    let r = post::generate_window_post(&data.randomness, &data.replicas.as_object(), data.prover_id);

    HttpResponse::Ok().json(r.map_err(|_| "Error"))
}

pub async fn verify_window_post(_req: HttpRequest, data: Json<VerifyWindowPostData>) -> HttpResponse {
    info!("verify_window_post");

    let r = post::verify_window_post(
        &data.randomness,
        data.proof.as_bytes(),
        &data.replicas.as_object(),
        data.prover_id,
    );

    HttpResponse::Ok().json(r.map_err(|_| "Error"))
}
