rust
fn main() {
    let a: u32;
    never_returns();
    &a; // No error, even though `a` is not initialized, because this code never run
}
