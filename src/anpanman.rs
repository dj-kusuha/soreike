use rand::Rng;
use slack::chat::PostMessageRequest;
use slack_api::sync as slack;
use std::env;

pub fn post_anpanman(channel: String) {
    // 投稿文字列を決定する
    let body = create_body();

    let token = env::var("SLACK_BOT_TOKEN").unwrap();
    let client = slack::default_client().unwrap();
    let response = slack::chat::post_message(
        &client,
        &token,
        &PostMessageRequest {
            channel: &channel,
            text: &body,
            ..PostMessageRequest::default()
        },
    )
    .unwrap();

    println!("{:?}", response.message);
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
    }

    return body;
}
