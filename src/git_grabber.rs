use git2::Repository;
use std::{env::current_dir, path::PathBuf};

pub struct GitGrabber {
    pub repo: Option<Repository>,
    dir: PathBuf,
}

impl GitGrabber {
    pub fn new() -> Self {
        GitGrabber {
            repo: None,
            dir: current_dir().unwrap(),
        }
    }

    pub fn get_repo(&mut self) {
        let repo = match Repository::open(self.dir.clone()) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to open: {}", e),
        };

        self.repo = Some(repo);
    }
}
