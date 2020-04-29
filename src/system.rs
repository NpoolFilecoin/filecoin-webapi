use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::post;
use log::info;
use serde::Deserialize;
use serde_json::json;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::polling::*;
// use crate::types::*;

pub async fn test() -> HttpResponse {
    HttpResponse::Ok().body("Worked!")
}

pub async fn test_polling(state: Data<Mutex<ServState>>) -> HttpResponse {
    info!("generate_winning_post_sector_challenge polling");

    let (tx, rx) = channel();
    let handle: JoinHandle<()> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(30));
        let r = "Ok!!!";

        tx.send(json!(r)).unwrap();
    });

    let response = state.lock().unwrap().enqueue(handle, rx);
    HttpResponse::Ok().json(response)
}

pub async fn query_state(state: Data<Mutex<ServState>>, token: Json<u64>) -> HttpResponse {
    let response = state.lock().unwrap().get(*token);

    HttpResponse::Ok().json(response)
}
