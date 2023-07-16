 rust
fn get_stream_mut<'a>(s: &'a mut String) -> Option<&'a mut String> {
    unimplemented!()
}
fn get_or_create_stream<'a>(s: &'a mut String) -> &'a mut String {
    {
        match get_stream_mut(s) {
            Some(stream) => return stream,
            None => (),
        };
    };
    new_request(s)
}
