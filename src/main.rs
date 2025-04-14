mod arg_parser;
mod commit_handler;
mod config_reader;
mod git_grabber;
mod ml_interface;
mod pr_handler;

use core::panic;

use arg_parser::ArgParser;
use commit_handler::CommitHandler;
use config_reader::ConfigReader;
use ml_interface::{MlBody, MlInterface, MlResponse};
use pr_handler::PrHandler;

fn main() {
    let mut ml = MlInterface::new();
    let config = ConfigReader::new();
    let prompt: Option<(String, String)> = match ArgParser::parse() {
        Some(arg_parser::ParsedArg::Commit) => CommitHandler::new(config),
        Some(arg_parser::ParsedArg::PullRequest) => PrHandler::new(config),
        None => None,
    };

    if prompt.is_none() {
        println!("No prompt found!");
        return;
    }

    let (directions, content) = prompt.unwrap();
    if content.len() < 1 {
        println!("There was no content found. Did you stage your files?");
        return;
    }

    let body = MlBody::new(content, directions);
    let res_text = ml.make_request(body).unwrap().text().unwrap();

    let response: Result<MlResponse, _> = serde_json::from_str(&res_text);
    if response.is_err() {
        println!("oop something went wrong: {:?}", response.err());
        return;
    }
    println!("{:?}", response.unwrap().response);
}
