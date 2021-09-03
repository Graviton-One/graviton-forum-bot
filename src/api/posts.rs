// https://transform.tools/json-to-rust-serde
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePosts {
    #[serde(rename = "post_stream")]
    pub post_stream: PostStream,
    #[serde(rename = "timeline_lookup")]
    pub timeline_lookup: Vec<Vec<i64>>,
    #[serde(rename = "suggested_topics")]
    pub suggested_topics: Option<Vec<SuggestedTopic>>,
    pub id: i64,
    pub title: Option<String>,
    #[serde(rename = "fancy_title")]
    pub fancy_title: Option<String>,
    #[serde(rename = "posts_count")]
    pub posts_count: Option<i64>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    pub views: Option<i64>,
    #[serde(rename = "reply_count")]
    pub reply_count: Option<i64>,
    #[serde(rename = "like_count")]
    pub like_count: Option<i64>,
    #[serde(rename = "last_posted_at")]
    pub last_posted_at: Option<String>,
    pub visible: Option<bool>,
    pub closed: Option<bool>,
    pub archived: Option<bool>,
    #[serde(rename = "has_summary")]
    pub has_summary: Option<bool>,
    pub archetype: Option<String>,
    pub slug: Option<String>,
    #[serde(rename = "category_id")]
    pub category_id: i64,
    #[serde(rename = "word_count")]
    pub word_count: Option<i64>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: ::serde_json::Value,
    #[serde(rename = "user_id")]
    pub user_id: Option<i64>,
    #[serde(rename = "featured_link")]
    pub featured_link: ::serde_json::Value,
    #[serde(rename = "pinned_globally")]
    pub pinned_globally: Option<bool>,
    #[serde(rename = "pinned_at")]
    pub pinned_at: Option<String>,
    #[serde(rename = "pinned_until")]
    pub pinned_until: ::serde_json::Value,
    #[serde(rename = "image_url")]
    pub image_url: ::serde_json::Value,
    #[serde(rename = "slow_mode_seconds")]
    pub slow_mode_seconds: Option<i64>,
    pub draft: ::serde_json::Value,
    #[serde(rename = "draft_key")]
    pub draft_key: Option<String>,
    #[serde(rename = "draft_sequence")]
    pub draft_sequence: Option<i64>,
    pub posted: Option<bool>,
    pub unpinned: Option<bool>,
    pub pinned: Option<bool>,
    #[serde(rename = "current_post_number")]
    pub current_post_number: Option<i64>,
    #[serde(rename = "highest_post_number")]
    pub highest_post_number: Option<i64>,
    #[serde(rename = "last_read_post_number")]
    pub last_read_post_number: Option<i64>,
    #[serde(rename = "last_read_post_id")]
    pub last_read_post_id: Option<i64>,
    #[serde(rename = "deleted_by")]
    pub deleted_by: ::serde_json::Value,
    #[serde(rename = "actions_summary")]
    pub actions_summary: Vec<ActionsSummary>,
    #[serde(rename = "chunk_size")]
    pub chunk_size: Option<i64>,
    pub bookmarked: Option<bool>,
    #[serde(rename = "topic_timer")]
    pub topic_timer: ::serde_json::Value,
    #[serde(rename = "message_bus_last_id")]
    pub message_bus_last_id: Option<i64>,
    #[serde(rename = "participant_count")]
    pub participant_count: Option<i64>,
    #[serde(rename = "show_read_indicator")]
    pub show_read_indicator: Option<bool>,
    pub thumbnails: ::serde_json::Value,
    #[serde(rename = "slow_mode_enabled_until")]
    pub slow_mode_enabled_until: ::serde_json::Value,
    pub details: Details,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostStream {
    pub posts: Vec<Post>,
    pub stream: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i64,
    pub name: Option<String>,
    pub username: String,
    #[serde(rename = "avatar_template")]
    pub avatar_template: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub cooked: String,
    #[serde(rename = "post_number")]
    pub post_number: Option<i64>,
    #[serde(rename = "post_type")]
    pub post_type: Option<i64>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "reply_count")]
    pub reply_count: Option<i64>,
    #[serde(rename = "reply_to_post_number")]
    pub reply_to_post_number: ::serde_json::Value,
    #[serde(rename = "quote_count")]
    pub quote_count: Option<i64>,
    #[serde(rename = "incoming_link_count")]
    pub incoming_link_count: Option<i64>,
    pub reads: Option<i64>,
    #[serde(rename = "readers_count")]
    pub readers_count: Option<i64>,
    pub score: f64,
    pub yours: Option<bool>,
    #[serde(rename = "topic_id")]
    pub topic_id: i64,
    #[serde(rename = "topic_slug")]
    pub topic_slug: Option<String>,
    #[serde(rename = "display_username")]
    pub display_username: Option<String>,
    #[serde(rename = "primary_group_name")]
    // pub primary_group_name: ::serde_json::Value,
    // #[serde(rename = "primary_group_flair_url")]
    // pub primary_group_flair_url: ::serde_json::Value,
    // #[serde(rename = "primary_group_flair_bg_color")]
    // pub primary_group_flair_bg_color: ::serde_json::Value,
    // #[serde(rename = "primary_group_flair_color")]
    // pub primary_group_flair_color: ::serde_json::Value,
    pub version: Option<i64>,
    #[serde(rename = "can_edit")]
    pub can_edit: Option<bool>,
    #[serde(rename = "can_delete")]
    pub can_delete: Option<bool>,
    #[serde(rename = "can_recover")]
    pub can_recover: Option<bool>,
    #[serde(rename = "can_wiki")]
    pub can_wiki: Option<bool>,
    pub read: Option<bool>,
    #[serde(rename = "user_title")]
    pub user_title: ::serde_json::Value,
    pub bookmarked: Option<bool>,
    #[serde(rename = "actions_summary")]
    pub actions_summary: Vec<ActionsSummary>,
    pub moderator: Option<bool>,
    pub admin: Option<bool>,
    pub staff: Option<bool>,
    #[serde(rename = "user_id")]
    pub user_id: Option<i64>,
    pub hidden: Option<bool>,
    #[serde(rename = "trust_level")]
    pub trust_level: Option<i64>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: ::serde_json::Value,
    #[serde(rename = "user_deleted")]
    pub user_deleted: Option<bool>,
    #[serde(rename = "edit_reason")]
    pub edit_reason: ::serde_json::Value,
    #[serde(rename = "can_view_edit_history")]
    pub can_view_edit_history: Option<bool>,
    pub wiki: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionsSummary {
    pub id: Option<i64>,
    pub count: Option<i64>,
    pub hidden: Option<bool>,
    #[serde(rename = "can_act")]
    pub can_act: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedTopic {
    pub id: Option<i64>,
    pub title: Option<String>,
    #[serde(rename = "fancy_title")]
    pub fancy_title: Option<String>,
    pub slug: Option<String>,
    #[serde(rename = "posts_count")]
    pub posts_count: Option<i64>,
    #[serde(rename = "reply_count")]
    pub reply_count: Option<i64>,
    #[serde(rename = "highest_post_number")]
    pub highest_post_number: Option<i64>,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "last_posted_at")]
    pub last_posted_at: Option<String>,
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
    pub unpinned: ::serde_json::Value,
    pub visible: Option<bool>,
    pub closed: Option<bool>,
    pub archived: Option<bool>,
    #[serde(rename = "notification_level")]
    pub notification_level: Option<i64>,
    pub bookmarked: Option<bool>,
    pub liked: Option<bool>,
    #[serde(rename = "like_count")]
    pub like_count: Option<i64>,
    pub views: Option<i64>,
    #[serde(rename = "category_id")]
    pub category_id: Option<i64>,
    #[serde(rename = "featured_link")]
    pub featured_link: ::serde_json::Value,
    pub posters: Vec<Poster>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Poster {
    pub extras: Option<String>,
    pub description: Option<String>,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "avatar_template")]
    pub avatar_template: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    #[serde(rename = "can_edit")]
    pub can_edit: Option<bool>,
    #[serde(rename = "notification_level")]
    pub notification_level: Option<i64>,
    #[serde(rename = "notifications_reason_id")]
    pub notifications_reason_id: Option<::serde_json::Value>,
    #[serde(rename = "can_create_post")]
    pub can_create_post: Option<bool>,
    #[serde(rename = "can_reply_as_new_topic")]
    pub can_reply_as_new_topic: Option<bool>,
    #[serde(rename = "can_flag_topic")]
    pub can_flag_topic: Option<bool>,
    pub participants: Vec<Participant>,
    #[serde(rename = "created_by")]
    pub created_by: User,
    #[serde(rename = "last_poster")]
    pub last_poster: User,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "avatar_template")]
    pub avatar_template: Option<String>,
    #[serde(rename = "post_count")]
    pub post_count: Option<i64>,
    #[serde(rename = "primary_group_name")]
    // pub primary_group_name: ::serde_json::Value,
    // #[serde(rename = "primary_group_flair_url")]
    // pub primary_group_flair_url: ::serde_json::Value,
    // #[serde(rename = "primary_group_flair_color")]
    // pub primary_group_flair_color: ::serde_json::Value,
    // #[serde(rename = "primary_group_flair_bg_color")]
    // pub primary_group_flair_bg_color: ::serde_json::Value,
    pub admin: Option<bool>,
    pub moderator: Option<bool>,
    #[serde(rename = "trust_level")]
    pub trust_level: Option<i64>,
}
