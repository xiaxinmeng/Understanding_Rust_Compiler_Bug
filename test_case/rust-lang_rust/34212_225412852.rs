 rust
pub mod module1; // it would not be available in this module on stable
#[macro_use] mod foo; // unless this were moved above `module1`
