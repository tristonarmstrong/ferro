use crate::git_grabber::GitGrabber;

pub struct PrHandler {}
impl PrHandler {
    pub fn new() -> Option<(String, String)> {
        let directions = String::from("
            <Variables>
            change_type: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert
            <PR Title>
            create a short PR title from this diff, with format <change_type>(<scope>): <pr_description>. 
            <PR Description>
            create an additional PR Description in markdown format to describe the changes made in this diff.
            <Response Constraints>
            Only respond with the Pr title and Pr description. 
        ");
        Some((directions, GitGrabber::generate_repo_desc()))
    }
}
