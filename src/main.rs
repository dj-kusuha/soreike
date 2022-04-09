/**
 * Respect for https://zenn.dev/dividebyzero/articles/2815cef7cd446f
 */
mod anpanman;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
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
        .route("/", post(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(Json(payload): Json<SlackRequest>) -> impl IntoResponse {
    if payload.r#type == "url_verification" && payload.token == "Etj9oY0CrhWMjsN4LqR9iBaz" {
        let response = SlackResponse {
            challenge: payload.challenge,
        };

        return (StatusCode::OK, Json(response));
    }

    if payload.event.r#type == "app_mention" {
        anpanman::post_anpanman(payload.event.channel);
    }

    let response = SlackResponse {
        challenge: "".to_string(),
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
struct SlackRequest {
    token: String,
    challenge: String,
    r#type: String,
    event: SlackEvent,
}

#[derive(Deserialize)]
struct SlackEvent {
    r#type: String,
    channel: String,
}

#[derive(Serialize)]
struct SlackResponse {
    challenge: String,
}
