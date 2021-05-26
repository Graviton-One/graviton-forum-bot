use std::error::Error;

use teloxide::{prelude::*, types::ParseMode::Html, utils::command::BotCommand, utils::html::link};

mod models;

use crate::models::forum::{Post, ResponsePosts, ResponseTopics, Topic};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "display forum topics.")]
    Topics,
}

pub fn numbered_html_link(json_value: ResponseTopics) -> String {
    let mut draft = String::new();
    json_value
        .topic_list
        .topics
        .iter()
        .enumerate()
        .for_each(|(index, ch)| {
            let Topic {
                id,
                title,
                posts_count,
                reply_count,
                like_count,
                views,
                ..
            } = &ch;
            let url = &format!("https://forum.graviton.one/{}", id.to_string());
            let post_link = link(url, &title);
            let num = index + 1;

            let for_draft = format!(
                "{}. {} \nid: {}, posts: {}, replies: {}, likes: {}, views: {}\n",
                &num, &post_link, &id, &posts_count, &reply_count, &like_count, &views
            );
            draft.push_str(&for_draft);
        });
    draft
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Topics => {
            let end_point = "https://forum.graviton.one/top.json";
            let body_text = reqwest::get(end_point).await.unwrap().text().await.unwrap();

            let html: String = match serde_json::from_str(&body_text) {
                Ok(json_value) => numbered_html_link(json_value),
                Err(_e) => {
                    println!("{:?}", _e);
                    "Something went wrong. There were errors while reading the subreddit."
                        .to_string()
                }
            };
            cx.answer(html).parse_mode(Html).await?
        }
    };

    Ok(())
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting graviton_forum_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "graviton_forum_bot".to_string();

    teloxide::commands_repl(bot, bot_name, answer).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
