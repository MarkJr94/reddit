use self::json::JsonLike;

use std::rt::io::{Reader, Writer};

use extra::json::Json;
use extra::url::{Url};

use http::client::{RequestWriter, ResponseReader};
use http::method::{Get, Post};
use http::headers::content_type::MediaType;
use std::rt::io::net::tcp::TcpStream;

use session::{Session};

pub mod json;

pub static REDDIT : &'static str = "http://www.reddit.com/";
pub static UASTR : &'static str = "github.com/MarkJr94/reddit";

#[deriving(ToStr, Eq, Encodable, Decodable)]
pub enum Vote {
    Upvote,
    Downvote,
    RemoveVote
}

impl Vote {
    fn as_str(&self) -> ~str {
        match *self {
            Upvote => ~"1",
            Downvote => ~"-1",
            RemoveVote => ~"0"
        }
    }
}

pub fn check_errors(json: &Json) -> Result<(), ~str> {
    let err_list = json.value(&~"json").value(&~"errors").as_list().unwrap();

    if err_list.len() != 0 {
        Err(err_list.to_str())
    } else {
        Ok(())
    }
}

pub fn get_secrets() -> (~str, ~str) {
    use std::rt::io::{fs, Open, Reader, Read};
    use std::path::Path;
    use std::str::from_utf8;

    let mut reader = fs::File::open_mode(&Path::new("secrets.txt"), Open, Read)
        .expect("Secret file couldn't be opened");

    let s: ~str = from_utf8(reader.read_to_end());

    let v: ~[~str] = s.split_iter(' ').map(|s| s.trim().to_owned()).collect();

    (v[0].clone(), v[1])
}

pub fn get_resp(url: Url, post_data: Option<&[u8]>, s: Option<&Session>)
-> Result<ResponseReader<TcpStream>, ~str> {
    let mut req = if post_data.is_some() {
        ~RequestWriter::new(Post, url)
    } else {
        ~RequestWriter::new(Get, url)
    };

    req.headers.user_agent = Some(UASTR.to_owned());

    match s {
        None => {}
        Some(sess) => {
            req.headers.extensions.insert(~"Cookie", sess.cookie.clone());
        }
    }

    match post_data {
        None => {}
        Some(data) => {
            req.headers.content_length = Some(data.len());
            req.headers.content_type = Some(MediaType(~"application",
                ~"x-www-form-urlencoded"
                , ~[]));

            req.write(data);
        }
    }

    match req.read_response() {
        Ok(resp) => {
            Ok(resp)
        }
        Err(_) => {
            Err(~"Bad request")
        }
    }
}
