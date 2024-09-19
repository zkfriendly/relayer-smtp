# Relayer SMTP

Relayer SMTP is a simple email relay server built with Rust, Actix Web, and Tokio. It provides an HTTP API for sending emails through a configured SMTP server.

## Features

- HTTP API for sending emails
- Configurable SMTP and server settings via environment variables
- Asynchronous email sending using Tokio
- Simple health check endpoint

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/config.rs`: Configuration structures and loading
- `src/server.rs`: HTTP server implementation
- `src/smtp_client.rs`: SMTP client implementation
- `src/lib.rs`: Library functions and module declarations
- `src/strings.rs`: Constant strings used in the project

## Setup

1. Clone the repository
2. Copy `.env.example` to `.env` and fill in the required environment variables:

```
SMTP_DOMAIN_NAME=your_smtp_domain
SMTP_LOGIN_ID=your_smtp_login
SMTP_LOGIN_PASSWORD=your_smtp_password
MESSAGE_ID_DOMAIN=your_message_id_domain
SERVER_HOST=localhost
SERVER_PORT=8080
```

3. Run `cargo build` to compile the project

## Usage

1. Start the server:

```
cargo run
```

2. The server will start on the configured host and port (default: `localhost:8080`)

## API Endpoints

### GET /api/ping

Health check endpoint.

**Response:**

```
Hello, world!
```

### POST /api/sendEmail

Send an email using the SMTP relay service.

**Sample Body:**

```bash
curl -X POST http://localhost:3000/api/sendEmail \
  -H "Content-Type: application/json" \
  -d '{
    "to": "recipient@example.com",
    "subject": "Test Email",
    "body_plain": "This is a test email.",
    "body_html": "<html><body><p>This is a test email.</p></body></html>",
    "reference": null,
    "reply_to": null,
    "body_attachments": null
  }'

```

**Success Response:**

```json
{
  "message_id": "<c4a4bb20-2948-4bfa-9108-62531f9d370f@mail.gmail.com>",
  "status": "success"
}
```
