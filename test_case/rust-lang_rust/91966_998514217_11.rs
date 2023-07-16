rust
fn func1(_: impl Fn(&i32) -> &i32) {}
fn func2(_: &impl Fn(&i32) -> &i32) {}
fn func3(_: &dyn Fn(&i32) -> &i32) {}

fn main() {
    func1(|i: &i32| i);
    func2(&|i: &i32| i);
    func3(&|i: &i32| i);
}
