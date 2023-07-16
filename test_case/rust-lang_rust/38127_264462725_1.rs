rust
macro_rules! parse_optional_string {
    ($field: ident, $json_map: ident, $decoder: expr) => {
        if let Some(ds) = remove(&mut $json_map, stringify!($field))
            .as_ref().and_then(Value::as_str)
        {
            ($decoder)(ds).map(Some)
        } else {
            Ok(None)
        }
    };
}
