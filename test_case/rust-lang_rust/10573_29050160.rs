 rust
mod foo {
    struct Private;

    pub struct Public { not_priv: Private } // not ok
    pub struct PrivField { priv not_not_priv: Private } // ok

    fn private() -> Private {} // ok

    pub fn public() -> Private {} // not ok

    pub fn closure(_: |&Private| -> int) {} // not ok
}
