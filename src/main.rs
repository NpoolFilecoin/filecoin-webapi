
use actix_web::{middleware};
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
    })
    .bind("localhost:8888")
    .expect("Bind failed")
    .run()
    .await
}
