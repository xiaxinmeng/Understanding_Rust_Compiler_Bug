
warning: warn(rust_2018_idioms) incompatible with previous forbid
 --> src/lib.rs:4:5
  |
1 | #![forbid(unsafe_code, future_incompatible)]
  |                        ------------------- `forbid` level set here
...
4 |     rust_2018_idioms,
  |     ^^^^^^^^^^^^^^^^ overruled by previous forbid
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
