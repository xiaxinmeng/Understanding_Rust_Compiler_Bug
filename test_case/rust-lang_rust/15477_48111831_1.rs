 rust
#![crate_type="lib"]
trait X<C: Y<C>> {}
trait Y<C: X<C>> {}
