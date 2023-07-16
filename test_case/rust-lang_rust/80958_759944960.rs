
error: use of module `core::i64` that will be deprecated in a future Rust version: all constants in this module replaced by associated constants on `i64`
 --> library/std/src/sys/unix/process/zircon.rs:4:5
4 | use crate::i64;
  |     ^^^^^^^^^^
  |
  |
  = note: `-D deprecated-in-future` implied by `-D warnings`

error: use of constant `core::i64::MAX` that will be deprecated in a future Rust version: replaced by the `MAX` associated constant on this type
  --> library/std/src/sys/unix/process/zircon.rs:19:41
   |
19 | pub const ZX_TIME_INFINITE: zx_time_t = i64::MAX;
