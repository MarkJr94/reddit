use std::str::from_utf8;
use extra::url;
use extra::json::{Json, from_str};

use util::json::{JsonLike};
use util::{REDDIT, check_errors, get_resp};

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

        get_resp(url, Some(jpostdata.as_bytes()), Some(&self)).and_then(|mut resp| {
            let body = from_utf8(resp.read_to_end());

            from_str(body).or_else(|e| Err(e.to_str()))
                .and_then(|json| {
                    check_errors(&json).and_then(|_| {
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
                            ..self.clone()
                        })
                    })
                })
        })
    }

    pub fn me(&self) -> Result<Json, ~str> {
        let url = url::from_str(format!("{}api/me.json", REDDIT)).unwrap();

        get_resp(url, None, Some(self)).and_then(|mut resp_data| {
            let body: ~str = from_utf8(resp_data.read_to_end());

            from_str(body).or_else(|e| Err(e.to_str()))
        })
    }

    pub fn clear(self, pass: &str, dest: &url::Url) -> Result<Session, ~str> {
        let url = url::from_str(format!("{}api/clear_sessions", REDDIT)).unwrap();

        let jpostdata = format!(r"dest={0}&curpass={1}&uh={2}&api_type=json",
            dest.to_str(), pass, self.modhash);


        get_resp(url, Some(jpostdata.as_bytes()), Some(&self)).and_then(|mut resp| {
            from_str(from_utf8(resp.read_to_end()))
                .or_else(|e| Err(e.to_str()) )
                .and_then(|json| {
                    Debug!(json.to_str());

                    check_errors(&json).and_then(|_| {
                        let cookie = resp.headers.extensions.find(&~"Set-Cookie")
                                    .expect("No cookie sent back")
                                    .to_owned();

                        Ok(Session { cookie: cookie, ..self.clone()})
                    })
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Session, SessionSettings};
    use util::{REDDIT, get_secrets};
    use extra::url::from_str;

    #[test]
    fn test_login() {
        let (user, pass) = get_secrets();

        let u = Session::new(user, pass, SessionSettings::default());
        let u = u.login().unwrap();

        Debug!("{}",u.to_str());

        assert!(u.cookie.len() > 0);
    }

    #[test]
    fn test_me() {
        let (user, pass) = get_secrets();

        let u = Session::new(user, pass, SessionSettings::default());
        let u = u.login().unwrap();

        let user_info = u.me().unwrap().to_str();

        assert!(user_info.len() > 0);
    }

    #[test]
    fn test_clear() {
        let (user, pass) = get_secrets();
        let dest = from_str(REDDIT).unwrap();

        let u = Session::new(user, pass, SessionSettings::default());
        let user = u.login().unwrap();
        let cookie = user.clear(pass, &dest).unwrap().cookie;

        Debug!("{}", cookie);
        assert!(cookie.len() > 0);
    }
}