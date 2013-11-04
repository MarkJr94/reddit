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

mod util;

fn main() {
    use redditor::{about_redditor};
    use extra::json::{Encoder};
    use extra::serialize::Encodable;
    use std::rt::io;

    let pd = about_redditor("pudukheba");

    match pd {
        Err(msg) => fail!(msg),
        Ok(pudu) => {
            println(pudu.to_str());
            let out = @mut io::stdout() as @mut io::Writer;
            let mut enc = Encoder(out);

            pudu.encode(&mut enc);
            println("");
        }
    }
}