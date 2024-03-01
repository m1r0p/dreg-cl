///// local modules
pub use crate::conf::PORTUS_REST_TAGS;
pub use crate::structures::tag::Tag;

///// external crates
use reqwest::header::{HeaderMap, HeaderName, ACCEPT};
use serde_json::Value;
use std::error::Error;

#[tokio::main]
pub async fn portus_get_tags_of_images(
    portus_address: &String,
    portus_user: &String,
    portus_token: &String,
) -> Result<Vec<Tag>, Box<dyn Error>> {
    let mut tag_list: Vec<Tag> = Vec::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, format!("application/json").parse().unwrap());
    headers.insert(
        HeaderName::from_static("portus-auth"),
        format!("{}:{}", portus_user.clone(), portus_token.clone())
            .parse()
            .unwrap(),
    );

    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{}{}", portus_address.clone(), PORTUS_REST_TAGS))
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;
    let raw_json: Value = serde_json::from_str(resp.as_str()).unwrap();
    let json_vec: &Vec<Value> = raw_json.as_array().unwrap();
    for json_tag in json_vec.iter() {
        let tag: Tag = serde_json::from_value(json_tag.to_owned()).unwrap();
        tag_list.push(tag);
    }

    return Ok(tag_list);
}
