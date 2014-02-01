#[feature(macro_rules)];
#[feature(managed_boxes)];

#[crate_id = "reddit#0.1"];
#[comment = "Rust binding to Reddit API"];
#[license = "LGPLv3"];
#[crate_type = "bin"];

extern mod extra;
extern mod http;

// Import macros
mod macros;

pub mod session;
pub mod redditor;
pub mod subreddit;
pub mod post;
pub mod comment;
pub mod objects;

mod util;

fn main() {
    use std::io::println;
    use post::{Top, AllTime};
    use subreddit::{Subreddit, about_subreddit};

    let sub = about_subreddit("programming").unwrap();

    let posts = Subreddit::sorted_posts(Some(sub), Top, AllTime).unwrap();

    for post in posts.iter().take(5) {
        println(post.to_str());
    }
}