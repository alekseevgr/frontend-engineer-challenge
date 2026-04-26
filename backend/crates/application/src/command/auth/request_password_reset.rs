use std::sync::Arc;

use chrono::{Duration, Utc};
use domain::user::{repository::UserRepository, value_objects::email::Email};
use uuid::Uuid;

use crate::{
    command::auth::reset_password::PasswordResetError,
    dto::requests::auth::request_password_reset::RequestPasswordResetDto, ports::mailer::Mailer,
};

pub struct RequestPasswordReset<T, M>
where
    T: UserRepository,
    M: Mailer,
{
    user_repository: Arc<T>,
    mailer: Arc<M>,
    token_ttl_hours: i64,
}

impl<T, M> RequestPasswordReset<T, M>
where
    T: UserRepository,
    M: Mailer,
{
    pub fn new(user_repository: Arc<T>, mailer: Arc<M>, token_ttl_hours: i64) -> Self {
        Self {
            user_repository,
            mailer,
            token_ttl_hours,
        }
    }

    pub async fn execute(
        &self,
        request: RequestPasswordResetDto,
    ) -> Result<(), PasswordResetError> {
        let email = Email::new(request.email);
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::hours(self.token_ttl_hours);

        self.user_repository
            .save_password_reset_token(&email, &token, expires_at)
            .await?;

        self.mailer
            .send_password_reset(&email.as_str().to_string(), &token)
            .await?;

        Ok(())
    }
}
