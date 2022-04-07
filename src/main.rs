use dotenv::dotenv;
use slack::chat::PostMessageRequest;
use slack_api::sync as slack;
use std::env;

fn main() {
    dotenv().unwrap();
    let token = env::var("SLACK_BOT_TOKEN").unwrap();
    let client = slack::default_client().unwrap();
    let response = slack::chat::post_message(
        &client,
        &token,
        &PostMessageRequest {
            channel: "#総合雑談窓",
            text: ":sore_an::ike::anpanman_an::anpanman_an::anpanman_an:", // TODO
            ..PostMessageRequest::default()
        },
    )
    .unwrap();

    println!("{:?}", response.message);
}
