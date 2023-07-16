
impl Trait for Type {
    fn f() {} // <-- ImplItem
}
impl Type {
    fn g() {} // <-- ImplItem
}
extern "C" {
    fn printf(); // <-- ForeignItem
}
