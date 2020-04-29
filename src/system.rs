use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use log::trace;
use serde_json::json;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::polling::*;

pub async fn test() -> HttpResponse {
    trace!("test");

    HttpResponse::Ok().body("Worked!")
}

pub async fn test_polling(state: Data<Mutex<ServState>>) -> HttpResponse {
    trace!("test polling");

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
    trace!("query_state");

    let response = state.lock().unwrap().get(*token);

    HttpResponse::Ok().json(response)
}

pub async fn remove_job(state: Data<Mutex<ServState>>, token: Json<u64>) -> HttpResponse {
    trace!("remove_job");

    let response = state.lock().unwrap().remove(*token);

    HttpResponse::Ok().json(response)
}
