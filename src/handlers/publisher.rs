use crate::models::error::BotError;
use crate::models::forum::{Post, Topic};
use crate::URL;
use html2text::from_read;
use substring::Substring;
use teloxide::utils::html::link;

pub async fn print_topic(topic: &Topic) -> Result<String, BotError> {
    let url = &format!("{}/{}", URL, topic.id.to_string());
    let topic_link = link(url, &topic.title);
    println!("Topic: {}", topic.title);
    Ok(format!(
        "{} created new topic \"{}\"",
        topic.last_poster_username, topic_link
    ))
}

pub async fn print_post(post: &Post) -> Result<String, BotError> {
    let url = &format!(
        "{}/t/{}/{}",
        URL,
        post.topic_id.to_string(),
        post.id.to_string()
    );
    let post_link = link(url, "posted");
    let preview_length = 140;
    let post_body = from_read(post.cooked.as_bytes(), preview_length);
    let post_preview = format!(
        "{}{}",
        post_body.substring(0, preview_length),
        if post_body.len() > preview_length {
            "..."
        } else {
            ""
        }
    );
    println!("Post: {}", &post_preview);
    Ok(format!("{} {}: {}", post.username, post_link, post_preview))
}
