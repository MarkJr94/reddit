use util::{REDDIT};
use util::json::{JsonLike};

#[deriving(ToStr, Clone, Encodable, Decodable, Eq)]
pub struct Post {
    author: ~str,
    title: ~str,
    url: ~str,
    domain: ~str,
    subreddit: ~str,
    subreddit_id: ~str,
    name: ~str,
    id: ~str,
    permalink: ~str,
    selftext: ~str,
    thumbnail: ~str,

    created_utc: f64,
    num_comments: int,
    score: int,
    ups: int,
    downs: int,
    over_18: bool,
    is_self: bool,
    clicked: bool,
    saved: bool,
    banned_by: Option<~str>
}

from_json!(Post,
    author,
    title,
    url,
    domain,
    subreddit,
    subreddit_id,
    name,
    id,
    permalink,
    selftext,
    thumbnail,
    created_utc,
    num_comments,
    score,
    ups,
    downs,
    over_18,
    is_self,
    clicked,
    saved,
    banned_by)



impl Post {
    pub fn full_permalink(&self) -> ~str {
        REDDIT.to_owned() + self.permalink
    }
}

#[deriving(ToStr, Eq, Encodable, Decodable, Clone)]
pub enum PopularitySort {
    DefaultPop,
    Hot,
    New,
    Rising,
    Top,
    Controversial
}

impl PopularitySort {
    pub fn as_str(&self) -> ~str {
        match *self {
            DefaultPop => ~"",
            Hot => ~"hot",
            New => ~"new",
            Rising => ~"rising",
            Top => ~"top",
            Controversial => ~"controversial"
        }
    }
}

#[deriving(ToStr, Eq, Encodable, Decodable, Clone)]
pub enum AgeSort {
    DefaultAge,
    Hour,
    Month,
    Year,
    AllTime,
}

impl AgeSort {
    pub fn as_str(&self) -> ~str {
        match *self {
            DefaultAge => ~"",
            Hour => ~"hour",
            Month => ~"month",
            Year => ~"year",
            AllTime => ~"all"
        }
    }
}

pub fn full_permalink(p: &Post) -> ~str {
    ~"http://reddit.com" + p.permalink
}


