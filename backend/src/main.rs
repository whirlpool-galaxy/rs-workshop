use actix_web::{web::Data, App, HttpServer};
use api::{admin::admin_config, participant::participant_config};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod api;

struct DataBase {
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rs-workshop:rs-workshop@localhost/rs-workshop")
        .await
        .expect("Cannot connect to Database!");

    HttpServer::new(move || {
        App::new()
            .configure(admin_config)
            .configure(participant_config)
            .app_data(Data::new(DataBase {
                db: db_pool.clone(),
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
