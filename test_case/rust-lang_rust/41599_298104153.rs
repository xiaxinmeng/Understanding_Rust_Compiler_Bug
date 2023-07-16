rust
fn parse<'b>(r: &mut [&'b u8], b: &'b u8) {
    r[0] = b;
}

fn main() {
    let mut buf = 0u8;
    let mut headers = [&0];
    parse(&mut headers, &buf);
    let immutable: [&u8; 1] = headers;
    assert_eq!(*immutable[0], 0);
    buf = 1;
    assert_eq!(*immutable[0], 1);
}
