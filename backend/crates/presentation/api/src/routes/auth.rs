use actix_web::web;

use crate::handlers::auth::{
    login_user, refresh_tokens, register_user, request_password_reset, reset_password,
};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .service(register_user)
            .service(login_user)
            .service(refresh_tokens)
            .service(request_password_reset)
            .service(reset_password),
    );
}
