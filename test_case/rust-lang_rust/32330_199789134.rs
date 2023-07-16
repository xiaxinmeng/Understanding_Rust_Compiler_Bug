 Rust
fn foo<'a, 'b>(_a: &'a str) -> &'b str where &'b (): Sized { "hello" }

fn main() {
    let f: fn(&str) -> &str = foo::<'static>;
}
