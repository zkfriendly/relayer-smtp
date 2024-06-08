use crate::{error, info};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use relayer_utils::LOG;
use serde_json::json;
use tokio::task;

use crate::{EmailMessage, SERVER_CONFIG, SMTP_CLIENT};

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn send_email(event: web::Json<EmailMessage>) -> impl Responder {
    let event_data = event.into_inner();
    info!(LOG, "Received email to send: {:?}", event_data);
    task::spawn(async move {
        if let Err(e) = SMTP_CLIENT.get().unwrap().send_new_email(event_data).await {
            error!(LOG, "Failed to send email: {}", e);
        }
    });
    HttpResponse::Ok().json(json!({"status": "success"}))
}

pub async fn run_server() -> Result<(), actix_web::Error> {
    info!(
        LOG,
        "Starting server at {}:{}",
        SERVER_CONFIG.get().unwrap().host,
        SERVER_CONFIG.get().unwrap().port
    );
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/sendEmail", web::post().to(send_email))
    })
    .bind(format!(
        "{}:{}",
        SERVER_CONFIG.get().unwrap().host,
        SERVER_CONFIG.get().unwrap().port
    ))?
    .run()
    .await?;

    Ok(())
}
