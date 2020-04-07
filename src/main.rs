use vk2tg_translator as v2t;
use teloxide::prelude::*;
use teloxide::types;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting vk2tg_translator_bot!");

    let page = v2t::Page::new("https://vk.com/appi.retelling");
    let new_posts = v2t::check_new_posts(page.get_posts(), v2t::get_old_posts("url_storage.txt"));

    let bot = Bot::from_env();

    for x in new_posts.iter() {
        bot.send_message(types::ChatId::ChannelUsername("@vk2tg_test_channel".to_owned()), x).send().await.log_on_error().await;
    }
}
