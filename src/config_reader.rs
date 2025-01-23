use core::panic;
use std::{env, fs::File, io::Read};

#[derive(Debug)]
pub struct CommitConfig {
    change_type: Vec<String>,
    instructions: String,
    constraints: String,
}

#[derive(Debug)]
pub struct PrConfig {
    change_type: Vec<String>,
    instructions: String,
    constraints: String,
}

#[derive(Debug)]
pub struct ConfigReader {
    commit: CommitConfig,
    pull_request: PrConfig,
}

impl ConfigReader {
    pub fn new() -> Option<Self> {
        let home = Self::get_home().unwrap();
        let config_path = format!("{}/.config/ferro/config.json", home);
        let file = Self::read_file(config_path);
        Some(Self {
            commit: Self::create_commit(),
            pull_request: Self::create_pr(),
        })
    }

    fn parse_json() -> String {
        todo!()
    }

    fn create_commit() -> CommitConfig {
        todo!()
    }

    fn create_pr() -> PrConfig {
        todo!()
    }

    fn get_home() -> Option<String> {
        let home = env::home_dir();
        if home.is_none() {
            panic!("No Home dir to read config from");
        }
        let home = home.unwrap();
        Some(home.display().to_string())
    }

    fn read_file(config_path: String) -> Option<String> {
        let file = File::open(config_path);
        if file.is_err() {
            println!("[ ERROR ]: {}", file.unwrap_err());
            return None;
        }

        let mut file_buf = String::new();
        let _ = file.unwrap().read_to_string(&mut file_buf);

        Some(file_buf)
    }
}
