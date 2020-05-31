use teloxide::prelude::*;
use teloxide::types;

mod group_data;
mod lp_response;
mod meta_data;

use lp_response::Response;

pub async fn run(source: &str, target_channel: &str, storage: &str) {
    let group = group_data::Group::new(source);

    'outer: loop {
        let mut meta_data = meta_data::get_meta_data(&group).await.unwrap();

        let bot = Bot::from_env();

        'inner: loop {
            let update = lp_response::get_update(&meta_data).await.unwrap();
            match update {
                Response::Ok(resp) => {
                    //send to telegram

                    for post in resp.posts.iter() {
                        bot.send_message(
                            types::ChatId::ChannelUsername(target_channel.to_owned()),
                            &post.text,
                        )
                        .send()
                        .await
                        .log_on_error()
                        .await;
                    }

                    dbg!(&resp);
                    meta_data.set_ts(resp.ts.to_string());
                    continue 'inner;
                }
                Response::Err(_) => {
                    continue 'outer;
                }
            }
        }
    }
}
