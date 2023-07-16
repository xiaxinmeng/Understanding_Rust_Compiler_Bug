rust
mod secret {
    enum Uninhabited { }
    pub struct Wrapper { u: Uninhabited }
    pub fn never_returns() -> Wrapper { panic!() }
}
fn main() {
    let a: u32;
    secret::never_returns();
    &a; // ERROR
}
