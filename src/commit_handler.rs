use crate::git_grabber::GitGrabber;

pub struct CommitHandler {}

impl CommitHandler {
    pub fn new() -> Option<(String, String)> {
        let dirs = String::from("create a short commit message from this diff, with format Task(<branch_name>): <commit_message>. only respond with the commit message");
        Some((dirs, GitGrabber::get_diff()))
    }
}
