use serde_json::{Value, from_str};
use tokio::time::{Duration, sleep};
use crate::api::posts::{ResponsePosts, Post as ResponsePost};
use crate::api::topics::ResponseTopics;
use crate::handlers::publisher::{print_post, print_topic};
use crate::models::error::BotError;
use crate::models::forum::{Post, Topic};
use crate::URL;

async fn fetch(url: &str) -> Result<String, BotError> {

    let mut body_text: String;
    let forum_api_key: String = std::env::var("FORUM_API_KEY").expect("forum api key get");
    let interval = Duration::from_secs(60);

    loop {
        let body = reqwest::Client::new()
            .get(url)
            .header("Api-Key", &forum_api_key)
            .header("Api-Username", "system")
            .send()
            .await?;
        body_text = body.text().await?;
        // println!("{}", body_text);
        let body_json: Value = from_str(&body_text)?;
        if body_json["error_type"] == "rate_limit" {
            // println!("rate limit, wait for {:#?}", interval);
            sleep(interval).await;
            continue
        }
        return Ok(body_text)
    }
}

pub async fn poll_latest() -> Result<Vec<Topic>, BotError> {
    // println!("poll_latest");

    // println!("fetch new json of topics");
    let url = format!("{}/latest.json", URL); // TODO: add support for pagination "/top?page=1&per_page=50"
    // println!("{}", &url);

    let body_text = fetch(&url).await?;

    // println!("parse into a vector of topics");
    let response: ResponseTopics = from_str(&body_text)?;

    let mut topics: Vec<Topic> = Vec::new();

    // println!("parsing topics");
    for t in response.topic_list.topics {
        // println!("initializing topic {}", t.id);
        let topic = Topic {
            id: t.id,
            title: t.title,
            slug: t.slug,
            created_at: t.created_at,
            last_posted_at: t.last_posted_at,
            posts_count: t.posts_count,
            reply_count: t.reply_count,
            like_count: t.like_count,
            views: t.views,
            category_id: t.category_id,
            highest_post_number: t.highest_post_number,
            posts: Vec::new(),
            last_poster_username: t.last_poster_username,
        };
        topics.push(topic);
    }

    for topic in &mut topics {
        // println!("fetch new json of posts for a topic {}", topic.id);
        let url = format!("{}/t/{}.json", URL, topic.id);
        // println!("{}", &url);

        let body_text = fetch(&url).await?;
        // println!("{},{}", topic.id, body_text);

        let response: ResponsePosts = from_str(&body_text)?;

        for post_id in response.post_stream.stream {
            // println!("fetch post {} for the topic {}", post_id, topic.id);
            let url = format!("{}/posts/{}.json", URL, post_id);
            // println!("{}", &url);

            let body_text = fetch(&url).await?;
            // println!("{},{}", post_id, body_text);

            let p: ResponsePost = from_str(&body_text)?;

            let post = Post {
                id: p.id,
                username: p.username,
                cooked: p.cooked,
                created_at: p.created_at,
                updated_at: p.updated_at,
                topic_id: p.topic_id,
            };
            topic.posts.push(post);
        }
    }

    Ok(topics)
}

// update the list of topics
pub async fn poll_updates(topics_old: &mut Vec<Topic>) -> Result<Option<String>, BotError> {
    let mut messages: Vec<String> = Vec::new();

    let topics_new: Vec<Topic> = poll_latest().await?;

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

    if !topics_diff.is_empty() {
        let topics_message = format!("found {} new topics", topics_diff.len());
        println!("{}", &topics_message);
    }

    // print new posts
    for topic_new in &topics_new {
        match topics_old
            .clone()
            .iter()
            .find(|topic| topic.id == topic_new.id)
        {
            None => continue,
            Some(topic_old) => {
                if topic_new.highest_post_number == topic_old.highest_post_number {
                    continue;
                }
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

                if !posts_diff.is_empty() {
                    let posts_message = format!(
                        "found {} new posts for topic {}",
                        posts_diff.len(),
                        topic_new.id
                    );
                    println!("{}", &posts_message);
                }
            }
        }
    }

    *topics_old = topics_new;
    if messages.is_empty() {
        Ok(None)
    } else {
        Ok(Some(messages.join("\n\n")))
    }
}
