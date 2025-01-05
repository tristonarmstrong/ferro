use crate::MindGen;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[allow(unused)]
pub static OLLAMA_ENDP: &str = "http://localhost:11434/api/generate";

#[allow(unused)]
pub struct Transporter {
    pub client: Client,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GenRes {
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
struct GenOptions {
    temperature: f32,
    num_predict: u8,
    repeat_last_n: u8,
    top_k: u8,
    top_p: f32,
}

#[derive(Debug, Serialize)]
pub struct MindGen {
    model: String,
    prompt: String,
    stream: bool,
    raw: bool,
    system: String,
    options: GenOptions,
}

impl MindGen {
    #[allow(unused)]
    pub fn new(directions: String, input: String) -> Self {
        Self {
            model: String::from("llama3.1"),
            stream: false,
            raw: false,
            prompt: input,
            system: directions,
            options: GenOptions {
                temperature: 0.1,
                num_predict: 0,
                repeat_last_n: 0,
                top_k: 10,
                top_p: 0.5,
            },
        }
    }
}

#[allow(unused)]
impl Transporter {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    #[allow(unused)]
    pub fn make_request(
        &mut self,
        gen_data: MindGen,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let json_body = serde_json::to_string(&gen_data).unwrap();
        self.client.post(OLLAMA_ENDP).body(json_body).send()
    }
}
