
error[E0599]: no function or associated item named `emit_struct` found for trait object `dyn Encodable` in the current scope
  --> /.../.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.1.40/src/biguint.rs:43:48
   |
43 | #[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
   |                                                ^^^^^^^^^^^^^^ function or associated item not found in `dyn Encodable`
   |
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
