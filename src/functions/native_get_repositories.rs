///// local modules
pub use crate::conf::NATIVE_REST_CATALOG;

///// external crates
//use reqwest::header::{HeaderMap, HeaderName, ACCEPT};
use serde_json::{Map, Value};
use std::error::Error;

#[tokio::main]
pub async fn native_get_repositories(
    registry_address: &String,
    registry_user: &String,
    registry_password: &String,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut repo_list: Vec<String> = Vec::new();
    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "{}{}",
            registry_address.clone(),
            NATIVE_REST_CATALOG
        ))
        .basic_auth(&registry_user, Some(&registry_password))
        .send()
        .await?
        .text()
        .await?;
    let raw_json: Value = serde_json::from_str(resp.as_str()).unwrap();
    let repos_obj_json: &Map<String, Value> = raw_json.as_object().unwrap();
    let repos_raw: &Vec<Value> = repos_obj_json["repositories"].as_array().unwrap();
    for repo_raw in repos_raw.iter() {
        let repo: String = serde_json::from_value(repo_raw.to_owned()).unwrap();
        repo_list.push(repo);
    }

    return Ok(repo_list);
}
