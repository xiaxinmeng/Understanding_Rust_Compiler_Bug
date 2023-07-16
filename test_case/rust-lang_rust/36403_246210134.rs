 rust
`fn get_or_create_stream<'a>(s: &'a mut String) -> &'a mut String {
    let b: bool = { get_stream_mut(s).is_some() };
    if b {
        get_stream_mut(s)
    } else {
        new_request(s)
    }
}
