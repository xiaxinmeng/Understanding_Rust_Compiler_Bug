
pub mod FooMod {    // synthesized from the "class Foo" that the user typed
    ... public Rust API ...

    pub struct Foo {
        ... generated from the class that the user typed ...
    };

    pub mod imp { // internals, implementation
        ... gobject boilerplate ...

        static mut PRIV: FooClassPrivate = FooClassPrivate {
            ... zeros, later set with ONCE.call_once() ...
        }

        use super::super::*; // bring in the user's namespace, for their implementations

        impl super::Foo { // implement the *public* API here
            pub fn foobarize(&self, ...) {
                ... implementation method pasted from the user's code ... 
            }
        }
    }
}
pub use FooMod::*;
