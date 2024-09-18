use crate::*;

use std::env;

use dotenv::dotenv;

#[derive(Clone, Debug)]
pub struct SmtpConfig {
    pub domain_name: String,
    pub id: String,
    pub password: String,
    pub message_id_domain: String,
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug)]
pub struct RelayerSMTPConfig {
    pub smtp_config: SmtpConfig,
    pub server_config: ServerConfig,
}

impl SmtpConfig {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            domain_name: env::var(SMTP_DOMAIN_NAME_KEY).unwrap(),
            id: env::var(SMTP_LOGIN_ID_KEY).unwrap(),
            password: env::var(SMTP_LOGIN_PASSWORD_KEY).unwrap(),
            message_id_domain: env::var(MESSAGE_ID_DOMAIN_KEY)
                .unwrap_or_else(|_| "mail.gmail.com".to_string()),
        }
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            host: env::var(SERVER_HOST_KEY).unwrap(),
            port: env::var(SERVER_PORT_KEY).unwrap().parse().unwrap(),
        }
    }
}

impl RelayerSMTPConfig {
    pub fn new() -> Self {
        Self {
            smtp_config: SmtpConfig::new(),
            server_config: ServerConfig::new(),
        }
    }
}
