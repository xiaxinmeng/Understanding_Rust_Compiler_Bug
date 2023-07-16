Rust
fn main() {
    let a = &5;
    *a = 6; // no MIR error !!!
}
