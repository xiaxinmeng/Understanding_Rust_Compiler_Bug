 rust
src/libstd/panic.rs:125:21: 125:32 error: use of deprecated item: renamed to `UnwindSafe`
src/libstd/panic.rs:125 impl<T: UnwindSafe> RecoverSafe for T {}
                                            ^~~~~~~~~~~
src/libstd/lib.rs:283:31: 283:39 note: lint level defined here
src/libstd/lib.rs:283 #![cfg_attr(not(stage0), deny(warnings))]
                                                    ^~~~~~~~
