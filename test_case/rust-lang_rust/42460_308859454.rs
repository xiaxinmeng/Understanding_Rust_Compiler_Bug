rust
enum E {
    V,
    U,
}

pub use E::*; // OK, but exports nothing, should be reported as unused import
