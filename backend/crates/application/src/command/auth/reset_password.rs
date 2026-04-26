use std::sync::Arc;

use chrono::Utc;
use domain::{
    shared::{repository::error::RepositoryError, value_objects::password_hash::PasswordHash},
    user::{repository::UserRepository, value_objects::id::UserId},
};

use crate::{
    dto::requests::auth::reset_password::ResetPasswordDto,
    ports::{
        hasher::{HashError, Hasher},
        mailer::MailerError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum PasswordResetError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Hash error: {0}")]
    Hash(#[from] HashError),
    #[error("Mailer error: {0}")]
    Mailer(#[from] MailerError),
    #[error("Invalid or expired token")]
    InvalidOrExpiredToken,
    #[error("Internal error")]
    Internal,
}

pub struct ResetPassword<T, H>
where
    T: UserRepository,
    H: Hasher,
{
    user_repository: Arc<T>,
    hasher: Arc<H>,
}

impl<T, H> ResetPassword<T, H>
where
    T: UserRepository,
    H: Hasher,
{
    pub fn new(user_repository: Arc<T>, hasher: Arc<H>) -> Self {
        Self {
            user_repository,
            hasher,
        }
    }

    pub async fn execute(&self, request: ResetPasswordDto) -> Result<UserId, PasswordResetError> {
        let (user_id, expires_at) = self
            .user_repository
            .get_user_id_and_expiry_by_password_reset_token(&request.token)
            .await
            .map_err(PasswordResetError::from)?;

        if expires_at < Utc::now() {
            return Err(PasswordResetError::InvalidOrExpiredToken);
        }

        let password_hash = PasswordHash::new(self.hasher.hash(&request.new_password)?);

        self.user_repository
            .update_password(&user_id, &password_hash)
            .await?;

        self.user_repository
            .invalidate_password_reset_token(&request.token)
            .await?;

        Ok(user_id)
    }
}
