// mod client;

use config::ConfigError;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::Arc;

lazy_static! {
    pub(crate) static ref CONFIG: Arc<Config> = Arc::new(Config::from_env().unwrap());
}

pub fn default_num_want() -> u16 {
    50
}

// pub fn accept_client_list() -> Vec<client::Client> {
//     vec![]
// }
//

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: String,
    pub redis_uri: String,
    pub database_url: String,
    pub backend_announce_addr: String,
}

impl Config {
    fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
