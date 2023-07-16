rust
// as_ref() is there to prevent consuming Value
let date_start =
    if let Some(ds) = remove(&mut map, "date_start")
        .as_ref().and_then(Value::as_str)
    {
        Some(DateTime::parse_from_str(ds, "%Y-%m-%d")?)
    } else {
        None
    };
