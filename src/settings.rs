use std::{io::BufWriter, path::PathBuf};

use config::Config;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub port: u16,
    pub address: String,
    pub token: String,
    pub repo: String,
    pub owner: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            port: 8080,
            token: "<github-token>".into(),
            address: "127.0.0.1".into(),
            repo: "<repository>".into(),
            owner: "<repository-owner>".into(),
        }
    }
}

pub fn get() -> Configuration {
    let config_path = PathBuf::from("./config.json");

    if !&config_path.exists() {
        let file = File::create(&config_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &Configuration::default()).unwrap();
        writer.flush().unwrap();
    }

    let settings = Config::builder()
        .add_source(config::File::from(config_path))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    return settings.try_deserialize::<Configuration>().unwrap();
}
