rust
use std::fmt;

struct Foo {
    bar: i32,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:f>5}", self.bar)
    }
}

fn main() {
    println!("|{:<30}|", "");
    println!("|{:<30}|", 12);
    println!("|{:<30}|", Foo {bar: 12});
    println!("|{:<30}|", format!("{}", Foo {bar: 12}));
}
