use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServerConfig {
    pub url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthConfig {
    pub token: String,
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        let config_file: PathBuf = match std::env::var("ITREE_CONFIG_HOME") {
            Ok(dir) => PathBuf::from(format!("{dir}/config.toml")),
            Err(_) => {
                let home = std::env::home_dir().unwrap_or_else(|| std::path::PathBuf::from("~"));
                PathBuf::from(format!(
                    "{}/.config/itree/config.toml",
                    home.to_string_lossy()
                ))
            }
        };
        if !config_file.is_file() {
            anyhow::bail!(
                "cannot find configuration file. Run 'itree config --help' for more information."
            )
        }
        Ok(config_file)
    }

    pub fn load(server_url: Option<String>, api_token: Option<String>) -> Result<Self> {
        let mut root_cfg: Config = Self::default();
        // Flags have highest priority
        if server_url.is_some() {
            root_cfg.server = ServerConfig {
                url: server_url.clone().unwrap(),
            }
        }
        if api_token.is_some() {
            root_cfg.auth = AuthConfig {
                token: api_token.clone().unwrap(),
            }
        }
        // If any flags missing try to read from config file
        if server_url.is_none() || api_token.is_none() {
            let path = Self::path()?;
            if path.exists() {
                let contents = fs::read_to_string(&path)
                    .with_context(|| format!("failed to read {}", path.display()))?;
                let toml_cfg: Config =
                    toml::from_str(&contents).with_context(|| "failed to parse config.toml")?;
                if server_url.is_none() {
                    root_cfg.server.url = toml_cfg.server.url;
                }
                if api_token.is_none() {
                    root_cfg.auth.token = toml_cfg.auth.token;
                }
            }
        }
        if root_cfg.server.url.len() == 0 {
            anyhow::bail!("server URL not configured. Run: itree config set url <URL>");
        }
        if root_cfg.auth.token.len() == 0 {
            anyhow::bail!("API token not configured. Run: itree config set token <TOKEN>");
        }
        Ok(root_cfg)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        fs::write(&path, contents)?;
        Ok(())
    }
}
pub fn list(config: Config) -> Result<()> {
    let contents = toml::to_string(&config)?;
    print!("{}", contents);
    Ok(())
}

pub fn get(config: Config, header: String, key: String) -> Result<()> {
    let root: serde_json::Value = serde_json::to_value(&config)?;
    let value = root
        .get(&header)
        .with_context(|| format!("header '{header}' not found"))?
        .get(&key)
        .with_context(|| format!("key '{key}' not found in [{header}]"))?;
    println!("{header}.{key} = {value}");
    Ok(())
}

pub fn set(mut config: Config, header: String, key: String, value: String) -> Result<()> {
    let mut root: serde_json::Value = serde_json::to_value(&config)?;

    let field = root
        .get_mut(&header)
        .with_context(|| format!("header '{header}' not found"))?
        .get_mut(&key)
        .with_context(|| format!("key '{key}' not found in [{header}]"))?;

    *field = serde_json::Value::String(value.clone());
    config = serde_json::from_value(root)?;
    config.save()?;
    println!("{header}.{key} = \"{value}\"");
    Ok(())
}
