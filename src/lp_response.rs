use anyhow::{Context, Result};
use serde_json::Value;

pub struct Post {
    text: String,
}

pub fn parse_response(response: &str) -> Result<(u64, Vec<Post>)> {
    let json_response: Value = serde_json::from_str(&response)?;
    let ts = json_response["ts"]
        .as_str()
        .context("No timestemp in response")?
        .parse::<u64>()
        .context("Cannot transform timestemp to u64")?;

    Ok((
        ts,
        vec![Post {
            text: "test".to_owned(),
        }],
    ))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn get_ts() {
        let test_response = r#"{"ts":"564", "updates":"nothing"}"#;
        let (ts, _) = parse_response(test_response).unwrap();
        assert_eq!(ts, 564);
    }

    #[test]
    fn get_update() {
        assert_eq!(3, 3);
    }

    #[test]
    fn bad_data() {
        assert_eq!(3, 3);
    }
}