/*
 * Hi there !
 *
 * I found that handling Json was quite annoying, so I wrote this
 * little lib to help. You can find an example of the usage in the
 * tests at the end.
 *
 * I lacked inspiration on the naming of value() and item(), and got
 * lost in the lifetimes, so from_json takes a Json and not a
 * JsonLike.
 *
 * I'm new to rust, so any comment is welcome.
 */

#[macro_escape];

use extra::json::{Json, Object, List, String, Boolean, Number, Null};
use std::fmt;

/// This structure is returned when an unexpected value is uncountered in the json
pub struct ValueError{
    /// Representation of the value that cause an error
    value: ~str,
    /// A message describing the type of error
    msg: ~str
}

/// Convenience function for errors
fn error<T>(json: &Json, msg: ~str) -> Result<T, ValueError> {
    Err(ValueError{value: json.to_pretty_str(), msg: msg})
}


impl fmt::Default for ValueError {
    fn fmt(obj: &ValueError, f: &mut fmt::Formatter) {
        write!(f.buf, r#"Can't convert value: "{}" Reason: "{}""#, obj.value, obj.msg);
    }
}

impl ToStr for ValueError {
    fn to_str(&self) -> ~str {
        format!("Can convert value \"{}\": {}", self.value, self.msg)
    }
}

/// This trait defines a number of operation that make getting data out of Json easier
pub trait JsonLike<'a> {
    /// If this json value is a string, returns a reference to it
    fn as_str(self) -> Result<&'a ~str, ValueError>;
    /// If this json value is an object, returns a reference to it
    fn as_obj(self) -> Result<&'a ~Object, ValueError>;
    /// If this json value is a list, returns a reference to it
    fn as_list(self) -> Result<&'a List, ValueError>;
    /// If this json value is a boolean, returns it
    fn to_bool(self) -> Result<bool, ValueError>;
    /// If this json value is a number, returns it
    fn to_num(self) -> Result<f64, ValueError>;
    /// If this json value is an object and it has a value matching the key, returns it
    fn value(self, key: &~str) -> Result<&'a Json, ValueError>;
    /// If this json value is a list and has an object to the index, returns it
    fn item(self, index: uint) -> Result<&'a Json, ValueError>;

    /// Convert this json value to a type that implements `FromJson`
    fn convert<T: FromJson>(self) -> Result<T, ValueError>;
}

// This implementation is simple, it's just a matter of matching the
// value and returning it if it's the right type
impl<'a> JsonLike<'a> for &'a Json {
    fn as_str(self) -> Result<&'a ~str, ValueError> {
        match *self {
            String(ref s) => Ok(s),
            _ => error(self, ~"not a string")
        }
    }

    fn as_obj(self) -> Result<&'a ~Object, ValueError> {
        match *self {
            Object(ref o) => Ok(o),
            _ => error(self, ~"not an object")
        }
    }

    fn as_list(self) -> Result<&'a List, ValueError> {
        match *self {
            List(ref l) => Ok(l),
            _ => error(self, ~"not a list")
        }
    }

//     fn as_null(self) -> Result<&'a Null, ValueError> {
//         match *self {
//             Null =>
//         }
//     }

    fn to_bool(self) -> Result<bool, ValueError> {
        match *self {
            Boolean(b) => Ok(b),
            _ => error(self, ~"not a boolean")
        }
    }

    fn to_num(self) -> Result<f64, ValueError> {
        match *self {
            Number(b) => Ok(b),
            _ => error(self, ~"not a number")
        }
    }

    fn value(self, key: &~str) -> Result<&'a Json, ValueError> {
        self.as_obj().and_then( |o| {
            match o.find(key) {
                Some(v) => Ok(v),
                None => error(self, format!("has no key \"{}\"", *key))
            }
        })
    }

    fn item(self, index: uint) -> Result<&'a Json, ValueError>{
        self.as_list().and_then(|l| {
            match l.get(index) {
                Some(v) => Ok(v),
                None => error(self, format!("has no index {}", index))
            }
        })
    }

    fn convert<T: FromJson>(self) -> Result<T, ValueError> {
        FromJson::from_json(self)
    }
}

// This implementation just pass the call to the enclosed value if
// there is one
impl<'a> JsonLike<'a> for Result<&'a Json, ValueError> {
    fn as_str(self) -> Result<&'a ~str, ValueError> {
        self.and_then( |j| {j.as_str()} )
    }

    fn as_obj(self) -> Result<&'a ~Object, ValueError> {
        self.and_then( |j| {j.as_obj()} )
    }

    fn as_list(self) -> Result<&'a List, ValueError> {
        self.and_then( |j| {j.as_list()} )
    }

     fn to_bool(self) -> Result<bool, ValueError> {
        self.and_then( |j| {j.to_bool()} )
    }

    fn to_num(self) -> Result<f64, ValueError> {
        self.and_then( |j| {j.to_num()} )
    }

    fn value(self, key: &~str) -> Result<&'a Json, ValueError> {
        self.and_then( |j| {j.value(key)} )
    }

    fn item(self, index: uint) -> Result<&'a Json, ValueError>{
        self.and_then( |j| {j.item(index)} )
    }

    fn convert<T: FromJson>(self) -> Result<T, ValueError> {
        self.and_then( |j| {j.convert()} )
    }
}

/// A trait for getting values from json
pub trait FromJson {
    fn from_json(j :&Json) -> Result<Self, ValueError>;
}

impl FromJson for f64 {
    fn from_json(j: &Json) -> Result<f64, ValueError> {
        j.to_num()
    }
}

impl FromJson for f32 {
    fn from_json(j: &Json) -> Result<f32, ValueError> {
        j.to_num().map(|n| n as f32)
    }
}

impl FromJson for int {
    fn from_json(j: &Json) -> Result<int, ValueError> {
        j.to_num().and_then( |i| {
            match i.to_int() {
                Some(v) => Ok(v),
                None => error(j, ~"is not an integer")
            }
        })
    }
}

impl FromJson for ~str {
    fn from_json(j: &Json) -> Result<~str, ValueError> {
        j.as_str().map( |s| {s.clone()} )
    }
}

impl FromJson for bool {
    fn from_json(j: &Json) -> Result<bool, ValueError> {
        j.to_bool()
    }
}

impl<T: FromJson> FromJson for ~[T] {
    fn from_json(j: &Json) -> Result<~[T], ValueError> {
        match *j {
            List(ref l) => {
                let mut list = l.iter()
                    .map(|json|  {
                        let dummy: Result<T, ValueError> = FromJson::from_json(json);
                        match dummy {
                            Err(ref e) => Debug!("{}", *e),
                            _ => {}
                        }
                        dummy
                    })
                    .filter(|result| result.is_ok())
                    .map(|result| result.unwrap())
                    .collect();

                Ok(list)
            }
            _ => error(j, ~"is not a list")
        }
    }
}
impl<T: FromJson> FromJson for Option<T> {
    fn from_json(j: &Json) -> Result<Option<T>, ValueError> {
        match *j {
            Null => Ok(None),
            ref otherwise => {
                let t = FromJson::from_json(otherwise);

                match t {
                    Ok(thing) => Ok(Some(thing)),
                    Err(v) => Err(v)
                }
            }
        }
    }
}

impl<T: FromJson, E: FromJson> FromJson for Result<T, E> {
    fn from_json(j: &Json) -> Result<Result<T, E>, ValueError> {
        let t: Result<T, ValueError> = FromJson::from_json(j);
        match t {
            Ok(t_res) => { return Ok(Ok(t_res)); }
            _ => {
                let e: Result<E, ValueError> = FromJson::from_json(j);
                match e {
                    Ok(e_res) => { return Ok(Err(e_res)); }
                    _ => {
                        return error(j, ~"Could not parse either Result type");
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use extra::json::{Json, from_str};
    use super::{FromJson, JsonLike};
    static DATA : &'static str = r###"{"count": 2, "results":[{"Name": "regalia", "Author": "madjar", "ID": 42}]}"###;

    #[test]
    fn test_get_some_value() {
        let json: Json = from_str(DATA).unwrap();

        // Get value from the json without having to match the result
        let author_object = json.value(&~"results").item(0).value(&~"Author");

        // Convert it using convert(), dispatched on the return type
        let author: ~str = author_object.convert().unwrap();

        // Not demonstrated here : all is wrapper into Result object, with nice error messages

        assert!(author == ~"madjar");
    }

    #[test]
    fn test_struct_macro() {
        // Create a struct that will be automatically populated from a json value

        // Defining a struct shouldn't be necessary, but #4357
        struct Object {
            name: ~str,
            author: ~str,
            id: int
        }
        json_struct!(Object,
                     "Name" -> name: ~str,
                     "Author" -> author: ~str,
                     "ID" -> id: int)

        let json: Json = from_str(DATA).unwrap();
        let object: Object = FromJson::from_json(json.value(&~"results").item(0).unwrap()).unwrap();
        assert!(object.author == ~"madjar");
        assert!(object.id == 42);
    }
}