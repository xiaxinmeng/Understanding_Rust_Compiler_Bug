 rust
use foo::bar::baz;

mod foo {
    use self::bar::baz;
    use foo::bar::qux;

    pub mod bar {
        pub fn baz() {}
        pub fn qux() {}
    }

    ...
}

...
