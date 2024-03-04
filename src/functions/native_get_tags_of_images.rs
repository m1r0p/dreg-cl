///// local modules

///// external crates
use reqwest::header::{HeaderMap, ACCEPT};
use serde_json::{Map, Value};
use std::error::Error;

#[tokio::main]
pub async fn native_get_tags_of_images(
    registry_address: &String,
    registry_user: &String,
    registry_password: &String,
    registry_repo: &String,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut tag_list: Vec<String> = Vec::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, format!("application/json").parse().unwrap());

    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "{}/v2/{}/tags/list",
            registry_address, registry_repo
        ))
        .basic_auth(&registry_user, Some(&registry_password))
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;
    let raw_json: Value = serde_json::from_str(resp.as_str()).unwrap();
    let tags_obj_json: &Map<String, Value> = raw_json.as_object().unwrap();
    let tags_raw: &Vec<Value> = tags_obj_json["tags"].as_array().unwrap();
    for tag_raw in tags_raw.iter() {
        let tag: String = serde_json::from_value(tag_raw.to_owned()).unwrap();
        tag_list.push(tag);
    }

    return Ok(tag_list);
}
