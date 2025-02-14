use crate::config_reader::ConfigReader;
use crate::git_grabber::GitGrabber;

pub struct CommitHandler {}

impl CommitHandler {
    pub fn new(config: Option<ConfigReader>) -> Option<(String, String)> {
        let config = config.unwrap();
        let types = config
            .commit
            .change_type
            .into_iter()
            .fold(String::new(), |acc, x| format!("{acc}, {x}"));
        let instructions = config.commit.instructions;
        let constraints = config.commit.constraints;

        //  check if theres an existing config file somewhere
        let dirs = String::from(format!(
            "
            <Variables>
            change_type: {types}
            <Commit Instructions>
            {instructions}
            <Response Constraints>
            {constraints}
        "
        ));
        let prompt = GitGrabber::get_diff();
        Some((dirs, prompt))
    }
}
