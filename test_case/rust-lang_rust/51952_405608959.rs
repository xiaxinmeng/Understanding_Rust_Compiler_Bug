rust
// Inside of Diesel:
#[macro_export]
macro_rules! __diesel_use_everything! {
    () => { pub use $crate::*: }
}

// In the derive
mod __dummy_mod {
    extern crate std;

    mod diesel {
        __diesel_use_everything!()
    }

    // derive code here
}
