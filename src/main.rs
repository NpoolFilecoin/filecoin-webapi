use actix_web::web::JsonConfig;
use actix_web::{error, middleware, web};
use actix_web::{App, FromRequest, HttpRequest, HttpResponse, HttpServer};

#[macro_use]
extern crate slice_as_array;

mod post;
mod post_data;
mod seal;
mod types;

use post_data::GenerateWinningPostData;

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    println!("{:?}", err);

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
            std::env::set_var("RUST_LOG", "filecoin_webapi=info,actix_web=info");
        } else {
            std::env::set_var("RUST_LOG", "filecoin_webapi=warn,actix_web=warn");
        }
    }

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/test").route(web::get().to(post::test)))
            .service(web::resource("/post/test").route(web::post().to(post::post_test)))
            .service(
                web::resource("/post/generate_winning_post_sector_challenge")
                    .route(web::post().to(post::generate_winning_post_sector_challenge)),
            )
            .service(
                web::resource("/post/generate_winning_post")
                    // .app_data(web::Json::<GenerateWinningPostData>::configure(|cfg| {
                    //     cfg.limit(4096)
                    //         .content_type(|mime| mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN)
                    //         .error_handler(json_error_handler)
                    // }))
                    .route(web::post().to(post::generate_winning_post)),
            )
            .service(web::resource("/post/verify_winning_post").route(web::post().to(post::verify_winning_post)))
            .service(web::resource("/post/generate_window_post").route(web::post().to(post::generate_window_post)))
            .service(web::resource("/post/verify_window_post").route(web::post().to(post::verify_window_post)))
    })
    .bind("[::]:8888")
    .expect("Bind failed")
    .run()
    .await
}
