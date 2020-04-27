mod types;
mod handle;
mod forms;

use actix_web::{middleware, web};
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

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
