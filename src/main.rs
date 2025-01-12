mod arg_parser;
mod commit_handler;
mod git_grabber;
mod ml_interface;
mod pr_handler;

use arg_parser::ArgParser;
use commit_handler::CommitHandler;
use ml_interface::{MlBody, MlInterface, MlResponse};
use pr_handler::PrHandler;

fn main() {
    let prompt: Option<(String, String)> = match ArgParser::parse() {
        Some(arg_parser::ParsedArg::Commit) => CommitHandler::new(),
        Some(arg_parser::ParsedArg::PullRequest) => PrHandler::new(),
        None => None,
    };

    if prompt.is_none() {
        return;
    }

    let mut ml = MlInterface::new();
    let (directions, content) = prompt.unwrap();
    let body = MlBody::new(content, directions);
    let res_text = ml.make_request(body).unwrap().text().unwrap();

    let response: Result<MlResponse, _> = serde_json::from_str(&res_text);
    if response.is_err() {
        panic!("oop something went wrong: {:?}", response.err());
    }
    println!("{:?}", response.unwrap().response);
}
