use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Form;
use log::info;

use crate::forms::*;
use crate::types::*;

pub fn test() -> HttpResponse {
    HttpResponse::Ok().body("Worked!")
}

pub fn generate_winning_post(_req: HttpRequest, form: Form<GenerateWinningPostData>) -> HttpResponse {
    info!("generate_winning_post");
    unimplemented!()
}