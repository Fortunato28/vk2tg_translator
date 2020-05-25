use anyhow::{Context, Result};
use serde_json::Value;
use std::any::type_name;

// TODO delete
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug)]
pub struct Post {
    text: String,
    attach_link: String,
}

impl Post {
    pub fn new(text: &str, attach_link: &str) -> Post {
        Post {
            text: text.to_owned(),
            attach_link: attach_link.to_owned(),
        }
    }
}

pub struct Failure {
    pub code: u64,
}

pub struct Success {
    pub ts: u64,
    pub posts: Vec<Post>,
}

impl Success {
    pub fn new(ts: u64, posts: Vec<Post>) -> Success {
        Success { ts, posts }
    }
}

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

    let object = updates[0]
        .get("object")
        .context("No object field in update")?;

    let text = object
        .get("text")
        .context("No text field in object")?
        .as_str()
        .context("Text field is not a string")?;
    let attachments = object
        .get("attachments")
        .context("No attachments field in object")?
        .as_array()
        .context("Attachment field does not contain an array")?;

    // The only supported attachment type is photo
    let attach_type = "photo".to_owned();
    for attach in attachments {
        let attach_data = attach
            .get(&attach_type)
            .context("Attach type is not a photo!")?;

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

        let attach_link = attach_link
            .context("No attach link")?
            .as_str()
            .context("Attach link is not a string O.o")?;

        let post = Post::new(text, attach_link);
        posts.push(post);
    }

    dbg!(&posts);
    let parsed_response = Success::new(ts, posts);
    Ok(Response::Ok(parsed_response))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    //#[test]
    //fn failure() {
    //    assert_eq!(3, 3);
    //}

    //#[test]
    //fn get_ts() {
    //    let test_response = r#"{"ts":"564", "updates":[]}"#;
    //    let parsed_response = parse_response(test_response).unwrap();
    //    let ts = parsed_response.get_ts().unwrap();
    //    assert_eq!(ts, 564);
    //}

    //#[test]
    //fn bad_ts() {
    //    let test_response = r#"{"ts":"", "updates":[]}"#;
    //    let parsed_response = parse_response(test_response);
    //    assert!(parsed_response.is_err());
    //}

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
                                   "photo_604":"https:\/\/sun9-24.userapi.com\/c858336\/v858336100\/1f764e\/vqUKPgqlXkc.jpg",
                                   "photo_75":"https:\/\/sun9-45.userapi.com\/c858336\/v858336100\/1f764c\/GIcqz2vk0nk.jpg",
                                   "photo_807":"https:\/\/sun9-8.userapi.com\/c858336\/v858336100\/1f764f\/dzmg6oYnHik.jpg",
                                   "post_id":27,
                                   "text":"",
                             "user_id":100,
                             "width":1125
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
            Response::Ok(resp) => assert!(!resp.posts.is_empty()),
            Response::Err(_) => panic!("Wrong response parsing"),
        }
    }

    //#[test]
    //fn empty_updates() {
    //    let test_response = r#"{
    //"ts":"16",
    //"updates":
    //[]
    //}"#;
    //    let parsed_response = parse_response(test_response).unwrap();
    //    match parsed_response {
    //        Response::Ok(resp) => assert!(resp.posts.is_empty()),
    //        Response::Err(_) => panic!("Wrong response parsing"),
    //    }
    //}

    //#[test]
    //fn bad_data() {
    //    assert_eq!(3, 3);
    //}
}
