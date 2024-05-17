mod conf;
mod functions;
mod structures;

///// functions attaching
use crate::functions::get_config_params::*;
use crate::functions::native_delete_manifest::*;
use crate::functions::native_get_digest::*;
use crate::functions::native_get_repositories::*;
use crate::functions::native_get_tags_of_images::*;
use crate::functions::portus_delete_image::*;
use crate::functions::portus_get_latest_images::*;
use crate::functions::portus_get_tags_of_images::*;

///// local modules
pub use crate::conf::VALID_TAG_LIST;

///// external crates
use std::env;
//use std::collections::HashMap;

///// test

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config_path: String = String::new();
    let mut i: usize = 0;
    for word in args.iter() {
        if word.as_str().eq("--config") {
            config_path.push_str(args[i + 1].as_str());
        }
        i = i + 1;
    }

    let vec_config: Vec<String> = get_config_params(config_path).unwrap();

    ///// Portus API
    if vec_config[3].eq("portus") {
        let tag_list: Vec<Tag> =
            portus_get_tags_of_images(&vec_config[0], &vec_config[1], &vec_config[2]).unwrap();
        let image_list: Vec<String> = portus_get_latest_images(&tag_list).unwrap();
        println!("LATEST IMAGES");
        for image_id in image_list.iter() {
            println!("{}", &image_id);
        }

        println!("DELETING IMAGES");
        for tag in tag_list.iter() {
            if tag.name.ne("latest") && !image_list.contains(&tag.image_id) {
                println!(
                    "tag_name - {} tag_id - {}, image_id - {}",
                    &tag.name, &tag.id, &tag.image_id
                );
                _ = portus_delete_image(&vec_config[0], &vec_config[1], &vec_config[2], &tag.id);
            }
        }
    }

    ///// docker registry native API
    if vec_config[3].eq("registry") {
        let mut valid_digests: Vec<String> = Vec::new();
        let repo_list: Vec<String> =
            native_get_repositories(&vec_config[0], &vec_config[1], &vec_config[2]).unwrap();
        for repo in repo_list.iter() {
            let tags: Vec<String> =
                native_get_tags_of_images(&vec_config[0], &vec_config[1], &vec_config[2], &repo)
                    .unwrap();
            for tag in tags.iter() {
                if VALID_TAG_LIST.contains(&tag.as_str()) {
                    let digest: String = match native_get_digest(
                        &vec_config[0],
                        &vec_config[1],
                        &vec_config[2],
                        &repo,
                        &tag,
                    ) {
                        Ok(digest) => digest,
                        Err(_) => continue,
                    };
                    valid_digests.push(digest);
                }
            }
            for tag in tags.iter() {
                let digest: String = match native_get_digest(
                    &vec_config[0],
                    &vec_config[1],
                    &vec_config[2],
                    &repo,
                    &tag,
                ) {
                    Ok(digest) => digest,
                    Err(_) => continue,
                };
                if !VALID_TAG_LIST.contains(&tag.as_str()) && !valid_digests.contains(&digest) {
                    _ = native_delete_manifest(
                        &vec_config[0],
                        &vec_config[1],
                        &vec_config[2],
                        &repo,
                        &digest,
                    );
                    println!("repo: {}, tag: {} - deleted", &repo, &tag);
                }
            }
        }
    }
}
