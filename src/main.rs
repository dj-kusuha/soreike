/**
 * Respect for https://zenn.dev/dividebyzero/articles/2815cef7cd446f
 */
mod anpanman;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/verify", post(verify_slack_bot));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    anpanman::post_anpanman();

    "Hello, World!"
}

async fn verify_slack_bot(Json(payload): Json<VerificationRequest>) -> impl IntoResponse {
    let response = VeificationResponse {
        challenge: "".to_string(),
    };

    if payload.token != "Etj9oY0CrhWMjsN4LqR9iBaz" {
        tracing::error!("invalid token");
        return (StatusCode::UNAUTHORIZED, Json(response));
    }

    if payload.r#type != "url_verification" {
        tracing::error!("invalid type: {}", payload.r#type);
        return (StatusCode::UNAUTHORIZED, Json(response));
    }

    let response = VeificationResponse {
        challenge: payload.challenge,
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
struct VerificationRequest {
    token: String,
    challenge: String,
    r#type: String,
}

#[derive(Serialize)]
struct VeificationResponse {
    challenge: String,
}
