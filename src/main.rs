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
use serde::Deserialize;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello))
        .route("/slack", post(slack));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn slack(Json(payload): Json<SlackRequest>) -> impl IntoResponse {
    println!("{:?}", payload);

    match env::var("SLACK_VERIFICATION_TOKEN") {
        Ok(token) => {
            if payload.token != token {
                return (StatusCode::UNAUTHORIZED, String::new());
            }
        }
        Err(message) => {
            println!("SLACK_VERIFICATION_TOKEN could not be used: {}", message);
            return (StatusCode::INTERNAL_SERVER_ERROR, String::new());
        }
    }

    if let Some(r#type) = payload.r#type {
        if r#type == "url_verification" {
            if let Some(challenge) = payload.challenge {
                return (
                    StatusCode::OK,
                    format!(r#"{{"challenge":"{}"}}"#, challenge),
                );
            }
        }
    }

    if let Some(event) = payload.event {
        if event.r#type == "app_mention" {
            anpanman::post_anpanman(event.channel);
        }
    }

    return (StatusCode::OK, String::new());
}

#[derive(Deserialize, Debug)]
struct SlackRequest {
    token: String,
    challenge: Option<String>,
    r#type: Option<String>,
    event: Option<SlackEvent>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct SlackEvent {
    r#type: String,
    user: String,
    text: String,
    ts: String,
    channel: String,
    event_ts: String,
}
