use actix_web::{middleware, web};
use actix_web::{App, HttpServer};

mod forms;
mod handle;
mod types;

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
            .service(web::resource("/test").route(web::get().to(handle::test)))
            .service(web::resource("/generate_winning_post").route(web::post().to(handle::generate_winning_post)))
    })
    .bind("localhost:8888")
    .expect("Bind failed")
    .run()
    .await
}
