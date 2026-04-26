use application::{
    command::auth::{
        register::RegisterUser, request_password_reset::RequestPasswordReset,
        reset_password::ResetPassword,
    },
    query::auth::login::LoginUser,
};
use infrastructure::{
    database::postgres::repositories::user::PostgresUserRepository, hasher::bcrypt::BcryptHasher,
    mailer::smtp_mailer::SmtpMailer,
};

pub struct AppState {
    pub register_user: RegisterUser<PostgresUserRepository, BcryptHasher>,
    pub login_user: LoginUser<PostgresUserRepository, BcryptHasher>,
    pub request_password_reset: RequestPasswordReset<PostgresUserRepository, SmtpMailer>,
    pub reset_password: ResetPassword<PostgresUserRepository, BcryptHasher>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(
        register_user: RegisterUser<PostgresUserRepository, BcryptHasher>,
        login_user: LoginUser<PostgresUserRepository, BcryptHasher>,
        request_password_reset: RequestPasswordReset<PostgresUserRepository, SmtpMailer>,
        reset_password: ResetPassword<PostgresUserRepository, BcryptHasher>,
        jwt_secret: String,
    ) -> Self {
        Self {
            register_user,
            login_user,
            request_password_reset,
            reset_password,
            jwt_secret,
        }
    }
}
