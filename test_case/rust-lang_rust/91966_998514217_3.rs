rust
fn func(_: &dyn Fn(&i32) -> &i32) {}

fn main() {
    let x = 42;
    func(&|_: &i32| &x);
}
