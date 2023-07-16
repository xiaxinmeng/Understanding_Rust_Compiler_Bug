 rust
extern {
    fn new<'a>() -> &'a X;
    fn delete<'a>(x: &'a X);
}
