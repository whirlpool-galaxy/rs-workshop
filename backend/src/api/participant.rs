use actix_web::{
    get, post,
    web::{scope, ServiceConfig},
    HttpResponse, Responder,
};

pub fn participant_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/participant")
            .service(signin)
            .service(workshops_list)
            .service(workshops_selection)
            .service(workshops_priority)
            .service(workshops_priority_commit),
    );
}

#[post("/signin")]
async fn signin() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/workshops/list")]
async fn workshops_list() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/workshops/selection")]
async fn workshops_selection() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/workshops/priority")]
async fn workshops_priority() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/workshops/priority/commit")]
async fn workshops_priority_commit() -> impl Responder {
    HttpResponse::Ok()
}
