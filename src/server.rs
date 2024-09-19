use crate::{error, info};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use relayer_utils::LOG;
use serde_json::json;

use crate::{EmailMessage, SERVER_CONFIG, SMTP_CLIENT};

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn send_email(event: web::Json<EmailMessage>) -> impl Responder {
    let event_data = event.into_inner();
    info!(LOG, "Received email to send: {:?}", event_data);

    let smtp_client = SMTP_CLIENT.get().unwrap().lock().unwrap();
    match smtp_client.send_new_email(event_data).await {
        Ok(message_id) => {
            HttpResponse::Ok().json(json!({"status": "success", "message_id": message_id}))
        }
        Err(e) => {
            error!(LOG, "Failed to send email: {}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error",
                "message": format!("Failed to send email: {}", e)
            }))
        }
    }
}

pub async fn run_server() -> Result<(), actix_web::Error> {
    let server_config = SERVER_CONFIG.get().unwrap();
    info!(
        LOG,
        "Starting server at {}:{}",
        server_config.host,
        server_config.port
    );
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/ping", web::get().to(ping))
                .route("/sendEmail", web::post().to(send_email)),
        )
    })
    .bind(format!(
        "{}:{}",
        server_config.host,
        server_config.port
    ))?
    .run()
    .await?;

    Ok(())
}
