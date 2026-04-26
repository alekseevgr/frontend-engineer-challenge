use async_trait::async_trait;

use chrono::{DateTime, Utc};
use domain::{
    shared::{repository::error::RepositoryError, value_objects::password_hash::PasswordHash},
    user::{
        User,
        repository::UserRepository,
        value_objects::{email::Email, id::UserId},
    },
};
use sqlx::{Pool, Postgres};

use crate::database::postgres::error::map_sqlx_error_to_domain_error;

pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
                INSERT INTO users (id, email, password_hash, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id().uuid(),
            user.email().as_str(),
            user.password_hash().as_str(),
            user.created_at(),
            user.updated_at()
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        Ok(())
    }

    async fn get_password_hash_by_email(
        &self,
        email: &Email,
    ) -> Result<PasswordHash, RepositoryError> {
        let record = sqlx::query!(
            r#"
                SELECT password_hash
                FROM users
                WHERE email = $1
            "#,
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        match record {
            Some(row) => Ok(PasswordHash::new(row.password_hash)),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn get_user_id_by_email(&self, email: &Email) -> Result<UserId, RepositoryError> {
        let record = sqlx::query!(
            r#"
                SELECT id
                FROM users
                WHERE email = $1
            "#,
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        match record {
            Some(row) => Ok(UserId::new(row.id)),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn save_password_reset_token(
        &self,
        email: &Email,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), RepositoryError> {
        let user_record = sqlx::query!(
            r#"
                SELECT id
                FROM users
                WHERE email = $1
            "#,
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        let user_id = match user_record {
            Some(r) => r.id,
            None => return Err(RepositoryError::NotFound),
        };

        sqlx::query!(
            r#"
                INSERT INTO password_reset_tokens (token, user_id, expires_at, used, created_at)
                VALUES ($1, $2, $3, false, now())
                ON CONFLICT (token)
                DO UPDATE SET user_id = EXCLUDED.user_id, expires_at = EXCLUDED.expires_at, used = false, created_at = now()
            "#,
            token,
            user_id,
            expires_at
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        Ok(())
    }

    async fn get_user_id_and_expiry_by_password_reset_token(
        &self,
        token: &str,
    ) -> Result<(UserId, DateTime<Utc>), RepositoryError> {
        let record = sqlx::query!(
            r#"
                SELECT user_id, expires_at, used
                FROM password_reset_tokens
                WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        match record {
            Some(row) => {
                if row.used {
                    return Err(RepositoryError::NotFound);
                }
                let user_id = UserId::new(row.user_id);
                let expires_at: DateTime<Utc> = row.expires_at;
                Ok((user_id, expires_at))
            }
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn invalidate_password_reset_token(&self, token: &str) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
                UPDATE password_reset_tokens
                SET used = true
                WHERE token = $1
            "#,
            token
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        Ok(())
    }

    async fn update_password(
        &self,
        user_id: &UserId,
        password_hash: &PasswordHash,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
                UPDATE users
                SET password_hash = $1, updated_at = now()
                WHERE id = $2
            "#,
            password_hash.as_str(),
            user_id.uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        Ok(())
    }
}
