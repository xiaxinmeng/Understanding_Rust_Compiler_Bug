
$ "/home/aburka1/rust/rust/work-stepwise/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "1" "--target" "x86_64-unknown-linux-gnu" "--release" "--features" " jemalloc" "--manifest-path" "/home/aburka1/rust/rust/work-stepwise/src/rustc/Cargo.toml"                                                                                            Compiling rustc_bitflags v0.0.0 (file:///home/aburka1/rust/rust/work-stepwise/src/librustc_bitflags)                                                                                           error[E0554]: #[feature] may not be used on the beta release channel
  --> src/librustc_bitflags/lib.rs:13:1
   |
13 | #![feature(associated_consts)]                                                                                                                                                                  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: #[feature] may not be used on the beta release channel
  --> src/librustc_bitflags/lib.rs:14:1
   |
14 | #![feature(staged_api)]
   | ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors

error: Could not compile `rustc_bitflags`.
