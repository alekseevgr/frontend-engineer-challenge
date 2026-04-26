use utoipa::OpenApi;

use crate::handlers::auth::{
    __path_login_user, __path_refresh_tokens, __path_register_user,
    __path_request_password_reset, __path_reset_password, login_user, refresh_tokens,
    register_user, request_password_reset, reset_password, LoginSuccessResponse,
    MessageSuccessResponse, RegisterUserSuccessResponse,
};
use crate::shared::dto::requests::auth::{
    login::LoginUserRequest, refresh_token::RefreshTokenRequest,
    registration::RegisterUserRequest, request_password_reset::RequestPasswordResetRequest,
    reset_password::ResetPasswordRequest,
};
use crate::shared::dto::responses::auth::registration::RegisterUserResponse;
use crate::shared::error::ErrorResponse;
use shared::jwt::claims::TokenPair;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Atlantis Authentication API",
        version = "1.0.0",
        description = "Production-ready authentication and authorization service built with Rust and REST API architecture.\n\n\
        This service provides secure user registration, JWT authentication, and password reset functionality.\n\n\
        ## Features\n\
        - User registration and authentication\n\
        - JWT token-based authentication (access and refresh tokens)\n\
        - Password reset flow via email\n\
        - Rate limiting protection\n\
        - Comprehensive error handling\n\n\
        ## Authentication\n\
        Most endpoints require a valid JWT access token in the Authorization header:\n\
        ```\n\
        Authorization: Bearer <access_token>\n\
        ```\n\n\
        Access tokens expire after 15 minutes. Use the refresh token endpoint to obtain new tokens.",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    paths(
        register_user,
        login_user,
        refresh_tokens,
        request_password_reset,
        reset_password,
    ),
    components(
        schemas(
            RegisterUserRequest,
            LoginUserRequest,
            RefreshTokenRequest,
            RequestPasswordResetRequest,
            ResetPasswordRequest,
            RegisterUserResponse,
            TokenPair,
            ErrorResponse,
            RegisterUserSuccessResponse,
            LoginSuccessResponse,
            MessageSuccessResponse,
        )
    ),
    tags(
        (name = "Authentication", description = "User authentication and token management endpoints"),
        (name = "Password Reset", description = "Password reset flow endpoints")
    )
)]
pub struct ApiDoc;
