use std::env;
use std::sync::{Arc, Mutex};

// use actix_web::FromRequest;
use actix_web::{error, middleware, web};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer};
use log::{error, warn};

// use crate::seal_data::SealCommitPhase2Data;
use polling::ServState;

mod polling;
pub mod post;
pub mod post_data;
pub mod seal;
pub mod seal_data;
mod system;
mod types;

#[allow(dead_code)]
fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    error!("{:?}", err);

    let detail = err.to_string();
    let response = match &err {
        error::JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType()
            .content_type("text/plain")
            .body(detail),
        _ => HttpResponse::BadRequest().content_type("text/plain").body(detail),
    };
    error::InternalError::from_response(err, response).into()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let bind_address = &args[1];

    if std::env::var("RUST_LOG").is_err() {
        if cfg!(debug_assertions) {
            std::env::set_var("RUST_LOG", "filecoin_webapi=trace,actix_web=info");
        } else {
            std::env::set_var("RUST_LOG", "filecoin_webapi=info,actix_web=warn");
        }
    }

    fil_logger::init();
    std::fs::create_dir_all("/tmp/upload/")?;
    let state = Arc::new(Mutex::new(ServState::new()));

    warn!("Listening: {}", bind_address);

    HttpServer::new(move || {
        let state = state.clone();

        App::new()
            .app_data(web::Data::new(state))
            .wrap(middleware::Logger::default())
            .service(web::resource("/test").route(web::get().to(system::test)))
            .service(web::resource("/sys/test_polling").route(web::post().to(system::test_polling)))
            .service(web::resource("/sys/query_state").route(web::post().to(system::query_state)))
            .service(web::resource("/sys/remove_job").route(web::post().to(system::remove_job)))
            .service(web::resource("/sys/upload_file").route(web::post().to(system::upload_file)))
            .service(web::resource("/sys/upload_test").route(web::get().to(system::upload_test)))
            .service(
                web::resource("/post/generate_winning_post_sector_challenge")
                    .route(web::post().to(post::generate_winning_post_sector_challenge)),
            )
            .service(web::resource("/post/generate_winning_post").route(web::post().to(post::generate_winning_post)))
            .service(web::resource("/post/verify_winning_post").route(web::post().to(post::verify_winning_post)))
            .service(web::resource("/post/generate_window_post").route(web::post().to(post::generate_window_post)))
            .service(web::resource("/post/verify_window_post").route(web::post().to(post::verify_window_post)))
            .service(web::resource("/seal/clear_cache").route(web::post().to(seal::clear_cache)))
            .service(web::resource("/seal/seal_pre_commit_phase1").route(web::post().to(seal::seal_pre_commit_phase1)))
            .service(web::resource("/seal/seal_pre_commit_phase2").route(web::post().to(seal::seal_pre_commit_phase2)))
            .service(web::resource("/seal/compute_comm_d").route(web::post().to(seal::compute_comm_d)))
            .service(web::resource("/seal/seal_commit_phase1").route(web::post().to(seal::seal_commit_phase1)))
            .service(
                web::resource("/seal/seal_commit_phase2")
                    // .app_data(web::Json::<SealCommitPhase2Data>::configure(|cfg| {
                    // cfg.limit(1024000)
                    //         .content_type(|_mime| true)
                    //         .error_handler(json_error_handler)
                    // }))
                    .route(web::post().to(seal::seal_commit_phase2)),
            )
            .service(web::resource("/seal/verify_seal").route(web::post().to(seal::verify_seal)))
            .service(web::resource("/seal/verify_batch_seal").route(web::post().to(seal::verify_batch_seal)))
            .service(web::resource("/seal/get_unsealed_range").route(web::post().to(seal::get_unsealed_range)))
            .service(
                web::resource("/seal/generate_piece_commitment").route(web::post().to(seal::generate_piece_commitment)),
            )
            .service(web::resource("/seal/add_piece").route(web::post().to(seal::add_piece)))
            .service(web::resource("/seal/write_and_preprocess").route(web::post().to(seal::write_and_preprocess)))
    })
    .bind(bind_address)
    .expect("Bind failed")
    .run()
    .await
}
