 rust
pub mod foo {}

fn main() {
    let foo {} = 0; // Trying to de-structure a module name causes the ICE.
}
