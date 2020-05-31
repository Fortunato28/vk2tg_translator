use telegraph_rs::{html_to_node, Telegraph};

pub fn make_new_article() -> String {
    let telegraph = Telegraph::new("vk2tg_translator").create().unwrap();
    let page = telegraph
        .create_page(
            "title",
            &html_to_node(
                r#"
                <img alt="Qries" src="https://sun9-22.userapi.com/c633124/v633124129/2c314/xfnUjyCrviA.jpg"
                width=150" height="70">
                  "#,
            ),
            false,
        )
        .unwrap();
    page.url
}
