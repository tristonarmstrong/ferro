use core::panic;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::config_reader::ConfigReader;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct MlResponse {
    model: String,
    created_at: String,
    pub response: String,
    done: bool,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64,
}

#[derive(Debug, Serialize)]
struct MlOptions {
    temperature: f32,
    num_predict: u8,
    repeat_last_n: u8,
    top_k: u8,
    top_p: f32,
    num_ctx: u8,
}

#[derive(Debug, Serialize)]
pub struct MlBody {
    model: String,
    prompt: String,
    stream: bool,
    raw: bool,
    system: String,
    options: MlOptions,
}

impl MlBody {
    pub fn new(content: String, directions: String) -> Self {
        let config = ConfigReader::new();
        if config.is_none() {
            panic!("Failed to get ollama endpoint because config file couldnt be read");
        }
        let config = config.unwrap();

        Self {
            model: config.model,
            stream: false,
            raw: false,
            prompt: content,
            system: directions,
            options: MlOptions {
                temperature: 0.1,
                num_predict: 0,
                repeat_last_n: 0,
                top_k: 10,
                top_p: 0.5,
                num_ctx: 0,
            },
        }
    }
}

#[allow(unused)]
pub struct MlInterface {
    pub client: Client,
}

#[allow(unused)]
impl MlInterface {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get_ollama_endpoint(&self) -> String {
        let config = ConfigReader::new();
        if config.is_none() {
            panic!("Failed to get ollama endpoint because config file couldnt be read");
        }
        let config = config.unwrap();
        let ollama_endp = format!("http://{}:{}/api/generate", config.address, config.port);
        ollama_endp
    }

    #[allow(unused)]
    pub fn make_request(&mut self, gen_data: MlBody) -> Result<reqwest::blocking::Response, &str> {
        if gen_data.prompt.len() < 1 {
            panic!("No prompt provided");
        }
        let json_body = serde_json::to_string(&gen_data).unwrap();
        let res = self
            .client
            .post(self.get_ollama_endpoint())
            .body(json_body)
            .send();
        if res.is_err() {
            panic!("Failed to send ollama payload");
        }
        Ok(res.unwrap())
    }
}
