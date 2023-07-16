
// Leave the following `use` lines commented out to see the suggestion

// The suggestion given, but doesn't work because inner is private
// use crate::outer::inner::Thing;

// This works because of the re-export, but isn't suggested at all
// use crate::outer::Thing;

fn main() {
    let x = Thing {};
}

pub mod outer {
    pub use self::inner::Thing;
    mod inner {
        pub struct Thing {}
    }
}
