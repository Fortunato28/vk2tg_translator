use anyhow::{Context, Result};
use serde_json::Value;
use std::any::type_name;

// TODO delete
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub struct Post {
    text: String,
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
        .context("Updates field has not contain an array")?;
    let posts: Vec<Post> = vec![];
    let parsed_response = Success::new(ts, posts);
    if updates.is_empty() {
        return Ok(Response::Ok(parsed_response));
    }

    Ok(Response::Ok(parsed_response))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn failure() {
        assert_eq!(3, 3);
    }

    #[test]
    fn get_ts() {
        let test_response = r#"{"ts":"564", "updates":[]}"#;
        let parsed_response = parse_response(test_response).unwrap();
        let ts = parsed_response.get_ts().unwrap();
        assert_eq!(ts, 564);
        //match parsed_response {
        //    Response::Ok(x) => assert_eq!(x.ts, 564),
        //    _ => (),
        //}
        //assert_eq!(ts, 564);
    }

    //#[test]
    //fn bad_ts() {
    //    let test_response = r#"{"ts":"", "updates":"nothing"}"#;
    //    let ts_error = parse_response(test_response);
    //    assert!(ts_error.is_err());
    //}

    //#[test]
    //fn get_update() {
    //    let test_response = r#"{
    //"ts":"16",
    //"updates":
    //[
    //    {
    //        "type":"wall_post_new",
    //        "object":
    //        {
    //            "id":16,
    //            "from_id":69751141,
    //            "owner_id":-192827874,
    //            "date":1589992602,
    //            "marked_as_ads":0,
    //            "post_type":"post",
    //            "text":"fvdfvd",
    //            "can_edit":1,
    //            "created_by":69751141,
    //            "can_delete":1,
    //            "attachments":
    //            [
    //                {
    //                    "type":"photo",
    //                    "photo":
    //                    {
    //                        "id":457239019,
    //                        "album_id":-8,
    //                        "owner_id":-192827874,
    //                        "user_id":100,
    //                        "photo_75":"https:\/\/sun1-97.userapi.com\/-jvQYpQNCxoUuc7dsIHzKoSQUYXMml3nX6IbsA\/mpvsDR8NaPA.jpg",
    //                        "photo_130":"https:\/\/sun1-89.userapi.com\/DwIt8XcKO5bdFH2a56dP9w7ApdLUZsxqi5kgXw\/q1dbkQ3MaBo.jpg",
    //                        "photo_604":"https:\/\/sun1-92.userapi.com\/JMKB28PJw50fsCLfDkjq6Tijn7BI2ar3tz2xgw\/AQiQEFRT5ps.jpg",
    //                        "photo_807":"https:\/\/sun1-20.userapi.com\/hZpdOxh1eH3bWQsGluhGdjBH_aVs8fAB2vH-aA\/mKLKpumhFVQ.jpg",
    //                        "width":700,
    //                        "height":488,
    //                        "text":"",
    //                        "date":1589992602,
    //                        "post_id":16,
    //                        "access_key":"71882b9b66a35769aa"
    //                    }
    //                }
    //            ],
    //            "comments":
    //            {
    //                "count":0
    //            }
    //        },
    //        "group_id":192827874,
    //        "event_id":"43e633c025f3aae41d90f1ef5d2cbb4aaaf719b0"
    //    }
    //]
    //}"#;
    //    let (_, _) = parse_response(test_response).unwrap();
    //    assert_eq!(3, 3);
    //}

    //#[test]
    //fn empty_updates() {
    //    let test_response = r#"{
    //"ts":"16",
    //"updates":
    //[]
    //}"#;
    //    let (_, updates) = parse_response(test_response).unwrap();
    //    assert!(updates.is_empty());
    //}

    //#[test]
    //fn bad_data() {
    //    assert_eq!(3, 3);
    //}
}
