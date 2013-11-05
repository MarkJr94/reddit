use std::str;
use std::rt::io::Reader;
use extra::url;
use extra::json;

use post::{Post, PopularitySort, AgeSort};
use util::{REDDIT, get_resp};
use util::json::{JsonLike, FromJson};

#[deriving(ToStr, Clone, Encodable, Decodable, Eq)]
pub struct Subreddit {
    display_name: ~str,
    title: ~str,
    description: ~str,
    public_description: ~str,
    url: ~str,
    name: ~str,
    header_img: ~str,
    created_utc: f64,
    subscribers: int,
    over18: bool
}

from_json!(Subreddit,
    display_name,
    title,
    description,
    public_description,
    url,
    name,
    header_img,
    created_utc,
    subscribers,
    over18)

pub fn about_subreddit(sr: &str) -> Result<Subreddit, ~str> {
    let url = url::from_str(format!("{0}r/{1}/about.json", REDDIT, sr)).unwrap();

    get_resp(url, None, None).and_then(|mut resp| {
        let body = str::from_utf8(resp.read_to_end());

        json::from_str(body).or_else(|e| Err(e.to_str())).and_then(|json| {
            json.value(&~"data").or_else(|e| Err(e.to_str())).and_then(|sr_json| {
                FromJson::from_json(sr_json).or_else(|e| Err(e.to_str()))
            })
        })
    })
}

#[deriving(Encodable,Decodable)]
struct inner {
    data: Option<Post>
}

#[deriving(Encodable, Decodable)]
struct data {
    children: ~[inner]
}

#[deriving(Encodable,Decodable)]
struct Response {
    data: data
}

impl Subreddit {
    pub fn front_page() -> Result<~[Post], ~str> {
        use extra::serialize::Decodable;

        let url = url::from_str("http://www.reddit.com/.json").unwrap();

        get_resp(url, None, None).and_then(|mut resp| {
            let body = str::from_utf8(resp.read_to_end());

            json::from_str(body).or_else(|e| Err(e.to_str())).and_then(|json|{
                let mut dec = json::Decoder(json);

                let res_data: Response = Decodable::decode(&mut dec);

                Ok(res_data.data.children.move_iter().map(|c| c.data.unwrap()).collect())
            })
        })
    }

    pub fn sorted_posts(sub: Option<Subreddit>, pop: PopularitySort, age: AgeSort)
    -> Result<~[Post], ~str> {
        use post::{New, Rising, Hot, DefaultAge, DefaultPop};
        use extra::serialize::Decodable;

        if age != DefaultAge {
            match pop {
                New | Rising | Hot => {
                    return Err(format!("Cannot sort {0} by {1}", pop.to_str(), age.to_str()));
                }
                _ => ()
            }
        }

        let mut raw_url = ~"http://reddit.com/";

        match sub {
            Some(s) => {
                raw_url = format!("{}r/{}/", raw_url, s.title);
            }
            _ => {}
        }

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

        let url = url::from_str(raw_url).unwrap();

        get_resp(url, None, None).and_then(|mut resp| {
            let body = str::from_utf8(resp.read_to_end());

            json::from_str(body).or_else(|e| Err(e.to_str())).and_then(|json| {
                let mut dec = json::Decoder(json);

                let res_data: Response = Decodable::decode(&mut dec);
                Ok(res_data.data.children.move_iter().map(|c| c.data.unwrap()).collect())
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::{about_subreddit, Subreddit};

    #[test]
    fn test_about() {
        let redditor = about_subreddit("programming").unwrap();

//         Debug!("{}", redditor.to_str());
    }

    #[test]
    fn test_front_page() {
        let front_posts =  Subreddit::front_page().unwrap();

//         Debug!("{}", front_posts.to_str());
    }

    #[test]
    fn test_sorted() {
        use post::{Top, AllTime};

        let sub = about_subreddit("programming").unwrap();

        let posts = Subreddit::sorted_posts(Some(sub), Top, AllTime).unwrap();

        Debug!("{}", posts.to_str());
    }
}