use actix_web::{HttpResponse, ResponseError, http::StatusCode};

use application::{
    command::auth::register::RegistrationError, ports::hasher::HashError,
    query::auth::login::LoginError,
};
use domain::shared::repository::error::RepositoryError;
use serde::Serialize;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message describing what went wrong
    #[schema(example = "Invalid credentials")]
    pub message: String,
    /// HTTP status code
    #[schema(example = 400)]
    pub status: u16,
}

pub trait ToHttpStatus {
    fn to_http_status(&self) -> StatusCode;
    fn to_user_message(&self) -> String;
}

impl ToHttpStatus for RepositoryError {
    fn to_http_status(&self) -> StatusCode {
        match self {
            RepositoryError::NotFound => StatusCode::NOT_FOUND,
            RepositoryError::AlreadyExists => StatusCode::CONFLICT,
            RepositoryError::ConnectionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RepositoryError::InvalidData(_) => StatusCode::BAD_REQUEST,
            RepositoryError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn to_user_message(&self) -> String {
        match self {
            RepositoryError::NotFound => "Resource not found".to_string(),
            RepositoryError::AlreadyExists => "Resource already exists".to_string(),
            RepositoryError::ConnectionError(_) => "Service temporarily unavailable".to_string(),
            RepositoryError::InvalidData(_) => "Invalid data provided".to_string(),
            RepositoryError::Internal(_) => "Internal server error".to_string(),
        }
    }
}

impl ToHttpStatus for HashError {
    fn to_http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn to_user_message(&self) -> String {
        "Authentication error".to_string()
    }
}

impl ToHttpStatus for LoginError {
    fn to_http_status(&self) -> StatusCode {
        match self {
            LoginError::Repository(e) => e.to_http_status(),
            LoginError::Hash(e) => e.to_http_status(),
            LoginError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            LoginError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn to_user_message(&self) -> String {
        match self {
            LoginError::Repository(e) => e.to_user_message(),
            LoginError::Hash(e) => e.to_user_message(),
            LoginError::InvalidCredentials => "Invalid credentials".to_string(),
            LoginError::Internal => "Internal server error".to_string(),
        }
    }
}

impl ToHttpStatus for RegistrationError {
    fn to_http_status(&self) -> StatusCode {
        match self {
            RegistrationError::Repository(e) => e.to_http_status(),
            RegistrationError::Hash(e) => e.to_http_status(),
        }
    }

    fn to_user_message(&self) -> String {
        match self {
            RegistrationError::Repository(e) => e.to_user_message(),
            RegistrationError::Hash(e) => e.to_user_message(),
        }
    }
}

impl ToHttpStatus for application::command::auth::reset_password::PasswordResetError {
    fn to_http_status(&self) -> StatusCode {
        match self {
            application::command::auth::reset_password::PasswordResetError::Repository(e) => e.to_http_status(),
            application::command::auth::reset_password::PasswordResetError::Hash(e) => e.to_http_status(),
            application::command::auth::reset_password::PasswordResetError::Mailer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            application::command::auth::reset_password::PasswordResetError::InvalidOrExpiredToken => StatusCode::BAD_REQUEST,
            application::command::auth::reset_password::PasswordResetError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn to_user_message(&self) -> String {
        match self {
            application::command::auth::reset_password::PasswordResetError::Repository(e) => e.to_user_message(),
            application::command::auth::reset_password::PasswordResetError::Hash(e) => e.to_user_message(),
            application::command::auth::reset_password::PasswordResetError::Mailer(_) => "Failed to send email".to_string(),
            application::command::auth::reset_password::PasswordResetError::InvalidOrExpiredToken => "Invalid or expired token".to_string(),
            application::command::auth::reset_password::PasswordResetError::Internal => "Internal server error".to_string(),
        }
    }
}

impl From<application::command::auth::reset_password::PasswordResetError> for ApiError {
    fn from(error: application::command::auth::reset_password::PasswordResetError) -> Self {
        Self::from_error(error)
    }
}

#[derive(Debug)]
pub struct ApiError {
    message: String,
    status: StatusCode,
}

impl ApiError {
    pub fn new(message: String, status: StatusCode) -> Self {
        Self { message, status }
    }

    fn from_error<E: ToHttpStatus + std::fmt::Debug>(error: E) -> Self {
        tracing::error!("API Error: {:?}", error);
        Self {
            message: error.to_user_message(),
            status: error.to_http_status(),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            message: self.message.clone(),
            status: self.status.as_u16(),
        };
        HttpResponse::build(self.status).json(response)
    }
}

impl From<LoginError> for ApiError {
    fn from(error: LoginError) -> Self {
        Self::from_error(error)
    }
}

impl From<RegistrationError> for ApiError {
    fn from(error: RegistrationError) -> Self {
        Self::from_error(error)
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(error: ValidationErrors) -> Self {
        tracing::error!("Validation error: {:?}", error);
        Self {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}
