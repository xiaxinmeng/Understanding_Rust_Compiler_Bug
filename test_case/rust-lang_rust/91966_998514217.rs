rust
fn func(_: &dyn Fn(&i32) -> &i32) {}

fn main() {
    let f = |_: &i32| &42;
    func(&f);
}
