use crate::api::posts::ResponsePosts;
use crate::api::topics::ResponseTopics;
use crate::models::error::BotError;
use crate::models::forum::{Post, Topic};

pub fn parse_topics(json: String) -> Result<Vec<Topic>, BotError> {
    // println!("deserializing topics");
    let response: ResponseTopics = serde_json::from_str(&json)?;

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
    // println!("parsed {} topics", topics.len());
    Ok(topics)
}

pub fn parse_posts(json: String) -> Result<Vec<Post>, BotError> {
    // println!("deserializing posts");
    let response: ResponsePosts = serde_json::from_str(&json)?;

    let mut posts: Vec<Post> = Vec::new();

    // println!("parsing posts");
    for p in response.post_stream.posts {
        // println!("parsing post {}", p.id);
        let post = Post {
            id: p.id,
            username: p.username,
            cooked: p.cooked,
            created_at: p.created_at,
            updated_at: p.updated_at,
            topic_id: p.topic_id,
        };
        posts.push(post);
    }
    // println!("parsed {} posts", posts.len());
    Ok(posts)
}
