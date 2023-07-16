 rust
#[prefix("foobar_")]
extern {
    fn func_1();
    fn func_2();
}
// expands to...
extern {
    fn foobar_func_1();
    fn foobar_func_2();
}
