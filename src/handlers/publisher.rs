use crate::models::error::BotError;
use crate::models::forum::{Post, Topic};

pub async fn print_topic(topic: &Topic) -> Result<String, BotError> {
    println!("Topic: {}", topic.title);
    Ok(topic.title.clone())
}

pub async fn print_post(post: &Post) -> Result<String, BotError> {
    println!("Post: {}", post.id);
    Ok(post.id.to_string())
}
