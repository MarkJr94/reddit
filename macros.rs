#[macro_escape];

// A nice macro to define structs that are built from json objects
macro_rules! json_struct(
    ($name:ident, $($json_field:expr -> $field:ident: $ty:ty), +) => (
        impl ::util::json::FromJson for $name {
            fn from_json(json: &::extra::json::Json) -> Result<$name, ::util::json::ValueError> {
                Ok($name {
                    $($field: match(json.value(&~$json_field).convert()) {
                        Ok(v) => v,
                        Err(e) => return Err(e)
                    }),+
                })
            }
        }
    )
)

macro_rules! Debug (
    ($msg:expr) => (
        debug!("{0} L{1}:C{2}: {3}", module_path!(), line!(), col!(), $msg)
    );

    ($msg:expr, $($arg:tt)*) => (
        debug!("{0} L{1}:C{2}: {3}", module_path!(), line!(), col!(),
            format!($msg, $($arg)*))
    )
)