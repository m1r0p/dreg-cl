///// local modules

///// external crates
use reqwest::header::{HeaderMap, ACCEPT};

#[tokio::main]
pub async fn native_get_digest(
    registry_address: &String,
    registry_user: &String,
    registry_password: &String,
    registry_repo: &String,
    registry_tag: &String,
) -> Result<String, String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        format!("application/vnd.docker.distribution.manifest.v2+json")
            .parse()
            .unwrap(),
    );

    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "{}/v2/{}/manifests/{}",
            registry_address, registry_repo, registry_tag
        ))
        .basic_auth(&registry_user, Some(&registry_password))
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    match &resp.status().as_u16() {
        200 => {
            let digest: String = resp
                .headers()
                .get("Docker-Content-Digest")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            return Ok(digest);
        }
        _ => return Err(String::from("Digest not found")),
    }
}
