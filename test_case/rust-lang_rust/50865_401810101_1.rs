rust
// main
// aux-build:impl_trait_bug.rs

extern crate impl_trait_bug;

fn main() {
    impl_trait_bug::bar(()); // Error won't happen if bar is called from same crate
}
