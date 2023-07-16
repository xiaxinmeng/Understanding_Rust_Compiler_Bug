
// This is kind of correct/portable Rust program:
extern crate localencoding;

fn main() {
    let str: Cow<'static, [u8]> = localencoding::encode("Hello Grzegorz Brzęczyszczykiewicz");
    std::io::stdout().write(str.borrow());
}

// This program is not portable

fn main() {
    println!("Hello Grzegorz Brzęczyszczykiewicz");
}
