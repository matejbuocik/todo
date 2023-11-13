use std::fs::File;
use std::io::Read;

const CONFIG_FILE: &str = "config.toml";

#[derive(serde_derive::Deserialize)]
pub struct Config {
    pub db_path: String,
}

impl Config {
    pub fn get() -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(CONFIG_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    }
}
