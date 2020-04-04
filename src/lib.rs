use scraper::Html;
use scraper::Selector;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Page {
    url: String,
    posts: Vec<LinkOnPost>,
}

#[derive(Debug)]
struct LinkOnPost {
    url: String,
}

impl LinkOnPost {
    pub fn new(page_url: String, tail: String) -> LinkOnPost {
        let url = page_url + "?w=" + &tail[1..];
        LinkOnPost { url }
    }
}

struct NewPost {
    url: String,
    pictures: Vec<String>,
    text: String,
    title: String,
}

impl Page {
    pub fn new(url: &str) -> Page {
        let url = url.to_owned();
        let data = Self::download_page(&url);
        let document = Html::parse_document(&data);
        let posts = Self::get_all_posts(document, &url);
        Page { url, posts }
    }

    fn download_page(url: &str) -> String {
        // TODO this shit only for testing
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

    fn get_all_posts(parsed_html: scraper::Html, page_url: &str) -> Vec<LinkOnPost> {
        // Reach posts on page
        let post_selector =
            Selector::parse(r#"div[class="pi_text"]"#).expect("Error while pi_text selector!");
        let posts = parsed_html.select(&post_selector).skip(1);

        // TODO refactoring it to the for_each()
        let mut links_on_posts = Vec::new();
        for post in posts {
            // Reach post data
            let link_selector = Selector::parse(r#"a"#).expect("Error while parse link selector");
            // TODO title here if whithout skip()
            let post_meta_info = post
                .select(&link_selector)
                .skip(1)
                .next()
                .expect("Error while getting post`s meta information");

            // Get Link
            let link = post_meta_info
                .value()
                .attr("href")
                .expect("Error while gettin link attribute");
            let result_link = LinkOnPost::new(page_url.to_string(), link.to_string());
            links_on_posts.push(result_link);
        }

        dbg!(&links_on_posts);
        links_on_posts
    }

    fn get_list_of_posts() -> Vec<String> {
        Vec::new()
    }
}

pub fn get_old_posts() -> Vec<String> {
    vec![]
}

pub fn check_new_posts(page: Page, old_posts: Vec<String>) -> Vec<String> {
    vec![]
}
