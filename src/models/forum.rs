use serde::Deserialize;

#[derive(Deserialize)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub fancy_title: String,
    pub slug: String,
    pub posts_count: i32,
    pub reply_count: i32,
    pub highest_post_number: i32,
    pub created_at: String,
    pub last_posted_at: String,
    pub like_count: i32,
    pub views: i32,
    pub category_id: i32,
}

#[derive(Deserialize)]
pub struct Topics {
    pub topics: Vec<Topic>,
}

#[derive(Deserialize)]
pub struct ResponseTopics {
    pub topic_list: Topics,
}

#[derive(Deserialize)]
pub struct Post {
    pub id: String,
    pub username: String,
    pub created_at: String,
    pub cooked: String,
    pub post_number: String,
    pub post_type: String,
    pub updated_at: String,
    pub reply_count: String,
    pub reply_to_post_number: String,
    pub quote_count: String,
    pub incoming_link_count: String,
    pub reads: String,
    pub readers_count: String,
    pub score: String,
    pub topic_id: String,
    pub topic_slug: String,
    pub user_id: String,
    pub edit_reason: String,
}

#[derive(Deserialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}

#[derive(Deserialize)]
pub struct ResponsePosts {
    pub post_stream: Posts,
}
