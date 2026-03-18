use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub auth: Option<AuthConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub token: String,
    pub api_url: String,
}

fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Impossible de trouver le répertoire home")
        .join(".plania")
}

fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn load() -> Config {
    let path = config_path();
    if !path.exists() {
        return Config::default();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    toml::from_str(&content).unwrap_or_default()
}

pub fn save(config: &Config) -> anyhow::Result<()> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;
    let content = toml::to_string_pretty(config)?;
    fs::write(config_path(), content)?;
    Ok(())
}

pub fn get_auth() -> Option<AuthConfig> {
    load().auth
}

pub fn clear_auth() {
    let mut config = load();
    config.auth = None;
    let _ = save(&config);
}
