use telegraph_rs::{html_to_node, Telegraph};

use crate::lp_response;

#[derive(Debug)]
pub struct Article {
    url: String,
}

impl Article {
    pub fn new(post: &lp_response::Post) -> Article {
        let telegraph = Telegraph::new("vk2tg_translator").create().unwrap();

        let content = Self::make_content(post);
        let page = telegraph
        .create_page(
            "New fucking post",
            &html_to_node(
                r#"
                <img alt="Qries" src="https://sun9-22.userapi.com/c633124/v633124129/2c314/xfnUjyCrviA.jpg"
                <img alt="Qries" src="https://sun9-22.userapi.com/c633124/v633124129/2c314/xfnUjyCrviA.jpg"
                  "#,
            ),
            false,
        )
        .unwrap();

        Article { url: page.url }
    }

    fn make_content(post: &lp_response::Post) -> String {
        let image_template = r#"<img alt="Qries" src=""#;
        dbg!(&"yeah");
        String::new()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn failure() {}
}
