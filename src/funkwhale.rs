use chrono::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::multipart;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct LibraryResponse {
    pub count: i32,
    pub results: Vec<Library>,
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

#[derive(Deserialize, Clone, Debug)]
pub struct TokenResponse {
    pub token: String,
}

pub fn get_token(
    instance: &str,
    username: &str,
    password: &str,
) -> Result<String, Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let mut body = std::collections::HashMap::new();
    body.insert("username", &username);
    body.insert("password", &password);

    let resp: TokenResponse = client
        .post(&format!("{}/api/v1/token/", instance))
        .json(&body)
        .send()?
        .json()?;

    Ok(resp.token)
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

pub fn upload(
    files: Vec<std::path::PathBuf>,
    library: String,
    instance: String,
    token: String,
    timeout: u64,
) -> Result<(), Box<std::error::Error>> {
    let now = Utc::now();
    let import_reference = format!("From CLI at {}", now);
    let url = format!("{}/api/v1/uploads/", instance);
    let total_files = files.len();

    let bar = ProgressBar::new(total_files as u64);
    println!("Uploading files. Please wait.");

    bar.set_style(
        ProgressStyle::default_bar()
            .template(" {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout))
        .build()?;

    for file in &files {
        let filename = file.file_name().unwrap().to_str().unwrap();

        bar.set_message(&format!("Uploading {}", filename));

        let form = multipart::Form::new()
            .text("library", library.clone())
            .text("import_reference", import_reference.clone())
            .text("source", format!("upload://{}", filename))
            .file("audio_file", file)?;

        let _resp = client
            .post(&url)
            .bearer_auth(token.clone())
            .multipart(form)
            .send()?;

        bar.inc(1);
    }
    bar.set_message("Finished!");
    bar.finish();

    Ok(())
}
