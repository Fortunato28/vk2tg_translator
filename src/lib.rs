use scraper::Html;
use scraper::Selector;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Page {
    url: String,
    // TODO maybe there are no reason to store that
    data: String,
    posts: Vec<Post>,
}

struct Post {}

impl Page {
    pub fn new(url: &str) -> Page {
        let url = url.to_owned();
        let data = Self::download_page(&url);
        let document = Html::parse_document(&data);
        Self::test_output(document);
        let posts = Self::get_all_posts(&data);
        Page { url, data, posts }
    }

    fn download_page(url: &str) -> String {
        if Path::new("retelling.txt").exists() {
            let mut file = File::open("retelling.txt").unwrap();
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .expect("Cannot read from file!");
            return buffer;
        }

        // If there are no needed file let`s download it
        let mut res = reqwest::blocking::get(url).expect("Cannot perform get reqwest.");
        let mut body = String::new();
        res.read_to_string(&mut body)
            .expect("Problem while read response to string");

        body
    }

    fn test_output(parsed_html: scraper::Html) {
        let post_selector =
            Selector::parse(r#"div[class="pi_text"]"#).expect("Error while parse selector!");
        let first_post = parsed_html
            .select(&post_selector)
            .skip(1)
            .next()
            .expect("Error while getting the first post!");

        //dbg!(first_post.inner_html());

        let link_selector = Selector::parse(r#"a"#).expect("Error while parse link selector");
        let part = first_post
            .select(&link_selector)
            .skip(1)
            .next()
            .expect("Error while getting link");
        dbg!(part.inner_html());

        let link = part.value().attr("href");
        dbg!(link);
    }

    fn get_all_posts(page: &str) -> Vec<Post> {
        Vec::new()
    }

    fn get_list_of_posts() -> Vec<String> {
        Vec::new()
    }
}

fn find_start_at(slice: &str, at: usize, pattern: &str) -> Option<usize> {
    slice[at + 1..].find(pattern).map(|i| at + i)
}
