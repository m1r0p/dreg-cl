//////
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: u64,
    pub name: String,
    //#[serde(flatten)]
    pub author: Author,
    pub digest: String,
    pub image_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub size: u64,
    pub scanned: u64,
    //#[serde(default)]
    pub vulnerabilities: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub id: u64,
    pub name: String,
}
