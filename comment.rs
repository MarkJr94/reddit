use std::str::from_utf8;

use extra::json::Json;
use extra::url;

use util::{REDDIT, get_resp, check_errors};
use util::json::{JsonLike, FromJson};

json_struct2!(Comment,
    "approved_by" -> approved_by: Option<~str>,
    "author" -> author: ~str,
    "author_flair_css_class" -> flair_class: Option<~str>,
    "author_flair_text" -> flair_text: Option<~str>,
    "banned_by" -> banned_by: Option<~str>,
    "body" -> body: ~str,
    "body_html" -> body_html: ~str,
//     "edited" -> edited: Result<bool, f64>,
    "gilded" -> gilded: int,
    "likes" -> likes: Option<bool>,
    // Link Author attr skipped
    "link_id" -> link_id: ~str,
    // Link Title attr skipped,
    "num_reports" -> num_reports: Option<int>,
    "parent_id" -> parent_id: ~str,
    "saved" -> saved: bool,
    "score_hidden" -> score_hidden: bool,
    "subreddit" -> subreddit: ~str,
    "subreddit_id" -> subreddit_id: ~str,
    "distinguished" -> distinguished: Option<~str>,
    "created" -> created: f64,
    "created_utc" -> created_utc: f64,
    "ups" -> ups: int,
    "downs" -> downs: int)