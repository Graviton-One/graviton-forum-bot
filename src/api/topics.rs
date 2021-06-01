// https://transform.tools/json-to-rust-serde
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTopics {
    pub users: Vec<User>,
    #[serde(rename = "primary_groups")]
    pub primary_groups: Vec<::serde_json::Value>,
    #[serde(rename = "topic_list")]
    pub topic_list: TopicList,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "avatar_template")]
    pub avatar_template: Option<String>,
    pub admin: Option<bool>,
    #[serde(rename = "trust_level")]
    pub trust_level: i64,
    pub moderator: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicList {
    #[serde(rename = "can_create_topic")]
    pub can_create_topic: Option<bool>,
    #[serde(rename = "for_period")]
    pub for_period: Option<String>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    pub topics: Vec<Topic>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub id: i64,
    pub title: String,
    #[serde(rename = "fancy_title")]
    pub fancy_title: Option<String>,
    pub slug: String,
    #[serde(rename = "posts_count")]
    pub posts_count: i64,
    #[serde(rename = "reply_count")]
    pub reply_count: i64,
    #[serde(rename = "highest_post_number")]
    pub highest_post_number: i64,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "last_posted_at")]
    pub last_posted_at: String,
    pub bumped: Option<bool>,
    #[serde(rename = "bumped_at")]
    pub bumped_at: Option<String>,
    pub archetype: Option<String>,
    pub unseen: Option<bool>,
    #[serde(rename = "last_read_post_number")]
    pub last_read_post_number: Option<i64>,
    pub unread: Option<i64>,
    #[serde(rename = "new_posts")]
    pub new_posts: Option<i64>,
    pub pinned: Option<bool>,
    pub unpinned: Option<bool>,
    pub visible: Option<bool>,
    pub closed: Option<bool>,
    pub archived: Option<bool>,
    #[serde(rename = "notification_level")]
    pub notification_level: Option<i64>,
    pub bookmarked: Option<bool>,
    pub liked: Option<bool>,
    pub views: i64,
    #[serde(rename = "like_count")]
    pub like_count: i64,
    #[serde(rename = "has_summary")]
    pub has_summary: Option<bool>,
    #[serde(rename = "last_poster_username")]
    pub last_poster_username: String,
    #[serde(rename = "category_id")]
    pub category_id: i64,
    #[serde(rename = "pinned_globally")]
    pub pinned_globally: Option<bool>,
    #[serde(rename = "featured_link")]
    pub featured_link: ::serde_json::Value,
    pub posters: Vec<Poster>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Poster {
    pub extras: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    #[serde(rename = "primary_group_id")]
    pub primary_group_id: ::serde_json::Value,
}
