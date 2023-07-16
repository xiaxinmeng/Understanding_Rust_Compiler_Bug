rust
fn foo(f: impl Fn(&i32)) {}

fn main() {
    let broken = |_| {};
    foo(broken);

    let works = |_: &_| {};
    foo(works);
}
