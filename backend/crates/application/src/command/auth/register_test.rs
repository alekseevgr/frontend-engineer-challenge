#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::{
        shared::repository::error::RepositoryError, user::repository::MockUserRepository,
    };
    use uuid::Uuid;

    use crate::{
        command::auth::register::{RegisterUser, RegistrationError},
        dto::requests::auth::register_user::RegisterUserRequest,
        ports::hasher::{HashError, MockHasher},
    };

    fn make_request() -> RegisterUserRequest {
        RegisterUserRequest::new("user@example.com".to_string(), "plain-password".to_string())
    }

    #[tokio::test]
    async fn execute_registers_user_successfully() {
        let request = make_request();
        let expected_email = request.email.clone();
        let expected_password = request.password.clone();
        let expected_hash = "hashed-password".to_string();

        let mut hasher = MockHasher::new();
        hasher
            .expect_hash()
            .withf(move |value| value == expected_password.as_str())
            .times(1)
            .returning(move |_| Ok(expected_hash.clone()));

        let mut user_repository = MockUserRepository::new();
        user_repository.expect_create_user().times(1).returning({
            let expected_email = expected_email.clone();
            move |user| {
                assert_eq!(user.email().as_str(), expected_email.as_str());
                assert_eq!(user.password_hash().as_str(), "hashed-password");
                assert_ne!(user.id().uuid(), Uuid::nil());
                Box::pin(async move { Ok(()) })
            }
        });

        let service = RegisterUser::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email().as_str(), "user@example.com");
        assert_eq!(user.password_hash().as_str(), "hashed-password");
        assert_ne!(user.id().uuid(), Uuid::nil());
    }

    #[tokio::test]
    async fn execute_propagates_hash_error() {
        let request = make_request();

        let mut hasher = MockHasher::new();
        hasher
            .expect_hash()
            .withf(|value| value == "plain-password")
            .times(1)
            .returning(|_| Err(HashError::Creation("hashing failed".to_string())));

        let mut user_repository = MockUserRepository::new();
        user_repository.expect_create_user().times(0);

        let service = RegisterUser::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(RegistrationError::Hash(HashError::Creation(message)))
                if message == "hashing failed"
        ));
    }

    #[tokio::test]
    async fn execute_propagates_repository_error() {
        let request = make_request();
        let expected_email = request.email.clone();
        let expected_password = request.password.clone();
        let expected_hash = "hashed-password".to_string();

        let mut hasher = MockHasher::new();
        hasher
            .expect_hash()
            .withf(move |value| value == expected_password.as_str())
            .times(1)
            .returning(move |_| Ok(expected_hash.clone()));

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_create_user()
            .times(1)
            .returning(move |user| {
                assert_eq!(user.email().as_str(), expected_email.as_str());
                assert_eq!(user.password_hash().as_str(), "hashed-password");
                Box::pin(async move { Err(RepositoryError::AlreadyExists) })
            });

        let service = RegisterUser::new(Arc::new(user_repository), Arc::new(hasher));

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(RegistrationError::Repository(
                RepositoryError::AlreadyExists
            ))
        ));
    }
}
