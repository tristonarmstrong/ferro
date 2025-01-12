use core::panic;
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
        String::from(b.strip_suffix("\n").unwrap())
    }

    pub fn get_diff() -> String {
        // just to print
        let output = Command::new("git")
            .args([
                "diff",
                "--staged",
                "--ignore-all-space",
                "--ignore-blank-lines",
                "--ignore-cr-at-eol",
                "--minimal",
                "--no-prefix",
                "--no-renames",
                "--word-diff",
                "--inter-hunk-context=0",
            ])
            .output()
            .expect("Failed to get diff");

        if !output.status.success() {
            panic!("Did you forget to stage your files?");
        }

        let b = from_utf8(&output.stdout).unwrap();
        String::from(b)
    }

    pub fn generate_repo_desc() -> String {
        let curr_branch = GitGrabber::get_current_branch();
        let output = Command::new("git")
            .args(["reflog", "show", &curr_branch])
            .output()
            .expect("Failed to get repo details");

        let b = from_utf8(&output.stdout).unwrap();
        String::from(b)
    }
}
