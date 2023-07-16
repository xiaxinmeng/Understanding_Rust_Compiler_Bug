
#[inline]
pub fn foo() -> fn() {
    fn bar() {}

   bar
}
