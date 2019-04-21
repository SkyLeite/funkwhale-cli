extern crate reqwest;
extern crate dialoguer;
extern crate spinners;
extern crate chrono;

use reqwest::multipart;
use serde::{Deserialize};
use spinners::{Spinner, Spinners};
use chrono::prelude::*;

#[derive(Deserialize, Clone, Debug)]
pub struct LibraryResponse {
    pub count: i32,
    pub results: Vec<Library>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Library {
    pub uuid: String,
    pub fid: String,
    pub name: String,
    pub privacy_level: String,
    pub uploads_count: i32,
    pub creation_date: String,
}

pub fn get_libraries(instance: &str, token: &str) -> Result<Vec<Library>, Box<std::error::Error>> {
    let client = reqwest::Client::new();

    let resp: LibraryResponse = client
        .get(&format!("{}/api/v1/libraries", instance))
        .bearer_auth(token)
        .send()?
        .json()?;

    Ok(resp.results)
}

pub fn upload(files: Vec<std::path::PathBuf>, library: String, instance: String, token: String) -> Result<(), Box<std::error::Error>> {
    let filename = &files[0].file_name().unwrap().to_str().unwrap();
    let now = Utc::now();

    let form = multipart::Form::new()
        .text("library", library)
        .text("import_reference", format!("From CLI at {}", now))
        .text("source", format!("upload://{}", filename))
        .file("audio_file", &files[0])?;

    let sp = Spinner::new(Spinners::Moon, "Uploading your files. Please wait.".into());
    let client = reqwest::Client::new();
    let _resp = client
        .post(&format!("{}/api/v1/uploads/", instance))
        .bearer_auth(token)
        .multipart(form)
        .send()?
        .text()?;

    sp.message("Uploaded!".into());
    sp.stop();

    Ok(())
}
