use http::client::RequestWriter;
use http::method::{Get, Post};
use http::headers::content_type::MediaType;
use std::str;
use std::rt::io::{Reader,Writer};
use extra::url;
use extra::json::{Json, from_str};

use util::json::{JsonLike};
use util::REDDIT;

#[deriving(Clone, Encodable, Decodable, Eq)]
pub struct User {
    cookie: ~str,
    modhash: ~str,

}

impl User {
    pub fn login(user: &str, pass: &str) -> Result<User, ~str> {
        let url = url::from_str("http://www.reddit.com/api/login").unwrap();

        let jpostdata = format!(r"user={0}&passwd={1}&rem=true&api_type=json", user, pass);

        let mut request = ~RequestWriter::new(Post, url);

        request.headers.content_length = Some(jpostdata.len());
        request.headers.content_type = Some(MediaType(~"application",
                                                        ~"x-www-form-urlencoded"
                                                        , ~[]));

        request.write(jpostdata.as_bytes());

        match request.read_response() {
            Ok(mut resp) => {
                debug!("Status: {}", resp.status);

                let body = str::from_utf8(resp.read_to_end());
                debug!("{}", body);

                match from_str(body) {
                    Err(jerror) => Err(jerror.to_str()),
                    Ok(json) => {
                        let err = check_login(&json);

                        match err {
                            Err(msg) => Err(msg),
                            Ok(()) => {
                                let cookie = resp.headers.extensions.find(&~"Set-Cookie")
                                    .expect("No cookie sent back")
                                    .to_owned();

                                let mhash = json.value(&~"json").value(&~"data")
                                    .value(&~"modhash")
                                    .expect("No modhash found!")
                                    .as_str()
                                    .unwrap()
                                    .to_owned();
                                Ok(User {
                                    cookie: cookie,
                                    modhash: mhash,
                                })

                            }
                        }
                    }
                }
            }
            Err(_) => Err(~"Request failed")
        }
    }

    pub fn me_json(&self) -> Result<Json, ~str> {
        let url = url::from_str(format!("{}api/me.json", REDDIT)).unwrap();

        let mut request = ~RequestWriter::new(Get, url);

        request.headers.extensions.insert(~"Cookie", self.cookie.clone());

        match request.read_response() {
            Ok(mut resp) => {
                debug!("Status: {}", resp.status);

                let body = str::from_utf8(resp.read_to_end());
                debug!("{}", body);

                match from_str(body) {
                    Err(jerror) => Err(jerror.to_str()),
                    Ok(json) => Ok(json),
                }
            }

            Err(_) => Err(~"Request not successful"),
        }
    }

    pub fn clear_sessions(self, pass: &str, dest: &url::Url) -> Result<User, ~str> {
        let url = url::from_str(format!("{}api/clear_sessions", REDDIT)).unwrap();

        let jpostdata = format!(r"dest={0}&curpass={1}&uh={2}&api_type=json", dest.to_str(), pass, self.modhash);

        let mut request = ~RequestWriter::new(Post, url);
        request.headers.extensions.insert(~"Cookie", self.cookie.clone());
        request.headers.content_length = Some(jpostdata.len());
        request.headers.content_type = Some(MediaType(~"application",
                                                        ~"x-www-form-urlencoded"
                                                        , ~[]));
        request.write(jpostdata.as_bytes());

        match request.read_response() {
            Ok(mut resp) => {
                debug!("Status: {}", resp.status);

                let body = str::from_utf8(resp.read_to_end());
                debug!("{}", body);

                match from_str(body) {
                    Err(jerror) => Err(jerror.to_str()),
                    Ok(json) => {
                        let err = check_login(&json);

                        match err {
                            Err(msg) => Err(msg),
                            Ok(()) => {
                                let cookie = resp.headers.extensions.find(&~"Set-Cookie")
                                    .expect("No cookie sent back")
                                    .to_owned();
                                Ok(User {
                                    cookie: cookie,
                                    modhash: self.modhash
                                })
                            }
                        }
                    }
                }
            }
            Err(_) => Err(~"Request failed")
        }
    }
}



fn check_login(json: &Json) -> Result<(), ~str> {
    debug!("Login Json return: {}", json.to_str());

    let err_list = json.value(&~"json").value(&~"errors").as_list().unwrap();

    if err_list.len() != 0 {
        Err(err_list.to_str())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{User};
    use util::REDDIT;
    use extra::url::from_str;

    fn get_user_pass() -> (~str, ~str) {
        use std::rt::io::{file, Open, Read};
        use std::path::Path;
        use std::str::from_utf8;

        let mut stream = file::open(&Path::new("secrets.txt"), Open, Read).expect("Secret file couldn't be opened");
        let s = from_utf8(stream.read_to_end());

        let v: ~[~str] = s.split_iter(' ').map(|s| s.trim().to_owned()).collect();

        debug!(r"\{ user: [{}], pass: [{}]\}", v[0], v[1]);

        (v[0].clone(), v[1])

    }

    #[test]
    fn test_login() {
        let (user, pass) = get_user_pass();

        let cookie_s = User::login(user, pass).unwrap().cookie;
        assert!(cookie_s.len() > 0);
    }

    #[test]
    fn test_get_me() {
        let (user, pass) = get_user_pass();

        let user = User::login(user, pass).unwrap();
        let user_info = user.me_json().unwrap().to_str();

        assert!(user_info.len() > 0);
    }

    #[test]
    fn test_clear_sessions() {
        let (user, pass) = get_user_pass();
        let dest = from_str(REDDIT).unwrap();

        let user = User::login(user, pass).unwrap();
        let cookie = user.clear_sessions(pass, &dest).unwrap().cookie;

        println(cookie);
        assert!(cookie.len() > 0);
    }
}