rust
enum Uninhabited { }
fn never_returns() -> Uninhabited { panic!() }
fn main() {
    let a: u32;
    never_returns();
    &a; // ERROR borrow of possibly-uninitialized variable: `a`
}
