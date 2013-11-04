pub use self::rd::Redditor;

use http::client::RequestWriter;
use http::method::{Get};
use std::str::from_utf8;
use std::rt::io::{Reader};
use extra::url;
use extra::json::{from_str};

use util::json::{JsonLike, FromJson};
use util::REDDIT;

json_struct2!(rd, Redditor,
    "id" -> id: ~str,
    "name" -> name: ~str,
    "link_karma" -> link_karma: int,
    "comment_karma" -> comment_karma: int,
    "created_utc" -> created_utc: f64,
    "is_gold" -> is_gold: bool,
    "is_mod" -> is_mod: bool,
    "has_mail" -> has_mail: Option<bool>,
    "has_mod_mail" -> has_mod_mail: Option<bool>)

pub fn about_redditor(username: &str) -> Result<Redditor, ~str> {
    let url = url::from_str(format!("{0}user/{1}/about.json", REDDIT, username)).unwrap();

    let req = ~RequestWriter::new(Get, url);

    struct Response {
        data: Redditor
    }

    json_struct!(Response,
        "data" -> data: Redditor)

    match req.read_response() {
        Ok(mut resp) => {
            let body = from_utf8(resp.read_to_end());

            match from_str(body) {
                Err(jerror) => Err(jerror.to_str()),
                Ok(json) => {
                    Debug!(json.value(&~"data").unwrap().to_str());

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
    use super::{about_redditor};

    #[test]
    fn test_about() {
        let redditor = about_redditor("Axelior").unwrap();

        Debug!("{}", redditor.to_str());
    }
}