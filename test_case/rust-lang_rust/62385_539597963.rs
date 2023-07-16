rust
struct Dummy;

impl From<fn(u32) -> String> for Dummy {
    fn from(_: fn(u32) -> String) -> Self { unimplemented!() }
}

fn foo(_: u32) -> String { unimplemented!() }

fn main() {
    Dummy::from(foo);
}
