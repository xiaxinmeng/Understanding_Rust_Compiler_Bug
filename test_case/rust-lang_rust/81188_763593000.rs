rust
error: any use of this value will cause an error
 --> hang.rs:2:55
  |
2 | pub const VALUES: [usize; std::u32::MAX as usize] = { todo!() };
  | ------------------------------------------------------^^^^^^^---
  |                                                       |
  |                                                       the evaluated program panicked at 'not yet implemented', hang.rs:2:55
  |
  = note: `#[deny(const_err)]` on by default
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info
