use teloxide::{prelude::*, types::ParseMode::Html};
use tokio::time::{sleep, Duration};

use crate::dialogue::Dialogue;
use crate::utils::numbered_html_link;

#[derive(Clone)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(
    _state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    loop {
        let end_point = "https://forum.graviton.one/top.json";
        let body_text = reqwest::get(end_point).await.unwrap().text().await.unwrap();

        let html: String = match serde_json::from_str(&body_text) {
            Ok(json_value) => numbered_html_link(json_value),
            Err(_e) => {
                println!("{:?}", _e);
                "Something went wrong. There were errors while reading the subreddit.".to_string()
            }
        };
        cx.answer(html).parse_mode(Html).await?;
        sleep(Duration::from_millis(60000)).await;
    }
    exit()
}
