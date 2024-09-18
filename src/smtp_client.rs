use anyhow::Result;
use lettre::{
    message::{header, Attachment, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::SmtpConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub reference: Option<String>,
    pub reply_to: Option<String>,
    pub body_plain: String,
    pub body_html: String,
    pub body_attachments: Option<Vec<EmailAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub inline_id: String,
    pub content_type: String,
    pub contents: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SmtpClient {
    config: SmtpConfig,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpClient {
    pub fn new(config: SmtpConfig) -> Result<Self> {
        let creds = Credentials::new(config.id.clone(), config.password.clone());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.domain_name)?
            .credentials(creds)
            .build();

        Ok(Self { config, transport })
    }

    pub async fn send_new_email(&self, email: EmailMessage) -> Result<String> {
        self.send_inner(
            email.to,
            email.subject,
            email.reference,
            email.reply_to,
            email.body_plain,
            email.body_html,
            email.body_attachments,
        )
        .await
    }

    async fn send_inner(
        &self,
        to: String,
        subject: String,
        reference: Option<String>,
        reply_to: Option<String>,
        body_plain: String,
        body_html: String,
        body_attachments: Option<Vec<EmailAttachment>>,
    ) -> Result<String> {
        let from_mbox = Mailbox::new(None, self.config.id.parse::<Address>()?);
        let to_mbox = Mailbox::new(None, to.parse::<Address>()?);

        let message_id = format!("<{}@{}>", Uuid::new_v4(), self.config.message_id_domain);
        let mut email_builder = Message::builder()
            .from(from_mbox)
            .subject(subject)
            .header(header::MessageId::from(message_id.clone()))
            .to(to_mbox);
        if let Some(reference) = reference {
            email_builder = email_builder.references(reference);
        }
        if let Some(reply_to) = reply_to {
            email_builder = email_builder.in_reply_to(reply_to);
        }
        let mut multipart = MultiPart::related().singlepart(SinglePart::html(body_html));
        if let Some(body_attachments) = body_attachments {
            for attachment in body_attachments {
                multipart = multipart.singlepart(
                    Attachment::new_inline(attachment.inline_id)
                        .body(attachment.contents, attachment.content_type.parse()?),
                );
            }
        }
        let email = email_builder.multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(body_plain))
                .multipart(multipart),
        )?;

        self.transport.send(email).await?;

        Ok(message_id)
    }
}
