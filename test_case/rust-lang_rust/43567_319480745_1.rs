rust
error[E0106]: missing lifetime specifier
 --> src/ffi/xproto.rs:8:16
  |
8 |             -> xcb_depth_iterator_t { unimplemented!() }
  |                ^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
  = help: consider giving it an explicit bounded or 'static lifetime
