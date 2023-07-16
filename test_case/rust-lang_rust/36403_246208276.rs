 rust
fn get_stream_mut(s: &mut String) -> Option<&mut String> {
    unimplemented!()
}

fn new_request(s: &mut String) -> &mut String {
    unimplemented!()
}

fn get_or_create_stream(s: &mut String) -> &mut String {
    {
        if get_stream_mut(s).is_some() {
            return get_stream_mut(s).unwrap()
        }
    };

    new_request(s)
}
