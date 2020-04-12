use vk2tg_translator as v2t;
use teloxide::prelude::*;
use teloxide::types;
use clap::{Arg, App};
use std::{thread, time};

#[tokio::main]
async fn main() {
    let cli = App::new("Translator from vkontakte group to telegram channel")
        .version("0.1.0")
        .author("Sapunov Anton <fort.sav.28@gmail.com")
        .args_from_usage(
            "
            -f, --from=[link] 'Link to some group in vk'
            -t, --to=[channel name] 'Telegram channel name'
            -s, --storage[storage filename] 'Filename for storing already published post'
            "
        )
        .get_matches();

    let source_vk_group = cli.value_of("from").expect("No required [from] parameter");
    let target_channel = cli.value_of("to").expect("No required [to] parameter");
    let storage = cli.value_of("storage").unwrap_or("test_url_storage.txt");

    loop {
        run(source_vk_group, target_channel, storage).await;

        thread::sleep(time::Duration::from_secs(3600));
    }
}


async fn run(source: &str, target_channel: &str, storage: &str) {

    teloxide::enable_logging!();
    log::info!("Starting vk2tg_translator_bot!");

    let page = v2t::Page::new(source).await;
    let new_posts = v2t::check_new_posts(page.get_posts(), v2t::get_old_posts(storage));

    let bot = Bot::from_env();

    for x in new_posts.iter().rev() {
        bot.send_message(types::ChatId::ChannelUsername(target_channel.to_owned()), x).send().await.log_on_error().await;
    }

    v2t::consume_new_posts(new_posts, storage);
    v2t::remove_old_posts(page.get_posts(), storage);
}
