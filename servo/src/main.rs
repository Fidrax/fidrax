pub mod api;
pub mod config;
pub mod service;
pub mod errors;

use std::env;

use crate::api::server::start_http_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let default_path = String::from("./configs/config.yaml");
    let path = env::args().nth(1).unwrap_or(default_path);

    let res = config::yaml::load_yaml_config(path).await;
    let cfg = match res {
        Ok(config) => config,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to load configuration",
            ));
        }
    };

    start_http_server(cfg).await?;
    Ok(())
}
