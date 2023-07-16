
pub trait Foo {}

struct Bar;
struct Baz;

impl Foo for Bar { }
impl Foo for Baz { }

fn not_all_paths(a: &str) -> u32 {
    match a {
        "baz" => 0,
        _ => 1,
    }; // "not all control paths return a value" due to ;
}

fn right(b: &str) -> Box<Foo> {
    match b {
        "baz" => Box::new(Baz),
        _ => Box::new(Bar),
    }
}

fn wrong(c: &str) -> Box<Foo> {
    match c {
        "baz" => Box::new(Baz),
        _ => Box::new(Bar),
    }; // "match arms have incompatible types" due to ;
}

fn main() {
}
