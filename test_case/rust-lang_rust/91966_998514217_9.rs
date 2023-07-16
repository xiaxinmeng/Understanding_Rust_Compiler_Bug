rust
fn func(_: impl Fn(&i32) -> &i32) {}

fn main() {
    let f = |i: &i32| i;
    func(f);
}
