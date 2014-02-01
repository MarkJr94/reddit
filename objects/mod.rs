// mod account;

pub trait Thing {
    fn id<'a>(&'a self) -> &'a str;
    fn name(&self) -> ~str {
        self.kind() + "_" + self.id()
    }
    fn kind<'a>(&'a self) -> &'a str;
    fn data<'a>(&'a self) -> &'a Any;
}


pub struct Listing {
    before: ~str,
    after: ~str,
    modhash: ~str,
    data: ~[~Thing]
}

pub trait Votes {
    fn ups(&self) -> i32;
    fn downs(&self) -> i32;
    fn net(&self) -> i32 {
        self.ups() - self.downs()
    }

    fn likes(&self) -> Option<bool>;
}

pub trait Create {
    fn created(&self) -> f64;
    fn created_utc(&self) -> f64;
}

