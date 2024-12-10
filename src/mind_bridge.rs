    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    #[allow(unused)]
    pub struct GenRes {
        model: String,
        created_at: String,
        response: String,
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
        pub fn new(input: &str) -> Self {
             Self {
            model: String::from("llama3.1"),
            stream: false,
            raw: false,
            prompt: String::from(input),
            system: String::from("You are a commit message generator. You will generate commit messages following this format Task(<task #>): <commit message> making sure to never go over 90 characters"),
            options: GenOptions { 
                temperature: 0.5,
                num_predict: 90
            }
        }
    }
}
