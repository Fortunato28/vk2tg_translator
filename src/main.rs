use vk2tg_translator as v2t;

fn main() {
    let page = v2t::Page::new("https://vk.com/appi.retelling");
    v2t::check_new_posts(page, v2t::get_old_posts());
}
