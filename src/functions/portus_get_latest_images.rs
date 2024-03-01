///// local modules
pub use crate::structures::tag::Tag;

///// external crates
use std::error::Error;

pub fn portus_get_latest_images(tag_list: &Vec<Tag>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut image_list: Vec<String> = Vec::new();
    for tag in tag_list.iter() {
        if tag.name.eq("latest") {
            image_list.push(tag.image_id.clone());
        }
    }
    return Ok(image_list);
}
