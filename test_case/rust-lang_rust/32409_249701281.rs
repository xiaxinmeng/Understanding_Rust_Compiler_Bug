 rust
#![feature(pub_restricted)]
fn main() { }

mod far {
    mod outer {
        pub        use far::outer::space::hello_world;
        pub(crate) use far::outer::space::hello_crate;
        pub(super) use far::outer::space::hello_far;
        pub(self)  use far::outer::space::hello_outer;

        mod space {           
            pub        fn hello_world() {}
            pub(crate) fn hello_crate() {}
            pub(far)   fn hello_far()   {}
            pub(super) fn hello_outer() {}
        }

        mod bar {
            use super::hello_world;  // ok
            use super::hello_crate;  // ERROR: no `hello_crate` in `far::outer`
            use super::hello_far;    // ERROR: no `hello_far` in `far::outer`
            use super::hello_outer;  // ERROR: no `hello_outer` in `far::outer`
        }
    }
}
