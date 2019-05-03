use directories;
use serde::{Deserialize, Serialize};
use toml;

#[path = "./funkwhale.rs"]
mod funkwhale;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub instance_url: String,
}

pub fn prompt_config() -> String {
    println!("Could not find a config file. Let's create one now.");
    let instance_url: String = dialoguer::Input::new()
        .with_prompt("Instance url")
        .interact()
        .unwrap();

    let config = Config { instance_url };

    let config_str = toml::to_string(&config).unwrap();
    return config_str;
}

fn prompt_token(instance_url: &str) -> Result<String, Box<std::error::Error>> {
    println!("Token file not found. Please provide your credentials.");
    println!(">> Funkwhale-cli does *NOT* store your password.");
    let username: String = dialoguer::Input::new()
        .with_prompt("Username")
        .interact()
        .unwrap();

    let password: String = dialoguer::PasswordInput::new()
        .with_prompt("Password")
        .interact()
        .unwrap();

    let token: String = funkwhale::get_token(&instance_url, &username, &password).unwrap();

    Ok(token)
}

fn get_config_dir() -> Result<std::path::PathBuf, Box<std::error::Error>> {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let mut config_dir = std::path::PathBuf::from(base_dirs.config_dir());
    config_dir.push("funkwhale-cli");

    Ok(config_dir)
}

pub fn get_token(instance_url: &str) -> Result<String, Box<std::error::Error>> {
    let mut token_file = get_config_dir().unwrap();
    token_file.push("token");

    if !&token_file.exists() {
        let token = prompt_token(instance_url).unwrap();
        std::fs::write(&token_file, &token)?;
        Ok(token)
    } else {
        let token = std::fs::read_to_string(token_file).unwrap();
        Ok(token)
    }
}

pub fn get_config() -> Result<Config, Box<std::error::Error>> {
    let config_dir = get_config_dir().unwrap();

    if !config_dir.clone().exists() {
        std::fs::create_dir(&config_dir).unwrap();
    }

    let mut config_file = config_dir.clone();
    config_file.push("config.toml");

    let config_string: String;
    if !config_file.clone().exists() {
        config_string = prompt_config();
        std::fs::write(config_file, &config_string).unwrap();
    } else {
        config_string = std::fs::read_to_string(&config_file).unwrap();
    }

    let config: Config = toml::from_str(&config_string).unwrap();
    Ok(config)
}
