use scraper::Html;
use teloxide::prelude::*;
use teloxide::types;

pub async fn run(source: &str, target_channel: &str, storage: &str) {
    let new_posts = vec!["str1".to_owned(), "str2".to_owned()];

    let bot = Bot::from_env();

    for x in new_posts.iter().rev() {
        bot.send_message(types::ChatId::ChannelUsername(target_channel.to_owned()), x)
            .send()
            .await
            .log_on_error()
            .await;
    }
}
