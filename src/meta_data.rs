use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::group_data;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub key: String,
    pub server: String,
    pub ts: String,
}

pub async fn get_meta_data(group: &group_data::Group) -> Result<MetaData> {
    let request = r#"https://api.vk.com/method/groups.getLongPollServer/?&v=5.103&group_id="#;

    let result_request = format!(
        "{}{}&access_token={}",
        request, group.id, group.access_token
    );

    let res = reqwest::get(&result_request)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Remove responce field from response
    let mut res = res.replace(r#"{"response":"#, "");
    res.pop();

    let meta_data: MetaData = serde_json::from_str(&res)?;
    Ok(meta_data)
}
