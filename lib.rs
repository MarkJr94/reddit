#[feature(macro_rules)];
// #[feature(globs)];
#[feature(managed_boxes)];

#[link(name="reddit",
    vers="0.1",
    url="https://github.com/MarkJr94/reddit")];

#[comment = "Rust binding to Reddit API"];
#[license = "LGPLv3"];
#[crate_type = "lib"];

extern mod extra;
extern mod http;

// Import macros
mod macros;

pub mod session;
pub mod redditor;
pub mod subreddit;
pub mod post;
pub mod objects;

mod util;

fn main() {
    use post::{Top, AllTime};
    use subreddit::{Subreddit, about_subreddit};

    let sub = about_subreddit("programming").unwrap();

    let posts = Subreddit::sorted_posts(Some(sub), Top, AllTime).unwrap();

    println(posts.to_str());
}