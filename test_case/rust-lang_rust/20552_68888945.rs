 rust
pub fn get_latest_as_json<T>() -> String where T: for<'a> Encodable<Encoder<'a>, fmt::Error>> + Record
