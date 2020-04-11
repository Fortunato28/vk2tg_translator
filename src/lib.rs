use scraper::Html;
use scraper::Selector;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub struct Page {
    url: String,
    // TODO maybe store here Html::document will be better and easy parsing later
    data: String,
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

    pub fn get_string(&self) -> &str {
        &self.url
    }
}

// TODO Well, maybe that will be usefull when I decided to send on the telegram channel data, not links
pub struct NewPost {
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
        Page { url, data, posts }
    }

    pub fn get_post_data(link: &str) -> NewPost {
        let url = link.to_owned();
        let pictures = vec!["S".to_owned()];
        let text = "Some text".to_owned();
        let title = "Some title".to_owned();

        NewPost {
            url,
            pictures,
            text,
            title,
        }
    }

    pub fn get_posts(&self) -> Vec<String> {
        self.posts
            .iter()
            .map(|post| post.get_string().to_owned())
            .collect()
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

        links_on_posts
    }
}

pub fn get_old_posts(storage: &str) -> Vec<String> {
    let file = File::open(storage).expect("Error while open url storage");
    let buf_reader = BufReader::new(&file);
    let mut posts_from_file = Vec::new();
    for line in buf_reader.lines() {
        posts_from_file.push(line.expect("Problem while read string"));
    }

    posts_from_file
}

pub fn check_new_posts(page_posts: Vec<String>, old_posts: Vec<String>) -> Vec<String> {
    let mut new_posts: Vec<String> = Vec::new();
    // TODO rewrite it to the iterators
    for page_post in &page_posts {
        if !old_posts.contains(page_post) {
            new_posts.push(page_post.to_owned())
        }
    }

    new_posts
}

pub fn consume_new_posts(new_posts: Vec<String>, storage: &str) {
    let mut file_storage = OpenOptions::new()
        .append(true)
        .open(storage)
        .expect("Problem while open storage for appending");

    for post in new_posts.iter() {
        if let Err(e) = writeln!(file_storage, "{}", post) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

// TODO get vec and vec -> easy to test
pub fn remove_old_posts(page_posts: Vec<String>, storage: &str) {
    let file = File::open(storage).expect("Error while open url storage");
    let buf_reader = BufReader::new(&file);
    let mut posts_from_file = Vec::new();
    for line in buf_reader.lines() {
        posts_from_file.push(line.expect("Problem while read string"));
    }

    let mut result_urls_to_store = Vec::new();
    for post_from_file in posts_from_file {
        if page_posts.contains(&post_from_file) {
            result_urls_to_store.push(post_from_file);
        }
    }

    //// Remove all data from file
    //file.set_len(0).expect("Error while remove file data");
    drop(file);

    let mut file_storage = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(storage)
        .expect("Problem while open storage for appending");
    dbg!(&result_urls_to_store);

    result_urls_to_store.iter().for_each(|line| {
        writeln!(file_storage, "{}", line).expect("Problem while write line in storage");
        ()
    });
}
