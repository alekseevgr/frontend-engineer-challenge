#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{Duration, Utc};
    use domain::{
        shared::repository::error::RepositoryError,
        user::{repository::MockUserRepository, value_objects::id::UserId},
    };
    use uuid::Uuid;

    use crate::{
        command::auth::reset_password::{PasswordResetError, ResetPassword},
        dto::requests::auth::reset_password::ResetPasswordDto,
        ports::hasher::{HashError, MockHasher},
    };

    fn make_request() -> ResetPasswordDto {
        ResetPasswordDto {
            token: "reset-token".to_string(),
            new_password: "new-password".to_string(),
        }
    }

    fn make_user_id() -> UserId {
        UserId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap())
    }

    #[tokio::test]
    async fn execute_resets_password_successfully() {
        let request = make_request();
        let expected_token_for_lookup = request.token.clone();
        let expected_token_for_invalidation = request.token.clone();
        let expected_new_password = request.new_password.clone();
        let expected_user_id = make_user_id();
        let expires_at = Utc::now() + Duration::hours(1);

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_user_id_and_expiry_by_password_reset_token()
            .withf(move |token| token == &expected_token_for_lookup)
            .times(1)
            .returning({
                let expected_user_id = expected_user_id.clone();
                move |_| {
                    let user_id = expected_user_id.clone();
                    Box::pin(async move { Ok((user_id, expires_at)) })
                }
            });

        user_repository
            .expect_update_password()
            .times(1)
            .returning({
                let expected_user_id = expected_user_id.clone();
                move |user_id, password_hash| {
                    assert_eq!(user_id.uuid(), expected_user_id.uuid());
                    assert_eq!(password_hash.as_str(), "hashed-new-password");
                    Box::pin(async move { Ok(()) })
                }
            });

        user_repository
            .expect_invalidate_password_reset_token()
            .withf(move |token| token == &expected_token_for_invalidation)
            .times(1)
            .returning(|_| Box::pin(async move { Ok(()) }));

        let mut hasher = MockHasher::new();
        hasher
            .expect_hash()
            .withf(move |value| value == expected_new_password.as_str())
            .times(1)
            .returning(|_| Ok("hashed-new-password".to_string()));

        let service = ResetPassword::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(result.is_ok());
        let user_id = result.unwrap();
        assert_eq!(user_id.uuid(), expected_user_id.uuid());
    }

    #[tokio::test]
    async fn execute_returns_invalid_or_expired_token_when_token_is_expired() {
        let request = make_request();
        let expected_token = request.token.clone();
        let user_id = make_user_id();
        let expires_at = Utc::now() - Duration::hours(1);

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_user_id_and_expiry_by_password_reset_token()
            .withf(move |token| token == &expected_token)
            .times(1)
            .returning({
                let user_id = user_id.clone();
                move |_| {
                    let user_id = user_id.clone();
                    Box::pin(async move { Ok((user_id, expires_at)) })
                }
            });

        user_repository.expect_update_password().times(0);
        user_repository
            .expect_invalidate_password_reset_token()
            .times(0);

        let hasher = MockHasher::new();

        let service = ResetPassword::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(PasswordResetError::InvalidOrExpiredToken)
        ));
    }

    #[tokio::test]
    async fn execute_propagates_repository_error_from_token_lookup() {
        let request = make_request();
        let expected_token = request.token.clone();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_user_id_and_expiry_by_password_reset_token()
            .withf(move |token| token == &expected_token)
            .times(1)
            .returning(|_| Box::pin(async move { Err(RepositoryError::NotFound) }));

        user_repository.expect_update_password().times(0);
        user_repository
            .expect_invalidate_password_reset_token()
            .times(0);

        let hasher = MockHasher::new();

        let service = ResetPassword::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(PasswordResetError::Repository(RepositoryError::NotFound))
        ));
    }

    #[tokio::test]
    async fn execute_propagates_hash_error() {
        let request = make_request();
        let expected_token = request.token.clone();
        let expected_new_password = request.new_password.clone();
        let user_id = make_user_id();
        let expires_at = Utc::now() + Duration::hours(1);

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_user_id_and_expiry_by_password_reset_token()
            .withf(move |token| token == &expected_token)
            .times(1)
            .returning({
                let user_id = user_id.clone();
                move |_| {
                    let user_id = user_id.clone();
                    Box::pin(async move { Ok((user_id, expires_at)) })
                }
            });

        user_repository.expect_update_password().times(0);
        user_repository
            .expect_invalidate_password_reset_token()
            .times(0);

        let mut hasher = MockHasher::new();
        hasher
            .expect_hash()
            .withf(move |value| value == expected_new_password.as_str())
            .times(1)
            .returning(|_| Err(HashError::Creation("hash failed".to_string())));

        let service = ResetPassword::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(PasswordResetError::Hash(HashError::Creation(message)))
                if message == "hash failed"
        ));
    }

    #[tokio::test]
    async fn execute_propagates_repository_error_from_update_password() {
        let request = make_request();
        let expected_token_for_lookup = request.token.clone();
        let expected_new_password = request.new_password.clone();
        let user_id = make_user_id();
        let expires_at = Utc::now() + Duration::hours(1);

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_user_id_and_expiry_by_password_reset_token()
            .withf(move |token| token == &expected_token_for_lookup)
            .times(1)
            .returning({
                let user_id = user_id.clone();
                move |_| {
                    let user_id = user_id.clone();
                    Box::pin(async move { Ok((user_id, expires_at)) })
                }
            });

        user_repository
            .expect_update_password()
            .times(1)
            .returning({
                let expected_user_id = user_id.clone();
                move |user_id, password_hash| {
                    assert_eq!(user_id.uuid(), expected_user_id.uuid());
                    assert_eq!(password_hash.as_str(), "hashed-new-password");
                    Box::pin(
                        async move { Err(RepositoryError::Internal("update failed".to_string())) },
                    )
                }
            });

        user_repository
            .expect_invalidate_password_reset_token()
            .times(0);

        let mut hasher = MockHasher::new();
        hasher
            .expect_hash()
            .withf(move |value| value == expected_new_password.as_str())
            .times(1)
            .returning(|_| Ok("hashed-new-password".to_string()));

        let service = ResetPassword::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(PasswordResetError::Repository(RepositoryError::Internal(message)))
                if message == "update failed"
        ));
    }

    #[tokio::test]
    async fn execute_propagates_repository_error_from_token_invalidation() {
        let request = make_request();
        let expected_token_for_lookup = request.token.clone();
        let expected_token_for_invalidation = request.token.clone();
        let expected_new_password = request.new_password.clone();
        let user_id = make_user_id();
        let expires_at = Utc::now() + Duration::hours(1);

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_user_id_and_expiry_by_password_reset_token()
            .withf(move |token| token == &expected_token_for_lookup)
            .times(1)
            .returning({
                let user_id = user_id.clone();
                move |_| {
                    let user_id = user_id.clone();
                    Box::pin(async move { Ok((user_id, expires_at)) })
                }
            });

        user_repository
            .expect_update_password()
            .times(1)
            .returning({
                let expected_user_id = user_id.clone();
                move |user_id, password_hash| {
                    assert_eq!(user_id.uuid(), expected_user_id.uuid());
                    assert_eq!(password_hash.as_str(), "hashed-new-password");
                    Box::pin(async move { Ok(()) })
                }
            });

        user_repository
            .expect_invalidate_password_reset_token()
            .withf(move |token| token == &expected_token_for_invalidation)
            .times(1)
            .returning(|_| {
                Box::pin(
                    async move { Err(RepositoryError::Internal("invalidate failed".to_string())) },
                )
            });

        let mut hasher = MockHasher::new();
        hasher
            .expect_hash()
            .withf(move |value| value == expected_new_password.as_str())
            .times(1)
            .returning(|_| Ok("hashed-new-password".to_string()));

        let service = ResetPassword::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(PasswordResetError::Repository(RepositoryError::Internal(message)))
                if message == "invalidate failed"
        ));
    }
}
