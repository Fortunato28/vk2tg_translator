use vk2tg_translator as v2t;

fn main() {
    let page = v2t::Page::new("https://vk.com/appi.retelling");
    let new_posts = v2t::check_new_posts(page.get_posts(), v2t::get_old_posts("url_storage.txt"));
    // TODO continue from here
    //v2t::Page::get_post_data(new_posts[0]);
}
