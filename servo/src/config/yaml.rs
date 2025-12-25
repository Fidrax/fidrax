use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct ServoConfig {
    // pub db: Database,
    pub app: Application,
}

// mongodb://mongodb:mongodb@localhost:27017/?authSource=admin&tls=false&directConnection=true
#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub user: String,
    pub pass: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Application {
    pub name: String,
    pub version: String,
    pub server_host: String,
    pub server_port: u16,
    pub disk_config_path: PathBuf,
    pub vm_config_path: PathBuf,
    pub run_time_config_path: PathBuf,
}

impl Application {
    pub fn get_host_url(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    // pub fn get_store_path(&self) -> PathBuf {
    //     PathBuf::from(&self.store_path)
    // }

    // pub fn get_runtime_dir(&self) -> PathBuf {
    //     PathBuf::from(&self.runtime_dir)
    // }
}

pub async fn load_yaml_config(path: String) -> Result<ServoConfig, Box<dyn std::error::Error>> {
    let yaml_content = fs::read_to_string(path)?;

    let config: ServoConfig = serde_yaml::from_str(&yaml_content)?;

    Ok(config)
}
