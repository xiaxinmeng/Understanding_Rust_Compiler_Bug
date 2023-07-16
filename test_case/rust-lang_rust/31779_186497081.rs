 rust
mod foo {
    mod bar { // `bar` should only be visible inside `foo`
        pub use baz;
    }
}

pub mod baz {
    fn f() {}

    fn g() {
        ::foo::bar::baz::f(); //< yet this line compiles
    }
}
