rust
fn func(_: &dyn Fn(&i32) -> &i32) {}

fn main() {
    func(&|_: &i32| &42);
}
