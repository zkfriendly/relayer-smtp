use crate::*;

use anyhow::{anyhow, Result};
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
    pub fn new() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            domain_name: env::var(SMTP_DOMAIN_NAME_KEY)
                .map_err(|e| anyhow!("Failed to get SMTP_DOMAIN_NAME: {}", e))?,
            id: env::var(SMTP_LOGIN_ID_KEY)
                .map_err(|e| anyhow!("Failed to get SMTP_LOGIN_ID: {}", e))?,
            password: env::var(SMTP_LOGIN_PASSWORD_KEY)
                .map_err(|e| anyhow!("Failed to get SMTP_LOGIN_PASSWORD: {}", e))?,
            message_id_domain: env::var(MESSAGE_ID_DOMAIN_KEY)
                .unwrap_or_else(|_| "mail.gmail.com".to_string()),
        })
    }
}

impl ServerConfig {
    pub fn new() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            host: env::var(SERVER_HOST_KEY)
                .map_err(|e| anyhow!("Failed to get SERVER_HOST: {}", e))?,
            port: env::var(SERVER_PORT_KEY)
                .map_err(|e| anyhow!("Failed to get SERVER_PORT: {}", e))?
                .parse()
                .map_err(|e| anyhow!("Failed to parse SERVER_PORT: {}", e))?,
        })
    }
}

impl RelayerSMTPConfig {
    pub fn new() -> Result<Self> {
        Ok(Self {
            smtp_config: SmtpConfig::new()?,
            server_config: ServerConfig::new()?,
        })
    }
}
