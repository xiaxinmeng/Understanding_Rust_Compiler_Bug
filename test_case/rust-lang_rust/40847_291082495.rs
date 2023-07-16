rust
macro m($foo:ident, $Bar:ident, $Baz:ident) {
    mod $foo {
        pub fn f() {}
    }
    trait $Bar {
        fn f() {}
    }
    struct $Baz;
    impl $Baz {
        pub fn f() {}
    }
}

m!(foo, Bar, Baz);
foo::f(); //< this doesn't resolve
Bar::f(); //< neither does this
Baz::f(); //< neither does this