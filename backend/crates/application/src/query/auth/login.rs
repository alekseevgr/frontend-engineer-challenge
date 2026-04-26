use std::sync::Arc;

use domain::{
    shared::repository::error::RepositoryError,
    user::{
        repository::UserRepository,
        value_objects::{email::Email, id::UserId},
    },
};

use crate::{
    dto::requests::auth::login_user::LoginUserRequest,
    ports::hasher::{HashError, Hasher},
};

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Hash error: {0}")]
    Hash(#[from] HashError),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Internal server error")]
    Internal,
}

pub struct LoginUser<T, H>
where
    T: UserRepository,
    H: Hasher,
{
    user_repository: Arc<T>,
    hasher: Arc<H>,
}

impl<T, H> LoginUser<T, H>
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

    pub async fn execute(&self, request: LoginUserRequest) -> Result<UserId, LoginError> {
        let email = &Email::new(request.email);
        let password_hash = self
            .user_repository
            .get_password_hash_by_email(email)
            .await?;

        if !self
            .hasher
            .verify(&request.password, password_hash.as_str())?
        {
            return Err(LoginError::InvalidCredentials);
        }

        let user_id = self.user_repository.get_user_id_by_email(email).await?;
        Ok(user_id)
    }
}
