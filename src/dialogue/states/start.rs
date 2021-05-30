use crate::dialogue::Dialogue;
use crate::handlers::poller::poll_topics;
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
    let mut forum = Vec::new();
    loop {
        // TODO: initialize with a fresh forum state instead of an empty one to avoid flooding
        match poll_topics(&mut forum).await {
            Ok(html) => {
                cx.answer(html).parse_mode(Html).await?;
                println!("forum topics: {:?}", forum.len());
            }
            Err(e) => println!("{:?}", e),
        }

        sleep(Duration::from_millis(21600)).await;
    }
    exit()
}
