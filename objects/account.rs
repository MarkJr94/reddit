
json_struct2!(Account,
    "comment_karma" -> comment_karma: int,
    "created" -> created: f64,
    "created_utc" -> created_utc: f64,
    "has_mail" -> has_mail: Option<bool>,
    "has_mod_mail" -> has_mod_mail: Option<bool>,
    "has_verified_email" -> has_verified_email: bool,
    "id" -> id: ~str,
    "is_friend" -> is_friend: bool,
    "is_gold" -> is_gold: bool,
    "is_mod" -> is_mod: bool,
    "link_karma" -> link_karma: int,
    "modhash" -> modhash: Option<~str>,
    "name" -> name: ~str,
    "over_18" -> over_18: bool)

static DEFAULT: Account = Account {
    comment_karma: 0i,
    created: 0f64,
    created_utc: 0f64,
    has_mail: None,
    has_mail: None,
    has_verified_email: false,
    id: ~"",
    is_friend: false,
    is_gold: false,
    is_mod: false,
    link_karma: 0i,
    name: ~"",
    over_18: false
};

impl Account {
    pub fn default() -> Account {
        DEFAULT.clone()
    }

    pub fn login(username: &str, password: &str)) -> Result<(Account, ~str), ~str> {

        let url = url::from_str(format!("{0}api/login/{1}", REDDIT, self.username)).unwrap();

        let jpostdata = format!(r"user={0}&passwd={1}&rem=true&api_type=json",
            username,
            password);

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

                        Ok((from_json(json.value(&~"json").value(&~"data")
                            .expect("No Data found!"))), cookie)
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

    pub fn clear(self, pass: &str, dest: &url::Url) -> Result<Account, ~str> {
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

                        Ok(Account { cookie: cookie, ..self.clone()})
                    })
            })
        })
    }
}