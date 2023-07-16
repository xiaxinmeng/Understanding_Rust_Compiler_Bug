rust
fn func(input: u8) {
    match input {
        3 => 4,
        _ => {
            // side effect ...
            return;
        },
    };
}
