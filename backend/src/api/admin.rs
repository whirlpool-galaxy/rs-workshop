use actix_web::{
    get, post,
    web::{scope, ServiceConfig},
    HttpResponse, Responder,
};

pub fn admin_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/admin")
            .service(signin)
            .service(selection_open)
            .service(selection_close)
            .service(selection_publish)
            .service(accesscodes_generate)
            .service(accesscodes_batch)
            .service(accesscodes_all)
            .service(accesscodes_user)
            .service(schedule_rooms)
            .service(schedule),
    );
}

#[post("/signin")]
async fn signin() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/selection/open")]
async fn selection_open() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/selection/close")]
async fn selection_close() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/selection/publish")]
async fn selection_publish() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/accesscodes/generate/{nocodes}")]
async fn accesscodes_generate() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/accesscodes/batch/{batch}")]
async fn accesscodes_batch() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/accesscodes/all")]
async fn accesscodes_all() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/accesscodes/u/{username}")]
async fn accesscodes_user() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/schedule")]
async fn schedule() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/schedule/rooms")]
async fn schedule_rooms() -> impl Responder {
    HttpResponse::Ok()
}
