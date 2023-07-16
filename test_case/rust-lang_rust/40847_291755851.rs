rust
macro m() {
    mod foo { // If we moved this module out of the macro, 
        pub fn f() {}
    }
}
m!();
foo::f(); // then this would resolve.
