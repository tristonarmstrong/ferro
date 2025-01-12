use crate::git_grabber::GitGrabber;

pub struct PrHandler {}
impl PrHandler {
    pub fn new() -> Option<(String, String)> {
        let directions = String::from("
            <Variables>
            change_type: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert

            <PR Title>
            create a short PR title from this diff, with format <change_type>(<scope>): <pr_description>. 

            <PR Description>
            create a PR Description to describe the changes made in this diff using the commit messages for the body content.

            follow this format:
            ## What?
            [what_description]
            #[ticket_number]
            ## Why?
            [why_description]
            ## How?
            [how_description]
            ## Testing?
            [testing_description]
            ## Screenshots (optional)
            [screenshots]
            ## Anything Else?
            [leftover_details]

            include signature at the end stating 'this is an ai generated pull request description'.

            <Response Constraints>
            Only respond with the Pr title and Pr description. 
            Use Emojis in description body only.
        ");
        Some((directions, GitGrabber::generate_repo_desc()))
    }
}
