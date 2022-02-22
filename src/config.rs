use std::path::PathBuf;

pub struct SlapConfig {
    pub ip_address: String,
    pub port: String,
    pub title: String
}

impl SlapConfig {
    pub fn new() -> SlapConfig {
        SlapConfig {
            ip_address: "127.0.0.1".to_string(),
            port: "8080".to_string(),
            title: "Placeholder Title".to_string()
        }
    }

    pub fn from_file(path: PathBuf) -> SlapConfig {
        SlapConfig::new() // TODO: Implement reading of a config file
    }
}
