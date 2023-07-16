rust
pub fn foo(callback: &'static Fn() -> ToString) {
    let mut x: Option<Box<Fn() -> ToString>> = None;
    x = Some(Box::new(callback));
}
