rust
macro_rules! m {
    () => { let created_local = 0; }
}
fn main() {
    m!();
    println!("{}", created_local); // resolution error
}
