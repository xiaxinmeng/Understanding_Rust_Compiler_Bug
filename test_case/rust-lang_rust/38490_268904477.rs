rust
mod foo {
    struct S;
    impl S {
        fn f(&self) {}
    }
    pub macro m() {
        S.f();
    }
}
... foo::m!(); ...
