pub mod config;
pub mod server;
pub mod smtp_client;
pub mod strings;

use config::*;
use server::*;
use smtp_client::*;
use strings::*;

use anyhow::Result;
use slog::{error, info};
use std::{
    error::Error,
    fmt,
    sync::{Arc, Mutex, OnceLock},
};

#[derive(Debug)]
struct ActixErrorWrapper(String);

impl fmt::Display for ActixErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ActixErrorWrapper {}

impl From<actix_web::Error> for ActixErrorWrapper {
    fn from(error: actix_web::Error) -> Self {
        ActixErrorWrapper(error.to_string())
    }
}

pub static SMTP_CONFIG: OnceLock<SmtpConfig> = OnceLock::new();
pub static SERVER_CONFIG: OnceLock<ServerConfig> = OnceLock::new();
pub static SMTP_CLIENT: OnceLock<Arc<Mutex<SmtpClient>>> = OnceLock::new();

pub async fn run(config: RelayerSMTPConfig) -> Result<()> {
    SMTP_CONFIG.set(config.smtp_config.clone()).unwrap();
    SERVER_CONFIG.set(config.server_config.clone()).unwrap();
    SMTP_CLIENT
        .set(Arc::new(Mutex::new(SmtpClient::new(config.smtp_config)?)))
        .unwrap();

    run_server().await.map_err(ActixErrorWrapper::from)?;
    Ok(())
}
