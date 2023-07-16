rust
//main.rs
extern crate lib;

fn main() {
    lib::bar(lib::Bar); // Error won't happen if bar is called from same crate
}
