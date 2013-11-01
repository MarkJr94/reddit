#[feature(macro_rules)];
#[feature(globs)];

#[link(name="reddit",
    vers="0.1",
    url="https://github.com/MarkJr94/reddit")];

#[comment = "Rust binding to Reddit API"];
#[license = "LGPLv3"];

extern mod extra;
extern mod http;

// Import macros
mod macros;

pub mod session;
pub mod redditor;

mod util;



fn main() {

}