use directories;
use serde::{Deserialize, Serialize};
use toml;

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

pub fn get_config() -> Result<Config, Box<std::error::Error>> {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let mut config_dir = std::path::PathBuf::from(base_dirs.config_dir());
    config_dir.push("funkwhale-cli");

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
