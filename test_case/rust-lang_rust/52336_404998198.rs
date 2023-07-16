plain
[00:59:17] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:17]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:20:29
[00:59:20]    |
[00:59:20] 20 |     let (a, b, c) = (&5 as &Any, &TEST as &Any, &Test as &Any);
[00:59:20]    |                             ^^^ help: use `dyn`: `dyn Any`
[00:59:20]    |
[00:59:20]    = note: requested on the command line with `-D bare-trait-objects`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:20:44
[00:59:20]    |
[00:59:20] 20 |     let (a, b, c) = (&5 as &Any, &TEST as &Any, &Test as &Any);
[00:59:20]    |                                            ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:20:59
[00:59:20]    |
[00:59:20] 20 |     let (a, b, c) = (&5 as &Any, &TEST as &Any, &Test as &Any);
[00:59:20]    |                                                           ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:37:41
[00:59:20]    |
[00:59:20] 37 |     let (a, b, c) = (box 5_usize as Box<Any>, box TEST as Box<Any>, box Test as Box<Any>);
[00:59:20]    |                                         ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:37:63
[00:59:20]    |
[00:59:20] 37 |     let (a, b, c) = (box 5_usize as Box<Any>, box TEST as Box<Any>, box Test as Box<Any>);
[00:59:20]    |                                                               ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:37:85
[00:59:20]    |
[00:59:20] 37 |     let (a, b, c) = (box 5_usize as Box<Any>, box TEST as Box<Any>, box Test as Box<Any>);
[00:59:20]    |                                                                                     ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:54:26
[00:59:20]    |
[00:59:20] 54 |     let a = &5_usize as &Any;
[00:59:20]    |                          ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:72:30
[00:59:20]    |
[00:59:20] 72 |     let a_r = &mut a as &mut Any;
[00:59:20]    |                              ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/any.rs:74:27
[00:59:20]    |
[00:59:20] 74 |     let b_r = tmp as &mut Any;
[00:59:20]    |                           ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/any.rs:116:26
[00:59:20]     |
[00:59:20] 116 |     let test = &test as &Any;
[00:59:20]     |                          ^^^ help: use `dyn`: `dyn Any`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]     |
[00:59:20]     |
[00:59:20] 131 |         let mut indirect_hasher: &mut Hasher = &mut hasher;
[00:59:20]     |                                       ^^^^^^ help: use `dyn`: `dyn Hasher`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    |
[00:59:20]    |
[00:59:20] 25 |     struct X(str); struct Y(Z + 'static);
[00:59:20]    |                             ^^^^^^^^^^^ help: use `dyn`: `dyn Z + 'static`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/mem.rs:112:33
[00:59:20]     |
[00:59:20] 112 |     let a = box 100isize as Box<Foo>;
[00:59:20]     |                                 ^^^ help: use `dyn`: `dyn Foo`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/mem.rs:116:21
[00:59:20]     |
[00:59:20] 116 |         let _x: Box<Foo> = transmute(x);
[00:59:20]     |                     ^^^ help: use `dyn`: `dyn Foo`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]     |
[00:59:20]     |
[00:59:20] 243 |     let mut functions: [Box<Fn() -> Option<()>>; 3] =
[00:59:20]     |                             ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Fn() -> Option<()>`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/ptr.rs:87:20
[00:59:20]    |
[00:59:20] 87 |     let ci: *const ToString = &3;
[00:59:20]    |                    ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/ptr.rs:90:18
[00:59:20]    |
[00:59:20] 90 |     let mi: *mut ToString = &mut 3;
[00:59:20]    |                  ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/ptr.rs:93:21
[00:59:20]    |
[00:59:20] 93 |     let nci: *const ToString = null::<isize>();
[00:59:20]    |                     ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]   --> libcore/../libcore/tests/ptr.rs:96:19
[00:59:20]    |
[00:59:20] 96 |     let nmi: *mut ToString = null_mut::<isize>();
[00:59:20]    |                   ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/ptr.rs:143:24
[00:59:20]     |
[00:59:20] 143 |         let ci: *const ToString = &3;
[00:59:20]     |                        ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/ptr.rs:146:22
[00:59:20]     |
[00:59:20] 146 |         let mi: *mut ToString = &mut 3;
[00:59:20]     |                      ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/ptr.rs:149:25
[00:59:20]     |
[00:59:20] 149 |         let nci: *const ToString = null::<isize>();
[00:59:20]     |                         ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/ptr.rs:152:23
[00:59:20]     |
[00:59:20] 152 |         let nmi: *mut ToString = null_mut::<isize>();
[00:59:20]     |                       ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/ptr.rs:185:22
[00:59:20]     |
[00:59:20] 185 |         let mi: *mut ToString = &mut 3;
[00:59:20]     |                      ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    --> libcore/../libcore/tests/ptr.rs:188:23
[00:59:20]     |
[00:59:20] 188 |         let nmi: *mut ToString = null_mut::<isize>();
[00:59:20]     |                       ^^^^^^^^ help: use `dyn`: `dyn ToString`
[00:59:20] 
[00:59:20] error: trait objects without an explicit `dyn` are deprecated
[00:59:20]    |
[00:59:20]    |
[00:59:20] 84 |     let mut functions: [Box<Fn() -> Result<(), isize>>; 3] =
[00:59:20]    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Fn() -> Result<(), isize>`
[00:59:30] error: aborting due to 26 previous errors
[00:59:30] 
[00:59:30] error: Could not compile `core`.
[00:59:30] 
[00:59:30] 
[00:59:30] To learn more, run the command again with --verbose.
[00:59:30] 
[00:59:30] 
[00:59:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[00:59:30] 
[00:59:30] 
[00:59:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:30] Build completed unsuccessfully in 0:17:15
[00:59:30] Build completed unsuccessfully in 0:17:15
[00:59:30] Makefile:58: recipe for target 'check' failed
[00:59:30] make: *** [check] Error 1
129508 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123072 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
122496 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
116144 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
---
travis_time:end:2f166500:start=1531543090945426092,finish=1531543091067797839,duration=122371747
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:273cebf2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2e552984
$ dmesg | grep -i kill
