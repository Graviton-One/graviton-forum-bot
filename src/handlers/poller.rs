use crate::handlers::parser::{parse_posts, parse_topics};
use crate::handlers::publisher::{print_post, print_topic};
use crate::models::error::BotError;
use crate::models::forum::{Post, Topic};

// const URL: &str = "https://forum.graviton.one";
// const URL: &str = "https://discourse.nixos.org";
const URL: &str = "https://try.discourse.org";

pub async fn poll_forum() -> String {
    "".to_string()
}

// update the list of topics
pub async fn poll_topics(topics_old: &mut Vec<Topic>) -> Result<String, BotError> {
    let mut messages: Vec<String> = vec![String::from("New updates")];

    // fetch new json of topics
    let end_point = format!("{}/latest.json", URL); // TODO: add support for pagination "/top?page=1&per_page=50"
    let body = reqwest::get(end_point).await?;
    let body_text = body.text().await?;

    // parse into a vector of topics
    let mut topics_new: Vec<Topic> = parse_topics(body_text)?;

    // populate each topic with a vector of posts
    for topic in &mut topics_new {
        poll_posts(topic).await?;
    }

    // find topics with new ids
    let topics_diff: Vec<Topic> = topics_new
        .clone()
        .into_iter()
        .filter(|topic_new| {
            !topics_old
                .iter()
                .any(|topic_old| topic_new.id == topic_old.id)
        })
        .collect();

    // print each new topic and its posts
    for topic in &topics_diff {
        let topic_message = print_topic(topic).await?;
        messages.push(topic_message);
        for post in &topic.posts {
            let post_message = print_post(post).await?;
            messages.push(post_message);
        }
    }
    let topics_message = format!("found {} new topics", topics_diff.len());
    println!("{}", &topics_message);
    messages.push(topics_message);

    // print new posts
    for topic_new in &topics_new {
        match topics_old
            .clone()
            .iter()
            .find(|topic| topic.id == topic_new.id)
        {
            None => continue,
            Some(topic_old) => {
                let posts_new = &topic_new.posts;
                let posts_old = &topic_old.posts;

                // for each topic find posts with new ids
                let posts_diff: Vec<Post> = posts_new
                    .clone()
                    .into_iter()
                    .filter(|post_new| !posts_old.iter().any(|post_old| post_new.id == post_old.id))
                    .collect();
                // print each new post
                for post in &posts_diff {
                    let post_message = print_post(post).await?;
                    messages.push(post_message);
                }

                if posts_diff.len() > 0 {
                    let posts_message = format!(
                        "found {} new posts for topic {}",
                        posts_diff.len(),
                        topic_new.id
                    );
                    println!("{}", &posts_message);
                    messages.push(posts_message);
                }
            }
        }
    }

    *topics_old = topics_new;
    Ok(messages.join("\n"))
}

// update a topic with a list of posts
pub async fn poll_posts(topic: &mut Topic) -> Result<(), BotError> {
    // fetch new json of posts for a topic
    let end_point = format!("{}/t/{}.json", URL, topic.id);
    let body = reqwest::get(end_point).await?;
    let body_text = body.text().await?;

    // parse into a vector of posts
    topic.posts = parse_posts(body_text)?;
    // println!("parsed posts for topic {}", topic.id);

    Ok(())
}
