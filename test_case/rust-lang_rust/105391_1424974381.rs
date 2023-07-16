
struct T;

struct Blah {
    a: T,
}

impl Drop for Blah {
    fn drop(&mut self) {}
}

fn main() {
    Blah {
        ..Blah { a: T }
    };
}
