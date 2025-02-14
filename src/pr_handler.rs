use crate::{config_reader::ConfigReader, git_grabber::GitGrabber};

pub struct PrHandler {}
impl PrHandler {
    pub fn new(config: Option<ConfigReader>) -> Option<(String, String)> {
        let config = config.unwrap();
        let types = config
            .pull_request
            .change_type
            .into_iter()
            .fold(String::new(), |acc, x| format!("{acc}, {x}"));
        let instructions = config.pull_request.instructions;
        let constraints = config.pull_request.constraints;
        let directions = String::from(format!("
            <Variables>
            change_type: {types}

            <PR Title>
            create a short PR title from this diff, with format <change_type>(<scope>): <pr_description>. 

            <PR Description>
            {instructions}

            <Response Constraints>
            {constraints}
        "));
        Some((directions, GitGrabber::generate_repo_desc()))
    }
}
