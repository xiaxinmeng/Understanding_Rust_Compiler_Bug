 rust
macro_rules! foo( () => (mod foo { fn bar() -> Option<int> { Some(3) } }) )

foo!()

fn main() {
    foo::bar();
}
