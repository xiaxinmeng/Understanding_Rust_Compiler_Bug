rust
struct X<T>(T);

trait Hey {
    fn from_iter(_: i32) -> Self;
}

impl Hey for X<i32> {
    fn from_iter(x: i32) -> Self {
        X(x)
    }
}

fn main() {
    X::from_iter(1);
}
