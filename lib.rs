#[feature(macro_rules)];
#[feature(managed_boxes)];

#[crate_id = "reddit#0.1"];
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
pub mod comment;
pub mod objects;

mod util;