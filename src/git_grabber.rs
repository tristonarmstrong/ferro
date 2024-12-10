use std::{fs::write, io::Write, process::Command, str::from_utf8};

// this is a comment test here
pub struct GitGrabber {}

impl GitGrabber {
    pub fn new() -> Self {
        GitGrabber {}
    }

    pub fn get_diff(&self) -> String {
        // just to print
        let staged_files_output = Command::new("git")
            .args(["diff", "--staged", "--stat"])
            .output()
            .expect("Failed to get diff");
        let _ = std::io::stdout().write_all(&staged_files_output.stdout);

        // actual output
        let output = Command::new("git")
            .args(["diff", "--staged", "--", ".", "':(exclude)*lock*'"])
            .output()
            .expect("Failed to execute process");

        let b = from_utf8(&output.stdout).unwrap();
        String::from(b)
    }
}
