rs
struct MyType;

impl From<&[u8]> for MyType {
    fn from(_: &[u8]) -> Self {
        Self
    }
}

fn takes_slice(_: &[u8]) {}

fn main() {
    takes_slice(b"");
    let _ = MyType::from(b""); //~ ERROR the trait bound `MyType: From<&[u8; 0]>` is not satisfied
}
