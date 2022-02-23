use std::path::PathBuf;
use crate::card::Card;

pub struct Config {
    pub title: String,
    pub n_cols: usize,
    pub n_rows: usize,
    pub cards: Vec<Card>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            title: "Placeholder Title".to_string(),
            n_cols: 3,
            n_rows: 3,
            cards: Vec::new(),
        }
    }

    pub fn from_file(path: PathBuf) -> Config {
        Config::new() // TODO: Implement reading of a config file
    }
}
