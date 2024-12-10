mod git_grabber;
mod mind_bridge;
mod transporter;

// use core::panic;
use git_grabber::GitGrabber;
use mind_bridge::*;
// use transporter::Transporter;

fn main() {
    // let a = MindGen::new("Some random commit message");
    // let mut transporter = Transporter::new();

    let mut gg = GitGrabber::new();
    gg.get_repo();

    gg.repo.unwrap().revwalk().unwrap().for_each(|x| {
        if x.is_ok() {
            println!("{:?}", x.unwrap())
        } else {
            println!("No rev")
        }
    });

    return ();

    // let res_text = transporter.make_request(a).unwrap().text().unwrap();
    // let response: Result<GenRes, _> = serde_json::from_str(&res_text);

    // if response.is_err() {
    //     panic!("oop something went wrong: {:?}", response.err());
    // }

    // println!("{:#?}", response.unwrap());
}
