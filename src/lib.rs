use std::io::Read;

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
        let posts = Self::get_all_posts();
        Page { url, data, posts }
    }

    fn download_page(url: &str) -> String {
        let mut res = reqwest::blocking::get(url).expect("Cannot perform get reqwest.");
        let mut body = String::new();
        res.read_to_string(&mut body)
            .expect("Problem while read response to string");

        body
    }

    fn get_all_posts() -> Vec<Post> {
        Vec::new()
    }

    fn get_list_of_posts() -> Vec<String> {
        Vec::new()
    }
}
