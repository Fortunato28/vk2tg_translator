use anyhow::{Context, Result};
use serde_json::Value;
use std::any::type_name;

use crate::meta_data;

pub async fn get_update(meta_data: &meta_data::MetaData) -> Result<Response> {
    let response = perform_lp_request(meta_data).await;
    parse_response(&response)
}

pub async fn perform_lp_request(meta_data: &meta_data::MetaData) -> String {
    let request = r#"https://lp.vk.com/wh192827874?act=a_check&key="#;

    let result_request = format!(
        "{}{}&wait=30&mode=2&ts={}",
        request, meta_data.key, meta_data.ts
    );

    let res = reqwest::get(&result_request)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    res
}

#[derive(Debug)]
pub struct Post {
    pub text: String,
    pub attach_links: Vec<String>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            text: String::new(),
            attach_links: vec![],
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.text = text.to_owned();
    }

    pub fn add_attach_link(&mut self, link: &str) {
        self.attach_links.push(link.to_owned());
    }
}

#[derive(Debug)]
pub struct Failure {
    pub code: u64,
}

#[derive(Debug)]
pub struct Success {
    pub ts: u64,
    pub posts: Vec<Post>,
}

impl Success {
    pub fn new(ts: u64, posts: Vec<Post>) -> Success {
        Success { ts, posts }
    }
}

#[derive(Debug)]
pub enum Response {
    Err(Failure),
    Ok(Success),
}

impl Response {
    pub fn get_ts(&self) -> Option<u64> {
        match self {
            Response::Ok(success) => return Some(success.ts),
            Response::Err(_) => return None,
        }
    }
}

pub fn parse_response(response: &str) -> Result<Response> {
    let json_response: Value = serde_json::from_str(&response)?;

    let failure_code = json_response.get("failed");
    if failure_code.is_some() {
        let result_code = failure_code
            .unwrap()
            .as_u64()
            .context("Something wrong with failure response")?;
        let failure = Failure { code: result_code };
        let parsed_response = Response::Err(failure);
        return Ok(parsed_response);
    }
    let ts = json_response["ts"]
        .as_str()
        .context("No timestemp in response")?
        .parse::<u64>()
        .context("Cannot transform timestemp to u64")?;

    let updates = json_response
        .get("updates")
        .context("No updates field in response")?
        .as_array()
        .context("Updates field does not contain an array")?;
    let mut posts = vec![];
    if updates.is_empty() {
        let parsed_response = Success::new(ts, posts);
        return Ok(Response::Ok(parsed_response));
    }

    // Parse updates
    for update in updates {
        let mut post = Post::new();
        let object = update.get("object").context("No object field in update")?;

        let text = object
            .get("text")
            .context("No text field in object")?
            .as_str()
            .context("Text field is not a string")?;
        post.add_text(text);

        let attachments = object.get("attachments");
        if attachments.is_none() {
            posts.push(post);
            continue;
        }
        // Here attachments data certanly is Some(value)
        let attachments = attachments.unwrap().as_array();
        if attachments.is_none() {
            posts.push(post);
            continue;
        }
        let attachments = attachments.unwrap();

        // The only supported attachment type is photo
        let attach_type = "photo".to_owned();
        for attach in attachments {
            let attach_data = attach.get(&attach_type);

            // No attach data - okey, skip it
            if attach_data.is_none() {
                continue;
            }

            // Here attach data certanly is Some(value)
            let attach_data = attach_data.unwrap();

            // Let`s find suitable photo-field
            let mut attach_link = attach_data.get(&format!("{}{}", attach_type, "_1280"));
            if attach_link.is_none() {
                attach_link = attach_data.get(&format!("{}{}", attach_type, "_807"));
                if attach_link.is_none() {
                    attach_link = attach_data.get(&format!("{}{}", attach_type, "_604"));
                    if attach_link.is_none() {
                        continue;
                    }
                }
            }

            // Here attach data certanly is Some(value)
            if let Some(attach_link) = attach_link.unwrap().as_str() {
                post.add_attach_link(attach_link);
            }
        }

        posts.push(post);
    }

    let parsed_response = Success::new(ts, posts);
    Ok(Response::Ok(parsed_response))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn failure() {
        let test_response = r#"{"failed":2}"#;
        let parsed_response = parse_response(test_response).unwrap();

        match parsed_response {
            Response::Ok(_) => panic!("Wrong response parsing"),
            Response::Err(resp) => assert_eq!(resp.code, 2),
        }
    }

    #[test]
    fn get_ts() {
        let test_response = r#"{"ts":"564", "updates":[]}"#;
        let parsed_response = parse_response(test_response).unwrap();
        let ts = parsed_response.get_ts().unwrap();
        assert_eq!(ts, 564);
    }

    #[test]
    fn bad_ts() {
        let test_response = r#"{"ts":"", "updates":[]}"#;
        let parsed_response = parse_response(test_response);
        assert!(parsed_response.is_err());
    }

    #[test]
    fn get_update() {
        let test_response = r#"
        {
            "ts":"27",
            "updates":
            [
               {
                   "type":"wall_post_new",
                   "object":
                   {
                       "id":26,
                       "from_id":69751141,
                       "owner_id":-192827874,
                       "date":1590342624,
                       "marked_as_ads":0,
                       "post_type":"post",
                       "text":"ак сложилось,",
                       "can_edit":1,
                       "created_by":69751141,
                       "can_delete":1,
                       "attachments":
                       [
                           {
                               "type":"photo",
                               "photo":
                               {
                                   "album_id":-8,
                                   "date":1590342624,
                                   "id":457239021,
                                   "owner_id":-192827874,
                                   "has_tags":false,
                                   "access_key":"204c3192e9a01342f3",
                                   "height":1080,
                                   "photo_1280":"https:\/\/sun1-95.userapi.com\/v5o9r-xfYt7K4SkkXf0atWLu_KvpCOwvffLKZg\/ozDaMS4IWjw.jpg",
                                   "photo_130":"https:\/\/sun1-83.userapi.com\/i-EhbtR2MrF_vWTXNQH5QNuSiCFiolIdP-xxOw\/v7q0Wu3au9c.jpg",
                                   "photo_604":"https:\/\/sun1-92.userapi.com\/k8yr1IMsRDFcN_2VgKO1HGG5N3vrlJrC_kw6gw\/3z1uYQk70uU.jpg",
                                   "photo_75":"https:\/\/sun1-94.userapi.com\/DzCGFHkmJJ-cwKO9UvvHTmdwfp4A-NC9evu05w\/LoY1fAHuTPw.jpg",
                                   "photo_807":"https:\/\/sun1-21.userapi.com\/uQeCuz0_tZrvx1wx3nMBmWdKBg1mYJzrOaBFRw\/9Q9SBpsB_dM.jpg",
                                   "post_id":26,
                                   "text":"",
                                   "user_id":100,
                                   "width":1080
                               }
                           },

                           {
                               "type":"photo"
                           }
                       ],
                       "comments":
                       {
                           "count":0
                       }
                   },
                   "group_id":192827874,
                   "event_id":"b99ca8705ee2387e7e685b8868a32a9d2a8a05c5"
               },
               {
                   "type":"wall_post_new",
                   "object":
                   {
                       "id":27,
                       "from_id":69751141,
                       "owner_id":-192827874,
                       "date":1590342681,
                       "marked_as_ads":0,
                       "post_type":"post",
                       "text":"аки шало",
                       "can_edit":1,
                       "created_by":69751141,
                       "can_delete":1,
                       "attachments":
                       [
                           {
                               "type":"photo",
                               "photo":
                               {
                                   "album_id":-8,
                                   "date":1590342681,
                                   "id":457239022,
                                   "owner_id":-192827874,
                                   "has_tags":false,
                                   "access_key":"55bd767dd4209f9c1e",
                                   "height":1157,
                                   "photo_1280":"https:\/\/sun9-29.userapi.com\/c858336\/v858336100\/1f7650\/-SaCtUpJLcI.jpg",
                                   "photo_130":"https:\/\/sun9-46.userapi.com\/c858336\/v858336100\/1f764d\/zbjeN2_5ny0.jpg",
                                   "photo_2560":"https:\/\/sun9-67.userapi.com\/c858336\/v858336100\/1f7651\/26VJRmbZGw4.jpg",
                                   "photo_75":"https:\/\/sun9-45.userapi.com\/c858336\/v858336100\/1f764c\/GIcqz2vk0nk.jpg",
                                   "photo_807":"https:\/\/sun9-8.userapi.com\/c858336\/v858336100\/1f764f\/dzmg6oYnHik.jpg",
                                   "post_id":27,
                                   "text":"",
                             "user_id":100,
                             "width":1125
                               }
                           },

                           {
                               "type":"photo",
                               "photo":
                               {
                                   "photo_604":"https:\/\/sun9-24.userapi.com\/c858336\/v858336100\/1f764e\/vqUKPgqlXkc.jpg"
                               }
                           }
                       ],
                       "comments":
                       {
                           "count":0
                       }
                   },
                   "group_id":192827874,
                   "event_id":"fb9e07ceb57000a3337c9af19097f8820a00e245"
               }
            ]
        }
        "#;
        let parsed_response = parse_response(test_response).unwrap();
        match parsed_response {
            Response::Ok(resp) => {
                assert_eq!(resp.posts[0].text, "ак сложилось,");
                assert_eq!(resp.posts[0].attach_links.len(), 1);
                assert_eq!(resp.posts[1].text, "аки шало");
                assert_eq!(
                    resp.posts[1].attach_links[1],
                    r#"https://sun9-24.userapi.com/c858336/v858336100/1f764e/vqUKPgqlXkc.jpg"#
                );
            }
            Response::Err(_) => panic!("Wrong response parsing"),
        }
    }

    #[test]
    fn empty_updates() {
        let test_response = r#"{
    "ts":"16",
    "updates":
    []
    }"#;
        let parsed_response = parse_response(test_response).unwrap();
        match parsed_response {
            Response::Ok(resp) => assert!(resp.posts.is_empty()),
            Response::Err(_) => panic!("Wrong response parsing"),
        }
    }

    // Absolutely wrong string
    #[test]
    fn bad_data() {
        let test_response = r#"just some string"#;

        let parsed_response = parse_response(test_response);
        assert!(parsed_response.is_err());
    }

    #[test]
    fn no_attachments() {
        let test_response = r#"
        {
            "ts":"27",
            "updates":
            [
               {
                   "type":"wall_post_new",
                   "object":
                   {
                       "text":"xbbb ,",
                       "comments":
                       {
                           "count":0
                       }
                   },
                   "group_id":192827874,
                   "event_id":"b99ca8705ee2387e7e685b8868a32a9d2a8a05c5"
               }
            ]
        }
        "#;

        let parsed_response = parse_response(test_response).unwrap();
        match parsed_response {
            Response::Ok(resp) => assert_eq!(resp.posts[0].text, "xbbb ,"),
            Response::Err(_) => panic!("Wrong response parsing"),
        }
    }
}
