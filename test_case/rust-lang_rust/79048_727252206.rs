rust
fn parse_data(data: &[u8]) -> u8 {
    match data {
        b"aaa" => 1,
        &[_, _, _] => 1,
    }
}
