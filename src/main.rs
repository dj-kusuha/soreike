use dotenv::dotenv;
use slack::chat::PostMessageRequest;
use slack_api::sync as slack;
use std::env;

fn main() {
    dotenv().unwrap();
    let token = env::var("SLACK_BOT_TOKEN").unwrap();
    let channel = format!("#{}", env::var("SLACK_POST_CHANNEL").unwrap());
    let client = slack::default_client().unwrap();
    let response = slack::chat::post_message(
        &client,
        &token,
        &PostMessageRequest {
            channel: &channel,
            text: ":sore_an::ike::anpanman_pan::anpanman_pan::anpanman_an:", // TODO
            ..PostMessageRequest::default()
        },
    )
    .unwrap();

    println!("{:?}", response.message);
}
