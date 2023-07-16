rust
mod m {
    struct A;
    struct B;
}

pub use m::*; // OK, but exports nothing, should be reported as unused import but doesn't currently
