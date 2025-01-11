use crate::git_grabber::GitGrabber;

pub struct PrHandler {}
impl PrHandler {
    pub fn new() -> Option<(String, String)> {
        let dirs = String::from("Pull Request Handler");
        Some((dirs, GitGrabber::generate_repo_desc("", "")))
    }
}
