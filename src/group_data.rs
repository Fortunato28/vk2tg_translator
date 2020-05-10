use anyhow::{anyhow, Result};
use std::env;

#[derive(Debug)]
pub struct Group {
    pub link: String,
    pub id: String,
    pub access_token: String,
}

impl Group {
    pub fn new(link: &str) -> Group {
        let link = link.to_owned();
        let id = Self::extract_group_id(&link).expect("Probably your group link is wrong.");

        let access_token =
            env::var("VK_GROUP_TOKEN").expect("No VK_GROUP_TOKEN environment variable");
        Group {
            link,
            id,
            access_token,
        }
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
