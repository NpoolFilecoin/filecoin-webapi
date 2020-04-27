use actix_web::web::Form;
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::post;
use log::info;

use crate::forms::*;
use crate::types::*;

pub fn test() -> HttpResponse {
    HttpResponse::Ok().body("Worked!")
}

pub fn generate_winning_post(_req: HttpRequest, form: Form<GenerateWinningPostData>) -> HttpResponse {
    info!("generate_winning_post");

    let _r = post::generate_winning_post(&form.randomness, &form.replicas.as_object(), form.prover_id);

    HttpResponse::Ok().finish()
}
