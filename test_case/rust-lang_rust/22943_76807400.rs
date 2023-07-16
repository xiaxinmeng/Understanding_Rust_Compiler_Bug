
------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/compile-fail/huge-array-simple.rs:14:8: 14:11 warning: unused variable: `fat`, #[warn(unused_variables)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/compile-fail/huge-array-simple.rs:14    let fat : [u8; (1<<61)+(1<<31)] = [0; (1u64<<61) as usize +(1u64<<31) as usize];
                                                                                                                    ^~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/compile-fail/huge-array-simple.rs:14:20: 14:25 error: bitshift exceeds the type's number of bits, #[deny(exceeding_bitshifts)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/compile-fail/huge-array-simple.rs:14    let fat : [u8; (1<<61)+(1<<31)] = [0; (1u64<<61) as usize +(1u64<<31) as usize];
                                                                                                                                ^~~~~
error: aborting due to previous error

------------------------------------------
