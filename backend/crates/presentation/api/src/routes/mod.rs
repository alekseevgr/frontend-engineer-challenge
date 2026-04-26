use actix_web::web;

pub mod auth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(auth::auth_routes));
}
