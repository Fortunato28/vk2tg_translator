use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use teloxide::prelude::*;
use teloxide::types;

mod group_data;

pub async fn run(source: &str, target_channel: &str, storage: &str) {
    //let new_posts = vec!["str1".to_owned(), "str2".to_owned()];
    let group = group_data::Group::new(source);
    let meta_data = get_meta_data(&group).await;
    dbg!(&meta_data);

    //let bot = Bot::from_env();

    //for x in new_posts.iter().rev() {
    //    bot.send_message(types::ChatId::ChannelUsername(target_channel.to_owned()), x)
    //        .send()
    //        .await
    //        .log_on_error()
    //        .await;
    //}
}

#[derive(Debug, Serialize, Deserialize)]
struct MetaData {
    key: String,
    server: String,
    ts: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VkResponse {
    pub response: String,
}

async fn get_meta_data(group: &group_data::Group) -> Result<MetaData> {
    let request = r#"https://api.vk.com/method/groups.getLongPollServer/?&v=5.103&group_id="#;

    let result_request = format!(
        "{}{}&access_token={}",
        request, group.id, group.access_token
    );

    let res = reqwest::get(&result_request)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Remove responce field from response
    let mut res = res.replace(r#"{"response":"#, "");
    res.pop();

    let meta_data: MetaData = serde_json::from_str(&res)?;
    Ok(meta_data)
}
