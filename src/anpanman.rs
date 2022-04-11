use rand::Rng;
use slack::chat::PostMessageRequest;
use slack_api::sync as slack;
use std::env;

pub fn post_anpanman(channel: String) {
    // 投稿文字列を決定する
    let body = create_body();

    let token;
    match env::var("SLACK_BOT_TOKEN") {
        Ok(t) => token = t,
        Err(message) => {
            println!("SLACK_BOT_TOKEN could not be used: {}", message);
            return;
        }
    }

    let client;
    match slack::default_client() {
        Ok(c) => client = c,
        Err(message) => {
            println!("Failed to get a Slack client: {}", message);
            return;
        }
    }

    match slack::chat::post_message(
        &client,
        &token,
        &PostMessageRequest {
            channel: &channel,
            text: &body,
            ..PostMessageRequest::default()
        },
    ) {
        Ok(response) => println!("{:?}", response.message),
        Err(message) => println!("Failed to post a message to Slack: {}", message),
    }
}

fn create_body() -> String {
    let dat = [
        ":sore_an:",
        ":ike:",
        ":anpanman_an:",
        ":anpanman_pan:",
        ":anpanman_man:",
        ":aa:",
        ":mama:",
        ":an_papa:",
    ];

    let mut rng = rand::thread_rng();
    let mut body = String::from(":sore_an::ike:");

    // 絵文字数の決定 (3～12)
    let length = rng.gen_range(3..=12);
    for _ in 0..length {
        // 絵文字の決定
        let index = rng.gen_range(0..dat.len());
        body.push_str(dat[index]);
        body.push_str(" ");
    }

    return body;
}
