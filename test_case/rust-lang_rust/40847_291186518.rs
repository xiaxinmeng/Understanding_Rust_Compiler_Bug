rust
mod foo {
    pub fn f() {} // (i)
}
trait Tr {
    fn f() {} // (ii)
}
macro m($t:ty) {
    foo::f(); // This always resolves to (i), no matter where the macro is invoked.
    Tr::f(); // This always resolves to (ii)
    impl Tr for $t {
        fn f() {} // This always refers to (ii)
    }
}
