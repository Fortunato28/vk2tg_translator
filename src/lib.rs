use anyhow::{anyhow, Result};
use teloxide::prelude::*;
use teloxide::types;

mod group_data;

pub async fn run(source: &str, target_channel: &str, storage: &str) {
    let new_posts = vec!["str1".to_owned(), "str2".to_owned()];
    let group = group_data::Group::new(source);
    let meta_data = get_meta_data(&group.id, "").await;

    //let bot = Bot::from_env();

    //for x in new_posts.iter().rev() {
    //    bot.send_message(types::ChatId::ChannelUsername(target_channel.to_owned()), x)
    //        .send()
    //        .await
    //        .log_on_error()
    //        .await;
    //}
}

#[derive(Debug)]
struct MetaData {
    key: String,
    server: String,
    ts: u64,
}

async fn get_meta_data(group_id: &str, access_token: &str) -> Result<MetaData> {
    let group_id = "192827874";
    let acces_token = "";
    let request = r#"https://api.vk.com/method/groups.getLongPollServer/?&v=5.103&group_id="#;
    //\&access_token\=

    let result_request = format!("{}{}&access_token={}", request, group_id, access_token);
    dbg!(&result_request);
    let key = "key".to_owned();
    let server = "server".to_owned();
    let ts = 0;
    Ok(MetaData { key, server, ts })
}
