use actix_web::{App, HttpServer};
use api::{admin::admin_config, participant::participant_config};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(admin_config)
            .configure(participant_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
