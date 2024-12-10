    use serde::{Deserialize, Serialize};

    // this is a test comment
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
        top_p: f32
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
        pub fn new(input: String) -> Self {
             Self {
            model: String::from("llama3.1"),
            stream: false,
            raw: false,
            prompt: input,
            system: String::from("create a short commit message from this diff, with format Task(<branch_name>): <commit_message>. only respond with the commit message"),
            options: GenOptions { 
                temperature: 0.1,
                num_predict: 0,
                repeat_last_n: 0,
                top_k: 10,
                top_p: 0.5
            }
        }
    }
}
