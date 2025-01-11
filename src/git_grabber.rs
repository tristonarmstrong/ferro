use std::{
    fs::write,
    io::Write,
    process::{Command, Output},
    str::from_utf8,
};

// this is a comment test here
pub struct GitGrabber {}

impl GitGrabber {
    pub fn new() -> Self {
        GitGrabber {}
    }

    pub fn get_current_branch() -> String {
        let branch = Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .expect("Failed to get branch");

        let b = from_utf8(&branch.stdout).unwrap();
        String::from(b)
    }

    pub fn get_diff() -> String {
        // just to print
        let staged_files_output = Command::new("git")
            .args(["diff", "--staged"])
            .output()
            .expect("Failed to get diff");

        let b = from_utf8(&staged_files_output.stdout).unwrap();
        String::from(b)
    }

    pub fn generate_repo_desc(origin_branch: &str, local_branch: &str) -> String {
        let output = Command::new("git")
            .args([
                "rev-list",
                "--left-right",
                "--pretty=oneline",
                &format!(
                    "{}...{}",
                    origin_branch.to_string(),
                    local_branch.to_string()
                ),
            ])
            .output()
            .expect("Failed to get repo details");

        let b = from_utf8(&output.stdout).unwrap();
        String::from(b)
    }
}
