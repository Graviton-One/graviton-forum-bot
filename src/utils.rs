use teloxide::utils::html::link;

use crate::models::forum::{ResponseTopics, Topic};

pub fn numbered_html_link(json_value: ResponseTopics) -> String {
    let mut draft = String::new();
    json_value
        .topic_list
        .topics
        .iter()
        .enumerate()
        .for_each(|(index, ch)| {
            let Topic {
                id,
                title,
                posts_count,
                reply_count,
                like_count,
                views,
                ..
            } = &ch;
            let url = &format!("https://forum.graviton.one/{}", id.to_string());
            let post_link = link(url, &title);
            let num = index + 1;

            let for_draft = format!(
                "{}. {} \nid: {}, posts: {}, replies: {}, likes: {}, views: {}\n",
                &num, &post_link, &id, &posts_count, &reply_count, &like_count, &views
            );
            draft.push_str(&for_draft);
        });
    draft
}
