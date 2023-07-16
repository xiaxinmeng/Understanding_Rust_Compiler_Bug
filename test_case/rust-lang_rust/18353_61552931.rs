
struct Str {
    f: [u8]
}

fn main() {
    let str: Option<&Str> = None;
    str.is_some();
}
