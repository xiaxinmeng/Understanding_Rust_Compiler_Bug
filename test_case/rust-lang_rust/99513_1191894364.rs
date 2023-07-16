bash
# Create a library with a trait with one associated function
echo '
#![feature(no_core)]
#![no_core]
#![no_std]
pub trait A {
    fn trait_a_fn();
}
' > a.rs

# Create another library that depends on the first library
echo '
#![feature(no_core)]
#![no_core]
#![no_std]
pub use a::*;
' > b.rs

# Compile the library
rustc +nightly --edition=2021 --crate-type rlib a.rs

# Generate rustdoc JSON for both libraries
rustdoc +nightly --edition=2021 --extern a=liba.rlib -Z unstable-options --output-format json b.rs

# Missing "kind": "trait" item
cat doc/b.json | python3 -m json.tool
