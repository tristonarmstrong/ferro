use std::env::args;

pub struct ArgParser {}

#[derive(Debug)]
pub enum ParsedArg {
    Commit,
    PullRequest,
}

impl ArgParser {
    pub fn parse() -> Option<ParsedArg> {
        let arg = args().nth(1);
        if arg.is_none() {
            // <-- interactive mode will go here
            println!("...(future) interactive mode not implimented... exiting");
            return None;
        }
        let arg = arg.unwrap();
        match arg.as_str() {
            "-c" => Some(ParsedArg::Commit),
            "-p" => Some(ParsedArg::PullRequest),
            "-h" => {
                println!("Available Commands: -c [commit] -p [pull request] -h [help]");
                None
            }
            _ => None,
        }
    }
}
