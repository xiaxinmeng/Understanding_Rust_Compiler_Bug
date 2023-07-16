plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: variable does not need to be mutable
   --> src/librustdoc/clean/utils.rs:238:17
    |
238 |             let mut s = if let Some(def) = def.as_local() {
    |                 |
    |                 help: remove this `mut`
    |
    |
    = note: `-D unused-mut` implied by `-D warnings`
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:23
