#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::{
        shared::{repository::error::RepositoryError, value_objects::password_hash::PasswordHash},
        user::{
            repository::MockUserRepository,
            value_objects::{email::Email, id::UserId},
        },
    };
    use uuid::Uuid;

    use crate::{
        dto::requests::auth::login_user::LoginUserRequest,
        ports::hasher::{HashError, MockHasher},
        query::auth::login::{LoginError, LoginUser},
    };

    fn make_request() -> LoginUserRequest {
        LoginUserRequest {
            email: "user@example.com".to_string(),
            password: "secret-password".to_string(),
        }
    }

    fn make_user_id() -> UserId {
        UserId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap())
    }

    #[tokio::test]
    async fn execute_returns_user_id_on_success() {
        let request = make_request();
        let expected_email_for_password = Email::new(request.email.clone());
        let expected_email_for_user_id = Email::new(request.email.clone());
        let expected_password_hash = PasswordHash::new("hashed-password".to_string());
        let expected_user_id = make_user_id();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_password_hash_by_email()
            .withf(move |email| email.as_str() == expected_email_for_password.as_str())
            .times(1)
            .returning({
                let expected_password_hash = expected_password_hash.clone();
                move |_| {
                    let password_hash = expected_password_hash.clone();
                    Box::pin(async move { Ok(password_hash) })
                }
            });

        user_repository
            .expect_get_user_id_by_email()
            .withf(move |email| email.as_str() == expected_email_for_user_id.as_str())
            .times(1)
            .returning({
                let expected_user_id = expected_user_id.clone();
                move |_| {
                    let user_id = expected_user_id.clone();
                    Box::pin(async move { Ok(user_id) })
                }
            });

        let mut hasher = MockHasher::new();
        hasher
            .expect_verify()
            .withf(|value, hash| value == "secret-password" && hash == "hashed-password")
            .times(1)
            .returning(|_, _| Ok(true));

        let service = LoginUser::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(result.is_ok());
        let user_id = result.unwrap();
        assert_eq!(user_id.uuid(), expected_user_id.uuid());
    }

    #[tokio::test]
    async fn execute_returns_invalid_credentials_when_password_does_not_match() {
        let request = make_request();
        let expected_email = Email::new(request.email.clone());
        let password_hash = PasswordHash::new("hashed-password".to_string());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_password_hash_by_email()
            .withf(move |email| email.as_str() == expected_email.as_str())
            .times(1)
            .returning({
                let password_hash = password_hash.clone();
                move |_| {
                    let password_hash = password_hash.clone();
                    Box::pin(async move { Ok(password_hash) })
                }
            });

        user_repository.expect_get_user_id_by_email().times(0);

        let mut hasher = MockHasher::new();
        hasher
            .expect_verify()
            .withf(|value, hash| value == "secret-password" && hash == "hashed-password")
            .times(1)
            .returning(|_, _| Ok(false));

        let service = LoginUser::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(result, Err(LoginError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn execute_propagates_repository_error_from_password_hash_lookup() {
        let request = make_request();
        let expected_email = Email::new(request.email.clone());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_password_hash_by_email()
            .withf(move |email| email.as_str() == expected_email.as_str())
            .times(1)
            .returning(|_| {
                Box::pin(
                    async move { Err(RepositoryError::ConnectionError("db down".to_string())) },
                )
            });

        user_repository.expect_get_user_id_by_email().times(0);

        let hasher = MockHasher::new();

        let service = LoginUser::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(LoginError::Repository(RepositoryError::ConnectionError(message)))
                if message == "db down"
        ));
    }

    #[tokio::test]
    async fn execute_propagates_hash_verification_error() {
        let request = make_request();
        let expected_email = Email::new(request.email.clone());
        let password_hash = PasswordHash::new("hashed-password".to_string());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_password_hash_by_email()
            .withf(move |email| email.as_str() == expected_email.as_str())
            .times(1)
            .returning({
                let password_hash = password_hash.clone();
                move |_| {
                    let password_hash = password_hash.clone();
                    Box::pin(async move { Ok(password_hash) })
                }
            });

        user_repository.expect_get_user_id_by_email().times(0);

        let mut hasher = MockHasher::new();
        hasher
            .expect_verify()
            .withf(|value, hash| value == "secret-password" && hash == "hashed-password")
            .times(1)
            .returning(|_, _| Err(HashError::Verifying("bad hash".to_string())));

        let service = LoginUser::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(LoginError::Hash(HashError::Verifying(message))) if message == "bad hash"
        ));
    }

    #[tokio::test]
    async fn execute_propagates_repository_error_from_user_id_lookup() {
        let request = make_request();
        let expected_email_for_password = Email::new(request.email.clone());
        let expected_email_for_user_id = Email::new(request.email.clone());
        let password_hash = PasswordHash::new("hashed-password".to_string());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_password_hash_by_email()
            .withf(move |email| email.as_str() == expected_email_for_password.as_str())
            .times(1)
            .returning({
                let password_hash = password_hash.clone();
                move |_| {
                    let password_hash = password_hash.clone();
                    Box::pin(async move { Ok(password_hash) })
                }
            });

        user_repository
            .expect_get_user_id_by_email()
            .withf(move |email| email.as_str() == expected_email_for_user_id.as_str())
            .times(1)
            .returning(|_| Box::pin(async move { Err(RepositoryError::NotFound) }));

        let mut hasher = MockHasher::new();
        hasher
            .expect_verify()
            .withf(|value, hash| value == "secret-password" && hash == "hashed-password")
            .times(1)
            .returning(|_, _| Ok(true));

        let service = LoginUser::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(LoginError::Repository(RepositoryError::NotFound))
        ));
    }
}
