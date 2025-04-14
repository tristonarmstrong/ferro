use core::panic;
use std::{
    env,
    fs::{self, File},
    io::Read,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommitConfig {
    pub change_type: Vec<String>,
    pub instructions: String,
    pub constraints: String,
}

#[derive(Debug, Deserialize)]
pub struct PrConfig {
    pub change_type: Vec<String>,
    pub instructions: String,
    pub constraints: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigReader {
    pub model: String,
    pub address: String,
    pub port: String,
    pub commit: CommitConfig,
    pub pull_request: PrConfig,
}

impl ConfigReader {
    pub fn new() -> Option<Self> {
        let home = Self::get_home();
        if home.is_none() {
            panic!("Failed to get home directory");
        }

        let config_path = format!("{}/.config/ferro/config.json", home.clone().unwrap());

        let exists = fs::exists(config_path.clone()).unwrap();

        if !exists {
            let dir_exists =
                fs::exists(format!("{}/.config/ferro", home.clone().unwrap())).unwrap();
            if !dir_exists {
                let create_dir_res = fs::create_dir(format!("{}/.config/ferro", home.unwrap()));
                if create_dir_res.is_err() {
                    panic!("Failed to create ferro directory");
                }
            }
            let res = fs::copy("./resources/config.json", config_path.clone());
            if res.is_err() {
                panic!(
                    "{}: {}",
                    "Failed to create default config file",
                    res.err().unwrap()
                );
            }
        }

        let file = Self::read_file(config_path);
        let json_version = Self::parse_json(file.unwrap());
        Some(json_version)
    }

    fn parse_json(file: String) -> ConfigReader {
        let res = serde_json::from_str::<ConfigReader>(file.as_str());
        if res.is_err() {
            panic!("Failed to parse JSON from file: {}", res.unwrap_err());
        }

        res.unwrap()
    }

    // fn create_commit() -> CommitConfig {
    //     todo!()
    // }

    // fn create_pr() -> PrConfig {
    //     todo!()
    // }

    fn get_home() -> Option<String> {
        let home = env::home_dir(); //TODO: change to support windows
        if home.is_none() {
            return None;
        }
        let home = home.unwrap();
        Some(home.display().to_string())
    }

    // TODO: probably need return result
    fn read_file(config_path: String) -> Option<String> {
        let file = File::open(config_path);
        if file.is_err() {
            return None;
        }

        let mut file_buf = String::new();
        let _ = file.unwrap().read_to_string(&mut file_buf);

        Some(file_buf)
    }
}
