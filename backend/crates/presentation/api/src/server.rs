use std::{error::Error, sync::Arc};

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use application::{
    command::auth::{
        register::RegisterUser, request_password_reset::RequestPasswordReset,
        reset_password::ResetPassword,
    },
    query::auth::login::LoginUser,
};
use infrastructure::{
    config::{mailer::MailerConfig, postgres::PostgresConfig},
    database::postgres::{
        connection::create_postgres_pool, repositories::user::PostgresUserRepository,
    },
    hasher::bcrypt::BcryptHasher,
    mailer::smtp_mailer::SmtpMailer,
};
use shared::rate_limiting::{
    config::RateLimitingConfig, create_governor_config, create_rate_limiting,
};

use crate::{config::ServerConfig, openapi::ApiDoc, routes, shared::state::AppState};

pub async fn run() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server_config = ServerConfig::from_env()?;
    let postgres_config = PostgresConfig::from_env()?;
    let rate_limiting_config = RateLimitingConfig::from_env()?;
    let mailer_config = MailerConfig::from_env()?;

    let postgres_pool = create_postgres_pool(postgres_config).await?;

    let user_repository = Arc::new(PostgresUserRepository::new(postgres_pool.clone()));

    let hasher = Arc::new(BcryptHasher::default());
    let mailer = Arc::new(SmtpMailer::from_config(mailer_config)?);

    let register_user = RegisterUser::new(user_repository.clone(), hasher.clone());
    let login_user = LoginUser::new(user_repository.clone(), hasher.clone());
    let request_password_reset =
        RequestPasswordReset::new(user_repository.clone(), mailer.clone(), 24); // 24 часов TTL
    let reset_password = ResetPassword::new(user_repository.clone(), hasher.clone());

    let app_state = web::Data::new(AppState::new(
        register_user,
        login_user,
        request_password_reset,
        reset_password,
        server_config.jwt_secret,
    ));

    let governor_config = create_governor_config(rate_limiting_config)
        .ok_or("Failed to create rate limiting configuration: invalid parameters")?;

    // Generate OpenAPI spec
    let openapi = ApiDoc::openapi();

    tracing::info!(
        "Swagger UI available at http://{}:{}/swagger-ui/",
        server_config.host, server_config.port
    );
    tracing::info!(
        "OpenAPI JSON available at http://{}:{}/api-docs/openapi.json",
        server_config.host, server_config.port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let rate_limiting = create_rate_limiting(&governor_config);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(rate_limiting)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .configure(routes::configure)
    })
    .bind((server_config.host, server_config.port))?
    .run()
    .await?;

    Ok(())
}
