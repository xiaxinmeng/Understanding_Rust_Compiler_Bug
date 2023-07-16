 rust
    trait A { fn foo(&self) { } }
    struct S;
    struct T;
    impl A for S { }
    impl S {
        fn foo(&self) {
            impl T where Self: A { }
        }
    }
