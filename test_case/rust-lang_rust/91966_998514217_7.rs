rust
fn func(_: impl Fn(&i32) -> &i32) {}

fn main() {
    let x = 42;
    let f = |_: &i32| &x;
    func(f);
}
