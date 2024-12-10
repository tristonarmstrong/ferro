use crate::MindGen;
use reqwest::blocking::Client;

#[allow(unused)]
pub static OLLAMA_ENDP: &str = "http://localhost:11434/api/generate";

#[allow(unused)]
pub struct Transporter {
    pub client: Client,
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
