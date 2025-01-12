use crate::git_grabber::GitGrabber;

pub struct CommitHandler {}

impl CommitHandler {
    pub fn new() -> Option<(String, String)> {
        let dirs = String::from(format!("
            <Variables>
            change_type: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert
            <Commit Instructions>
            create a short commit message from this entire diff, with format <change_type>(<scope>): <commit_message>. 
            the commit message should vaguely describe the changes present unless a small change is made that can be 
            precisely described withtin a short commit message
            <Response Constraints>
            Only respond with the commit message.
        "));
        let prompt = GitGrabber::get_diff();
        Some((dirs, prompt))
    }
}
