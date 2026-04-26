#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use domain::{
        shared::repository::error::RepositoryError, user::repository::MockUserRepository,
    };

    use crate::{
        command::auth::{
            request_password_reset::RequestPasswordReset, reset_password::PasswordResetError,
        },
        dto::requests::auth::request_password_reset::RequestPasswordResetDto,
        ports::mailer::{MailerError, MockMailer},
    };

    fn make_request() -> RequestPasswordResetDto {
        RequestPasswordResetDto {
            email: "user@example.com".to_string(),
        }
    }

    #[tokio::test]
    async fn execute_saves_reset_token_and_sends_email() {
        let request = make_request();
        let expected_email_for_save = request.email.clone();
        let expected_email_for_send = request.email.clone();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_save_password_reset_token()
            .withf(move |email, token, expires_at| {
                email.as_str() == expected_email_for_save.as_str()
                    && !token.is_empty()
                    && *expires_at > Utc::now()
            })
            .times(1)
            .returning(|_, _, _| Box::pin(async move { Ok(()) }));

        let mut mailer = MockMailer::new();
        mailer
            .expect_send_password_reset()
            .withf(move |to, token| to == expected_email_for_send.as_str() && !token.is_empty())
            .times(1)
            .returning(|_, _| Box::pin(async move { Ok(()) }));

        let service = RequestPasswordReset::new(Arc::new(user_repository), Arc::new(mailer), 1);

        let result = service.execute(request).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn execute_propagates_repository_error() {
        let request = make_request();
        let expected_email = request.email.clone();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_save_password_reset_token()
            .withf(move |email, token, expires_at| {
                email.as_str() == expected_email.as_str()
                    && !token.is_empty()
                    && *expires_at > Utc::now()
            })
            .times(1)
            .returning(|_, _, _| {
                Box::pin(
                    async move { Err(RepositoryError::ConnectionError("db down".to_string())) },
                )
            });

        let mailer = MockMailer::new();

        let service = RequestPasswordReset::new(Arc::new(user_repository), Arc::new(mailer), 1);

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(PasswordResetError::Repository(RepositoryError::ConnectionError(message)))
                if message == "db down"
        ));
    }

    #[tokio::test]
    async fn execute_propagates_mailer_error() {
        let request = make_request();
        let expected_email_for_save = request.email.clone();
        let expected_email_for_send = request.email.clone();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_save_password_reset_token()
            .withf(move |email, token, expires_at| {
                email.as_str() == expected_email_for_save.as_str()
                    && !token.is_empty()
                    && *expires_at > Utc::now()
            })
            .times(1)
            .returning(|_, _, _| Box::pin(async move { Ok(()) }));

        let mut mailer = MockMailer::new();
        mailer
            .expect_send_password_reset()
            .withf(move |to, token| to == expected_email_for_send.as_str() && !token.is_empty())
            .times(1)
            .returning(|_, _| {
                Box::pin(async move { Err(MailerError::Sending("smtp down".to_string())) })
            });

        let service = RequestPasswordReset::new(Arc::new(user_repository), Arc::new(mailer), 1);

        let result = service.execute(request).await;

        assert!(matches!(
            result,
            Err(PasswordResetError::Mailer(MailerError::Sending(message)))
                if message == "smtp down"
        ));
    }
}
