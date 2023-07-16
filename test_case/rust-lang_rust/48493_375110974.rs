
fn main() {
    enum Void {}
    std::mem::forget(std::rc::Weak::<Void>::new());
}
