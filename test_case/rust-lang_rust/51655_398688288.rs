rust
const PATH_DOT: &[u8] = &[b'.'];

fn append_to_existing_path(element: &[u8]) {
    match element {
        &[] => {}
        PATH_DOT => {}
        _ => {}
    }
}
