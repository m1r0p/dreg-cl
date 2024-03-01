///// local modules
pub use crate::conf::PORTUS_REST_TAGS;

///// external crates
use reqwest::header::{HeaderMap, HeaderName, ACCEPT};
use std::error::Error;

#[tokio::main]
pub async fn portus_delete_image(
    portus_address: &String,
    portus_user: &String,
    portus_token: &String,
    tag_id: &u64,
) -> Result<(), Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, format!("application/json").parse().unwrap());
    headers.insert(
        HeaderName::from_static("portus-auth"),
        format!("{}:{}", portus_user.clone(), portus_token.clone())
            .parse()
            .unwrap(),
    );

    let client = reqwest::Client::new();

    _ = client
        .delete(format!(
            "{}{}/{}",
            portus_address.clone(),
            PORTUS_REST_TAGS,
            &tag_id
        ))
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;

    return Ok(());
}
