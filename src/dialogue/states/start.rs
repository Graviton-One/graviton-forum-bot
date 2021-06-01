use crate::dialogue::Dialogue;
use crate::handlers::poller::{poll_topics, poll_updates};
use log::info;
use teloxide::{prelude::*, types::ParseMode::Html};
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(
    _state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    // try to initialize forum before listening to updates to avoid flood on start
    info!("Initializing forum state...");
    let interval = Duration::from_secs(60); // polling interval in seconds
    let mut forum = match poll_topics().await {
        Ok(vec) => vec,
        Err(_) => Vec::new(),
    };
    loop {
        info!("Polling updates...");
        match poll_updates(&mut forum).await {
            Ok(msg) => {
                match msg {
                    Some(html) => {
                        cx.answer(html).parse_mode(Html).await?;
                        info!("Published updates, next in {:?}...", interval);
                        sleep(interval).await;
                    }
                    None => {
                        info!("No updates, next in {:?}...", interval);
                        sleep(interval).await;
                    }
                };
            }
            Err(e) => println!("{:?}", e),
        }
    }
    exit()
}
