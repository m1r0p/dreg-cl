///// local modules

///// external crates
use reqwest::header::{HeaderMap, ACCEPT};
use std::error::Error;

#[tokio::main]
pub async fn native_delete_manifest(
    registry_address: &String,
    registry_user: &String,
    registry_password: &String,
    registry_repo: &String,
    registry_digest: &String,
) -> Result<(), Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        format!("application/vnd.docker.distribution.manifest.v2+json")
            .parse()
            .unwrap(),
    );

    let client = reqwest::Client::new();

    _ = client
        .delete(format!(
            "{}/v2/{}/manifests/{}",
            registry_address, registry_repo, registry_digest
        ))
        .basic_auth(&registry_user, Some(&registry_password))
        .headers(headers.clone())
        .send()
        .await?;

    //let digest: String = resp.headers().get("Docker-Content-Digest").expect("Something wrong").to_str().unwrap().to_string();

    return Ok(());
}
