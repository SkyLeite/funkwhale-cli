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

pub fn upload(files: Vec<std::path::PathBuf>, library: String, instance: String, token: String, timeout: u64) -> Result<(), Box<std::error::Error>> {
    let now = Utc::now();
    let import_reference = format!("From CLI at {}", now);
    let url = format!("{}/api/v1/uploads/", instance);
    let total_files = &files.len();
    let mut current_file = 0;

    for file in &files {
        let filename = file.file_name().unwrap().to_str().unwrap();

        let form = multipart::Form::new()
            .text("library", library.clone())
            .text("import_reference", import_reference.clone())
            .text("source", format!("upload://{}", filename))
            .file("audio_file", file)?;

        let sp = Spinner::new(Spinners::Moon, "Uploading your files. Please wait.".into());
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout))
            .build()?;

        let _resp = client
            .post(&url)
            .bearer_auth(token.clone())
            .multipart(form)
            .send()?
            .text()?;

        sp.message(format!("Uploaded file {} of {}. Uploading...", &current_file + 1, &total_files));
        sp.stop();
        current_file += 1;
    }

    Ok(())
}
