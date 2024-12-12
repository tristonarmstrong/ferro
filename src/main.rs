mod git_grabber;
mod mind_bridge;
mod transporter;

use core::panic;
use git_grabber::GitGrabber;
use mind_bridge::*;
use transporter::Transporter;

fn main() {
    let mut transporter = Transporter::new();

    let gg = GitGrabber::new();
    let diff = gg.get_diff();
    let commits_msg = String::from("create a short commit message from this diff, with format Task(<branch_name>): <commit_message>. only respond with the commit message");
    let mind_gen_text = MindGen::new(commits_msg, format!("input: {}; branch: {}", diff, "dev"));
    let res_text = transporter
        .make_request(mind_gen_text)
        .unwrap()
        .text()
        .unwrap();
    let response: Result<GenRes, _> = serde_json::from_str(&res_text);
    if response.is_err() {
        panic!("oop something went wrong: {:?}", response.err());
    }
    println!("{:#?}", response.unwrap().response);

    let z = gg.generate_repo_desc("master", "dev");
    let pr_msg = String::from(
        "create a pull request description in markdown from the following revlog output. Only respond with pr description, nothing else.",
    );
    let mind_gen_repo = MindGen::new(pr_msg, z);
    let repo_text = transporter
        .make_request(mind_gen_repo)
        .unwrap()
        .text()
        .unwrap();
    let repo_response: Result<GenRes, _> = serde_json::from_str(&repo_text);
    if repo_response.is_err() {
        panic!("oop something went wrong: {:?}", repo_response.err());
    }
    println!("{:#?}", repo_response.unwrap().response);
}
