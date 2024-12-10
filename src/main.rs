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
    let mind_gen_text = MindGen::new(format!("input: {}; branch: {}", diff, "dev"));
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
}
