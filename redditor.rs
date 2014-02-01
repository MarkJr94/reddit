use http::client::RequestWriter;
use http::method::{Get};
use std::str::from_utf8_owned;

use extra::url;
use extra::json;
use extra::serialize::Decodable;

use comment::Comment;
use post::{Post, PopularitySort, AgeSort};
use post::{New, Rising, Hot, DefaultAge, DefaultPop};
use util::json::{JsonLike, FromJson};
use util::{REDDIT, get_resp};

#[deriving(ToStr, Clone, Encodable, Decodable, Eq)]
pub struct Redditor {
    id: ~str,
    name: ~str,
    link_karma: int,
    comment_karma: int,
    created_utc: f64,
    is_gold: bool,
    is_mod: bool,
    has_mail: Option<bool>,
    has_mod_mail: Option<bool>
}

from_json!(Redditor,
    id,
    name,
    link_karma,
    comment_karma,
    created_utc,
    is_gold,
    is_mod,
    has_mail,
    has_mod_mail)

// #[deriving(Encodable,Decodable)]
// struct inner {
//     data: Option<Comment>
// }
//
// #[deriving(Encodable, Decodable)]
// struct data {
//     children: ~[inner]
// }
//
// #[deriving(Encodable,Decodable)]
// struct Response {
//     data: data
// }

impl Redditor {
    pub fn comments(&self) -> Result<~[Comment], ~str> {
        self.comments_sorted(New, DefaultAge)
    }

    pub fn comments_sorted(&self, pop: PopularitySort, age: AgeSort) -> Result<~[Comment], ~str> {
        if age != DefaultAge {
            match pop {
                New | Rising | Hot => {
                    return Err(format!("Cannot sort {0} by {1}", pop.to_str(), age.to_str()));
                }
                _ => ()
            }
        }

        let mut raw_url = format!("http://www.reddit.com/user/{}/comments/", self.name);

        if pop != DefaultPop {
            if pop == New || pop == Rising {
                raw_url = format!("{}new.json?sort={}", raw_url, pop.as_str());
            } else {
                raw_url = format!("{}{}.json?sort={}", raw_url, pop.as_str(), pop.as_str())
            }
        } else {
            raw_url = format!("{}.json", raw_url);
        }

        if age != DefaultAge {
            if pop != DefaultPop {
                raw_url = format!("{}&t={}", raw_url, age.as_str());
            } else {
                raw_url = format!("{}?t={}", raw_url, age.as_str())
            }
        }

        Debug!("Fetching sorted posts at url: {}", raw_url);

        let url = url::from_str(raw_url).unwrap();

        struct inner {
            data: Comment
        }

        from_json!(inner, data)

        struct data {
            children: ~[inner]
        }

        from_json!(data, children)

        struct Response {
            data: data
        }

        from_json!(Response, data)

        get_resp(url, None, None).or_else(|e| { Err(e) }).and_then(|mut resp| {
            let bytes = resp.read_to_end();

            let body = from_utf8_owned(bytes).expect("Non-UTF8 response");

//             println!("{}", body);

            json::from_str(body).or_else(|e| { Err(e.to_str()) } ).and_then(|json| {
//                 Debug!("{}", json.to_str());
                let res_data: Response = FromJson::from_json(&json).unwrap();
                Debug!("response.data.children.len() = {:u}", res_data.data.children.len());

                Ok(res_data.data.children.move_iter().map(|c| c.data).collect())
            })
        })
    }
}

pub fn about_redditor(username: &str) -> Result<Redditor, ~str> {
    let url = url::from_str(format!("{0}user/{1}/about.json", REDDIT, username)).unwrap();

    let req = RequestWriter::new(Get, url);

    struct Response {
        data: Redditor
    }

    json_struct!(Response,
        "data" -> data: Redditor)

    match req.read_response() {
        Ok(mut resp) => {
            let body = from_utf8_owned(resp.read_to_end()).expect("Non-UTF8 response");

            match json::from_str(body) {
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

    #[test]
    fn test_comments() {
        let redditor = about_redditor("Axelior").unwrap();
        let comments = redditor.comments().unwrap();

        Debug!("{:?}", comments);
    }
}