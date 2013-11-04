pub use self::sr::Subreddit;

use self::sr::Subreddit;
use std::str::from_utf8;
use std::rt::io::Reader;
use extra::url;
use extra::json::from_str;

use util::{REDDIT, get_resp};
use util::json::{JsonLike, FromJson};

json_struct2!(sr, Subreddit,
    "display_name" -> name: ~str,
    "title" -> title: ~str,
    "description" -> desc: ~str,
    "public_description" -> public_desc: ~str,
    "url" -> url: ~str,
    "name" -> full_id: ~str,
    "header_img" -> header_img: ~str,
    "created_utc" -> date_made: f64,
    "subscribers" -> num_subs: int,
    "over18" -> is_nsfw: bool)

pub fn about_subreddit(sr: &str) -> Result<Subreddit, ~str> {
    let url = url::from_str(format!("{0}r/{1}/about.json", REDDIT, sr)).unwrap();

    get_resp(url, None, None).and_then(|mut resp| {
        let body = from_utf8(resp.read_to_end());

        from_str(body).or_else(|e| Err(e.to_str())).and_then(|json| {
            json.value(&~"data").or_else(|e| Err(e.to_str())).and_then(|sr_json| {
                FromJson::from_json(sr_json).or_else(|e| Err(e.to_str()))
            })
        })
    })
}

#[cfg(test)]
mod test {
    use super::{about_subreddit};

    #[test]
    fn test_about() {
        let redditor = about_subreddit("programming").unwrap();

        Debug!("{}", redditor.to_str());
    }
}