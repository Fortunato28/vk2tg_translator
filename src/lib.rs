use teloxide::prelude::*;
use teloxide::types;

mod group_data;
mod lp_response;
mod meta_data;

pub async fn run(source: &str, target_channel: &str, storage: &str) {
    //let new_posts = vec!["str1".to_owned(), "str2".to_owned()];
    let group = group_data::Group::new(source);
    let meta_data = meta_data::get_meta_data(&group).await.unwrap();
    let new_posts = perform_lp_request(meta_data).await;

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

    let request = r#"https://lp.vk.com/wh192827874?act=a_check&key="#;

    let result_request = format!(
        "{}{}&wait=30&mode=2&ts={}",
        request, meta_data.key, meta_data.ts
    );

    println!("{}", &result_request);
    let res = reqwest::get(&result_request)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", &res);
    let (ts, posts) = lp_response::parse_response(&res).unwrap();
    String::new()
}
