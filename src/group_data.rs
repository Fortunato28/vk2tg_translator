use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Group {
    pub link: String,
    pub id: String,
}

impl Group {
    pub fn new(link: &str) -> Group {
        let link = link.to_owned();
        let id = Self::extract_group_id(&link).expect("Probably your group link is wrong.");
        Group { link, id }
    }

    fn extract_group_id(link: &str) -> Result<String> {
        dbg!(&link);
        let mut id = String::new();
        for letter in link.chars() {
            if letter.is_digit(10) {
                id.push(letter);
            }
        }

        if id.is_empty() {
            return Err(anyhow!("Group link has no id."));
        }

        Ok(id)
    }
}
