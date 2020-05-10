use teloxide::prelude::*;
use teloxide::types;

mod group_data;
mod meta_data;

pub async fn run(source: &str, target_channel: &str, storage: &str) {
    //let new_posts = vec!["str1".to_owned(), "str2".to_owned()];
    let group = group_data::Group::new(source);
    let meta_data = meta_data::get_meta_data(&group).await.unwrap();
    let new_post = perform_lp_request(meta_data).await;

    //let bot = Bot::from_env();

    //for x in new_posts.iter().rev() {
    //    bot.send_message(types::ChatId::ChannelUsername(target_channel.to_owned()), x)
    //        .send()
    //        .await
    //        .log_on_error()
    //        .await;
    //}
}

async fn perform_lp_request(meta_data: meta_data::MetaData) -> String {
    dbg!(&meta_data);
    String::new()
}
