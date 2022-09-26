use lazy_static::lazy_static;
use serde_derive::{Serialize, Deserialize};
use std::fs;

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut config = dirs::config_dir().expect("Could not get the config directory");
        config.push("git-server");
        config.push("git-server.toml");
        if config.exists() {
           let config:Config = toml::from_str(&String::from_utf8_lossy(&fs::read(config.as_path()).expect("Failed to read the config file"))).expect("Could not parse the toml in the config file");
           config
        }
        else {
            let config_struct = Config {
                name: "Example User".to_string(),
                description: "This is billy and he loves his [website](https://example.com)!!!".to_string()
            };
            config.pop();
            let _ = fs::create_dir_all(config.clone());
            config.push("git-server.toml");
            fs::write(config, toml::to_string(&config_struct).expect("Failed to stringify config")).expect("Failed to set the config content");
            config_struct
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub description: String,
}
