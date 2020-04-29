#[macro_use]
extern crate lazy_static;

use actix_web::{error, middleware, web};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer};
use log::error;
use std::sync::Mutex;

mod polling;
mod post;
mod post_data;
mod seal;
mod system;
mod types;

use polling::ServState;

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
    if std::env::var("RUST_LOG").is_err() {
        if cfg!(debug_assertions) {
            std::env::set_var("RUST_LOG", "filecoin_webapi=trace,actix_web=info");
        } else {
            std::env::set_var("RUST_LOG", "filecoin_webapi=info,actix_web=warn");
        }
    }

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Mutex::new(ServState::new())))
            .wrap(middleware::Logger::default())
            .service(web::resource("/test").route(web::get().to(system::test)))
            .service(web::resource("/test_polling").route(web::post().to(system::test_polling)))
            .service(web::resource("/query_state").route(web::post().to(system::query_state)))
            .service(web::resource("/remove_job").route(web::post().to(system::remove_job)))
            .service(
                web::resource("/post/generate_winning_post_sector_challenge")
                    // .app_data(web::Json::<GenerateWinningPostData>::configure(|cfg| {
                    //     cfg.limit(4096)
                    //         .content_type(|mime| mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN)
                    //         .error_handler(json_error_handler)
                    // }))
                    .route(web::post().to(post::generate_winning_post_sector_challenge)),
            )
            .service(web::resource("/post/generate_winning_post").route(web::post().to(post::generate_winning_post)))
            .service(web::resource("/post/verify_winning_post").route(web::post().to(post::verify_winning_post)))
            .service(web::resource("/post/generate_window_post").route(web::post().to(post::generate_window_post)))
            .service(web::resource("/post/verify_window_post").route(web::post().to(post::verify_window_post)))
    })
    .bind("[::]:8888")
    .expect("Bind failed")
    .run()
    .await
}
