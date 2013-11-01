use http::client::RequestWriter;
use http::method::{Get, Post};
use http::headers::content_type::MediaType;
use std::str::from_utf8;
use std::rt::io::{Reader,Writer};
use extra::url;
use extra::json::{Json, from_str, Decoder};
use extra::serialize::Decodable;

use util::json::{JsonLike, FromJson};
use util::REDDIT;

#[deriving(ToStr, Clone, Encodable, Decodable, Eq)]
pub struct Redditor {
    id: ~str,
    name: ~str,
    link_karma: int,
    comment_karma: int,
    created_utc: f32,
    is_gold: bool,
    is_mod: bool,
    has_mail: Option<bool>,
    has_mod_mail: Option<bool>
}

json_struct!(Redditor,
    "id" -> id: ~str,
    "name" -> name: ~str,
    "link_karma" -> link_karma: int,
    "comment_karma" -> comment_karma: int,
    "created_utc" -> created_utc: f32,
    "is_gold" -> is_gold: bool,
    "is_mod" -> is_mod: bool,
    "has_mail" -> has_mail: Option<bool>,
    "has_mod_mail" -> has_mod_mail: Option<bool>)

pub fn about_redditor(username: &str) -> Result<Redditor, ~str> {
    let url = url::from_str(format!("{0}user/{1}/about.json", REDDIT, username)).unwrap();

    let mut req = ~RequestWriter::new(Get, url);

    struct Response {
        data: Redditor
    }

    json_struct!(Response,
        "data" -> data: Redditor)

    match req.read_response() {
        Ok(mut resp) => {
            let body = from_utf8(resp.read_to_end());

            Debug!(body);

            match from_str(body) {
                Err(jerror) => Err(jerror.to_str()),
                Ok(json) => {
                    let dec = Decoder(json.clone());

                    let res_data: Result<Response, ::util::json::ValueError> = FromJson::from_json(&json);

                    match res_data {
                        Ok(r) => Ok(r.data),
                        Err(jerror) => Err(jerror.to_str()),
                    }
                }
            }
        }
        Err(_) => Err(~"RequestWriter")
    }
}

#[cfg(test)]
mod test {
    use super::{Redditor, about_redditor};

    fn get_user_pass() -> (~str, ~str) {
        use std::rt::io::{file, Open, Read};
        use std::path::Path;
        use std::str::from_utf8;

        let mut stream = file::open(&Path::new("secrets.txt"), Open, Read)
            .expect("Secret file couldn't be opened");
        let s = from_utf8(stream.read_to_end());

        let v: ~[~str] = s.split_iter(' ').map(|s| s.trim().to_owned()).collect();

        (v[0].clone(), v[1])

    }

    #[test]
    fn test_about() {
        let redditor = about_redditor("Axelior").unwrap();

//         Debug!("{}", redditor.to_str());
    }
}