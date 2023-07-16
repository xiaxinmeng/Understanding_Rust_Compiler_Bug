

failures:

---- [compile-fail] compile-fail/packed-struct-generic-transmute.rs stdout ----

error: error pattern ' transmute called on types with different size' not found!
status: exit code: 101
command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-generic-transmute.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail/packed-struct-generic-transmute.stage2-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/packed-struct-generic-transmute.stage2-x86_64-apple-darwin --cfg rtopt -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-generic-transmute.rs:22:5: 22:11 warning: struct field is never used: `bar`, #[warn(dead_code)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-generic-transmute.rs:22     bar: T,
                                                                                                                               ^~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-generic-transmute.rs:23:5: 23:11 warning: struct field is never used: `baz`, #[warn(dead_code)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-generic-transmute.rs:23     baz: S
                                                                                                                               ^~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-generic-transmute.rs:34:38: 34:52 error: transmute called with differently sized types: Foo<[u8; 5], i32> (72 bits) to Oof<[u8; 5], i32> (96 bits) [E0512]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-generic-transmute.rs:34         let oof: Oof<[u8; 5], i32> = mem::transmute(foo);
                                                                                                                                                                ^~~~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------

thread '[compile-fail] compile-fail/packed-struct-generic-transmute.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/compiletest/runtest.rs:1501

---- [compile-fail] compile-fail/packed-struct-transmute.rs stdout ----

error: error pattern ' transmute called on types with different size' not found!
status: exit code: 101
command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-transmute.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail/packed-struct-transmute.stage2-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/packed-struct-transmute.stage2-x86_64-apple-darwin --cfg rtopt -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-transmute.rs:22:5: 22:12 warning: struct field is never used: `bar`, #[warn(dead_code)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-transmute.rs:22     bar: u8,
                                                                                                                       ^~~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-transmute.rs:23:5: 23:15 warning: struct field is never used: `baz`, #[warn(dead_code)] on by default
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-transmute.rs:23     baz: usize
                                                                                                                       ^~~~~~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-transmute.rs:35:24: 35:38 error: transmute called with differently sized types: Foo (72 bits) to Oof (128 bits) [E0512]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/packed-struct-transmute.rs:35         let oof: Oof = mem::transmute(foo);
                                                                                                                                          ^~~~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------

thread '[compile-fail] compile-fail/packed-struct-transmute.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/compiletest/runtest.rs:1501

---- [compile-fail] compile-fail/transmute-different-sizes.rs stdout ----

error: unexpected compiler error or warning: '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/transmute-different-sizes.rs:18:17: 18:26 error: transmute called with differently sized types: i16 (16 bits) to i8 (8 bits) [E0512]'
status: exit code: 101
command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/transmute-different-sizes.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail/transmute-different-sizes.stage2-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/transmute-different-sizes.stage2-x86_64-apple-darwin --cfg rtopt -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/transmute-different-sizes.rs:18:17: 18:26 error: transmute called with differently sized types: i16 (16 bits) to i8 (8 bits) [E0512]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/transmute-different-sizes.rs:18     let _: i8 = transmute(16i16);
                                                                                                                                     ^~~~~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/transmute-different-sizes.rs:23:17: 23:26 error: transmute called with differently sized types: &T (could be 64 bits) to i8 (could be 8 bits) [E0512]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/transmute-different-sizes.rs:23     let _: i8 = transmute(x);
                                                                                                                                     ^~~~~~~~~
error: aborting due to 2 previous errors

------------------------------------------

thread '[compile-fail] compile-fail/transmute-different-sizes.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/compiletest/runtest.rs:1501


failures:
    [compile-fail] compile-fail/packed-struct-generic-transmute.rs
    [compile-fail] compile-fail/packed-struct-transmute.rs
    [compile-fail] compile-fail/transmute-different-sizes.rs

