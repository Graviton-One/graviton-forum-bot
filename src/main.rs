use teloxide::prelude::*;

mod api;
mod dialogue;
mod handlers;
mod models;

use crate::dialogue::Dialogue;
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

const URL: &str = "https://forum.graviton.one";
// pub const URL: &str = "https://try.discourse.org";

#[tokio::main]
async fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    run().await;
}

async fn run() {
    // teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::dialogues_repl(bot, |message, dialogue| async move {
        handle_message(message, dialogue)
            .await
            .expect("Something wrong with the bot!")
    })
    .await;
}

async fn handle_message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("Send me a text message.").await?;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}
