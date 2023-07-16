 rust
#[feature(macro_rules)]

macro_rules! foo (
    () => (
        struct Foo;
    )
)

fn main() {
    foo!()
}
