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

    if let Some(r#type) = payload.r#type {
        if r#type == "url_verification"
            && payload.token == env::var("SLACK_VERIFICATION_TOKEN").unwrap()
        {
            if let Some(challenge) = payload.challenge {
                let response = SlackResponse {
                    challenge: challenge,
                };

                return (StatusCode::OK, Json(response));
            }
        }
    }

    if let Some(event) = payload.event {
        if event.r#type == "app_mention" {
            anpanman::post_anpanman(event.channel);
        }
    }

    let response = SlackResponse {
        challenge: "".to_string(),
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize, Debug)]
struct SlackRequest {
    token: String,
    challenge: Option<String>,
    r#type: Option<String>,
    event: Option<SlackEvent>,
}

#[derive(Deserialize, Debug)]
struct SlackEvent {
    r#type: String,
    user: String,
    text: String,
    ts: String,
    channel: String,
    event_ts: String,
}

#[derive(Serialize)]
struct SlackResponse {
    challenge: String,
}
