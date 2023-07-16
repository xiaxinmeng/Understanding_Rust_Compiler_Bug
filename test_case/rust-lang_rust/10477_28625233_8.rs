 rust
#[crate_type="lib"];
mod foo {
    pub struct Bar; // not externally visible without the `pub use`
}
