use crate::config::mailer::MailerConfig;
use application::ports::mailer::{Mailer, MailerError};
use async_trait::async_trait;
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::sync::{Arc, Mutex};
use urlencoding::encode;

#[derive(Clone)]
pub struct SmtpMailer {
    inner: Arc<Inner>,
}

struct Inner {
    mailer: Mutex<SmtpTransport>,
    from_mailbox: Mailbox,
    reset_base_url: String,
}

impl SmtpMailer {
    pub fn from_config(cfg: MailerConfig) -> Result<Self, MailerError> {
        let from_mailbox: Mailbox = cfg
            .from
            .parse()
            .map_err(|e| MailerError::Sending(format!("invalid MAILER_FROM: {}", e)))?;

        let transport = SmtpTransport::relay(&cfg.host)
            .map_err(|e| MailerError::Sending(format!("failed to create relay: {}", e)))?
            .credentials(Credentials::new(cfg.username, cfg.password))
            .port(cfg.port)
            .build();

        Ok(Self {
            inner: Arc::new(Inner {
                mailer: Mutex::new(transport),
                from_mailbox,
                reset_base_url: cfg.password_reset_base_url,
            }),
        })
    }
}

#[async_trait]
impl Mailer for SmtpMailer {
    async fn send_password_reset(&self, to: &str, token: &str) -> Result<(), MailerError> {
        let reset_link = format!(
            "{}/auth/recovery/new-password?token={}",
            self.inner.reset_base_url.trim_end_matches('/'),
            encode(token).into_owned()
        );

        let html_body = format!(
            r#"<p>Здравствуйте,</p>
            <p>Вы запросили сброс пароля. Перейдите по ссылке ниже:</p>
            <p><a href="{link}">{link}</a></p>
            <p>Если вы не запрашивали сброс пароля, проигнорируйте это сообщение.</p>"#,
            link = reset_link
        );

        let to_mailbox: Mailbox = to
            .parse()
            .map_err(|e| MailerError::Sending(format!("invalid recipient address: {}", e)))?;

        let email = Message::builder()
            .from(self.inner.from_mailbox.clone())
            .to(to_mailbox)
            .subject("Сброс пароля")
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .map_err(|e| MailerError::Sending(format!("failed to build message: {}", e)))?;

        let mailer = self
            .inner
            .mailer
            .lock()
            .map_err(|e| MailerError::Sending(format!("mutex poisoned: {}", e)))?;

        mailer
            .send(&email)
            .map_err(|e| MailerError::Sending(format!("failed to send email: {}", e)))?;

        Ok(())
    }
}
