rust
fn parse<'b>(_r: *mut [&'b u8], _b: &'b u8) {}

fn main() {
    let mut buf = 0u8;
    let mut headers = [];
    loop {
        {&mut buf;}
        parse(&mut headers, &buf);
    }
}
