#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub created_at: String,
    pub last_posted_at: String,
    pub posts_count: i64,
    pub reply_count: i64,
    pub like_count: i64,
    pub views: i64,
    pub category_id: i64,
    pub highest_post_number: i64,
    pub posts: Vec<Post>,
    pub last_poster_username: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Post {
    pub id: i64,
    pub username: String,
    pub cooked: String,
    pub created_at: String,
    pub updated_at: String,
    pub topic_id: i64,
}
