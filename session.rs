use http::client::RequestWriter;
use http::method::{Get, Post};
use http::headers::content_type::MediaType;
use std::str::from_utf8;
use std::rt::io::{Reader,Writer};
use extra::url;
use extra::json::{Json, from_str};

use util::json::{JsonLike};
use util::REDDIT;

#[deriving(ToStr, Clone, Encodable, Decodable, Eq)]
pub struct Session {
    default: bool,
    username: ~str,
    password: Option<~str>,
    cookie: ~str,
    modhash: ~str,
    settings: ~SessionSettings
}

#[deriving(ToStr, Clone, Encodable, Decodable, Eq)]
pub struct SessionSettings {
    delete_pass: bool,
    auto_relogin: bool,
}

impl SessionSettings {
    pub fn new(delete_pass: bool, auto_relogin: bool) -> SessionSettings {
        SessionSettings {
            delete_pass: delete_pass,
            auto_relogin: auto_relogin,
        }
    }

    pub fn default() -> SessionSettings {
        SessionSettings {
            delete_pass: false,
            auto_relogin: false,
        }
    }
}

impl Session {
    pub fn default() -> Session {
        Session {
            default: true,
            username: ~"",
            password: None,
            cookie: ~"",
            modhash: ~"",
            settings: ~SessionSettings::default(),
        }
    }

    pub fn new(username: &str, pass: &str, settings: SessionSettings) -> Session {
        Session {
            default: false,
            username: username.to_owned(),
            password: Some(pass.to_owned()),
            cookie: ~"",
            modhash: ~"",
            settings: ~settings,
        }
    }

    pub fn login(self) -> Result<Session, ~str> {
        if self.default {
            return Err(~"Cannot login with default session");
        }

        let url = url::from_str(format!("{0}api/login/{1}", REDDIT, self.username)).unwrap();

        let jpostdata = format!(r"user={0}&passwd={1}&rem=true&api_type=json",
            self.username,
            *self.password.as_ref().expect("No password!"));

        let mut request = ~RequestWriter::new(Post, url);

        request.headers.content_length = Some(jpostdata.len());
        request.headers.content_type = Some(MediaType(~"application",
                                                        ~"x-www-form-urlencoded"
                                                        , ~[]));

        request.write(jpostdata.as_bytes());

        match request.read_response() {
            Ok(mut resp) => {
                let body = from_utf8(resp.read_to_end());

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
                                Ok(Session {
                                    cookie: cookie,
                                    modhash: mhash,
                                    ..self
                                })

                            }
                        }
                    }
                }
            }
            Err(_) => Err(~"Request failed")
        }
    }

    pub fn me(&self) -> Result<Json, ~str> {
        let url = url::from_str(format!("{}api/me.json", REDDIT)).unwrap();

        let mut request = ~RequestWriter::new(Get, url);

        request.headers.extensions.insert(~"Cookie", self.cookie.clone());

        match request.read_response() {
            Ok(mut resp) => {
                let body = from_utf8(resp.read_to_end());

                match from_str(body) {
                    Err(jerror) => Err(jerror.to_str()),
                    Ok(json) => Ok(json),
                }
            }

            Err(_) => Err(~"Request not successful"),
        }
    }

    pub fn clear(self, pass: &str, dest: &url::Url) -> Result<Session, ~str> {
        let url = url::from_str(format!("{}api/clear_sessions", REDDIT)).unwrap();

        let jpostdata = format!(r"dest={0}&curpass={1}&uh={2}&api_type=json",
            dest.to_str(), pass, self.modhash);

        let mut request = ~RequestWriter::new(Post, url);
        request.headers.extensions.insert(~"Cookie", self.cookie.clone());
        request.headers.content_length = Some(jpostdata.len());
        request.headers.content_type = Some(MediaType(~"application",
                                                        ~"x-www-form-urlencoded"
                                                        , ~[]));
        request.write(jpostdata.as_bytes());

        match request.read_response() {
            Ok(mut resp) => {
                let body = from_utf8(resp.read_to_end());
                Debug!("Body of User.clear() response [{}]", body);

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
                                Ok(Session {
                                    cookie: cookie,
                                    ..self
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
    let err_list = json.value(&~"json").value(&~"errors").as_list().unwrap();

    if err_list.len() != 0 {
        Err(err_list.to_str())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Session, SessionSettings};
    use util::REDDIT;
    use extra::url::from_str;

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
    fn test_login() {
        let (user, pass) = get_user_pass();

        let u = Session::new(user, pass, SessionSettings::default());
        let u = u.login().unwrap();

        Debug!("{}",u.to_str());

        assert!(u.cookie.len() > 0);
    }

    #[test]
    fn test_me() {
        let (user, pass) = get_user_pass();

        let u = Session::new(user, pass, SessionSettings::default());
        let u = u.login().unwrap();

        let user_info = u.me().unwrap().to_str();

        assert!(user_info.len() > 0);
    }

    #[test]
    fn test_clear() {
        let (user, pass) = get_user_pass();
        let dest = from_str(REDDIT).unwrap();

        let u = Session::new(user, pass, SessionSettings::default());
        let user = u.login().unwrap();
        let cookie = user.clear(pass, &dest).unwrap().cookie;

        Debug!("{}", cookie);
        assert!(cookie.len() > 0);
    }
}