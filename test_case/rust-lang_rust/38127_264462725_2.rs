rust
let date_start =
    parse_optional_string!(date_start, map,
                           |ds| DateTime::parse_from_str(ds, "%Y-%m-%d"))?; // <- note `?` here
