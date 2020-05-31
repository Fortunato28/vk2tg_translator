use clap::{App, Arg};
use std::{thread, time};
use vk2tg_translator as v2t;

#[tokio::main]
async fn main() {
    //let cli = App::new("Translator from vkontakte group to telegram channel")
    //    .version("0.1.0")
    //    .author("Sapunov Anton <fort.sav.28@gmail.com>")
    //    .args_from_usage(
    //        "
    //        -f, --from=[link] 'Link to some group in vk'
    //        -t, --to=[channel name] 'Telegram channel name'
    //        -s, --storage[storage filename] 'Filename for storing already published post'
    //        ",
    //    )
    //    .get_matches();

    //let source_vk_group = cli.value_of("from").expect("No required [from] parameter");
    //let target_channel = cli.value_of("to").expect("No required [to] parameter");
    //let storage = cli.value_of("storage").unwrap_or("test_url_storage.txt");

    //teloxide::enable_logging!();
    //log::info!("Starting vk2tg_translator_bot!");

    let source = "https://vk.com/club192827874";
    let target_channel = "@vk2tg_test_channel";
    // TODO Has to be connected with group id
    let storage = "last_id_test";

    dbg!(&"New hour, new iteration!");
    v2t::run(source, target_channel, storage).await;
}
