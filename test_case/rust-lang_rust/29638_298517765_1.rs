sh
set -e
cargo new aa
cargo new bb
cargo new cc
echo 'bb = {path = "../bb"}' >> aa/Cargo.toml
echo 'cc = {path = "../cc"}' >> bb/Cargo.toml

echo '#[macro_use] extern crate bb;' > aa/src/lib.rs
echo '#[test] fn a() { assert_eq!(c!(), 42) }' >> aa/src/lib.rs

echo '#[macro_use] extern crate cc;' > bb/src/lib.rs
echo 'pub use cc::*;' >> bb/src/lib.rs

echo '#[macro_export] macro_rules! c { () => { 42 } }' > cc/src/lib.rs

(cd aa && cargo +1.15.0 test)
rm -r aa bb cc
