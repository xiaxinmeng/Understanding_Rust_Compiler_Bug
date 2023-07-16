plain
[01:03:50] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:50]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:20:29
[01:03:53]    |
[01:03:53] 20 |     let (a, b, c) = (&5 as &Any, &TEST as &Any, &Test as &Any);
[01:03:53]    |                             ^^^ help: use `dyn`: `dyn Any`
[01:03:53]    |
[01:03:53]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:20:44
[01:03:53]    |
[01:03:53] 20 |     let (a, b, c) = (&5 as &Any, &TEST as &Any, &Test as &Any);
[01:03:53]    |                                            ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:20:59
[01:03:53]    |
[01:03:53] 20 |     let (a, b, c) = (&5 as &Any, &TEST as &Any, &Test as &Any);
[01:03:53]    |                                                           ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:37:41
[01:03:53]    |
[01:03:53] 37 |     let (a, b, c) = (box 5_usize as Box<Any>, box TEST as Box<Any>, box Test as Box<Any>);
[01:03:53]    |                                         ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:37:63
[01:03:53]    |
[01:03:53] 37 |     let (a, b, c) = (box 5_usize as Box<Any>, box TEST as Box<Any>, box Test as Box<Any>);
[01:03:53]    |                                                               ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:37:85
[01:03:53]    |
[01:03:53] 37 |     let (a, b, c) = (box 5_usize as Box<Any>, box TEST as Box<Any>, box Test as Box<Any>);
[01:03:53]    |                                                                                     ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:54:26
[01:03:53]    |
[01:03:53] 54 |     let a = &5_usize as &Any;
[01:03:53]    |                          ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:72:30
[01:03:53]    |
[01:03:53] 72 |     let a_r = &mut a as &mut Any;
[01:03:53]    |                              ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/any.rs:74:27
[01:03:53]    |
[01:03:53] 74 |     let b_r = tmp as &mut Any;
[01:03:53]    |                           ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/any.rs:116:26
[01:03:53]     |
[01:03:53] 116 |     let test = &test as &Any;
[01:03:53]     |                          ^^^ help: use `dyn`: `dyn Any`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]     |
[01:03:53]     |
[01:03:53] 131 |         let mut indirect_hasher: &mut Hasher = &mut hasher;
[01:03:53]     |                                       ^^^^^^ help: use `dyn`: `dyn Hasher`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    |
[01:03:53]    |
[01:03:53] 25 |     struct X(str); struct Y(Z + 'static);
[01:03:53]    |                             ^^^^^^^^^^^ help: use `dyn`: `dyn Z + 'static`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/mem.rs:112:33
[01:03:53]     |
[01:03:53] 112 |     let a = box 100isize as Box<Foo>;
[01:03:53]     |                                 ^^^ help: use `dyn`: `dyn Foo`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/mem.rs:116:21
[01:03:53]     |
[01:03:53] 116 |         let _x: Box<Foo> = transmute(x);
[01:03:53]     |                     ^^^ help: use `dyn`: `dyn Foo`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]     |
[01:03:53]     |
[01:03:53] 243 |     let mut functions: [Box<Fn() -> Option<()>>; 3] =
[01:03:53]     |                             ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Fn() -> Option<()>`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/ptr.rs:87:20
[01:03:53]    |
[01:03:53] 87 |     let ci: *const ToString = &3;
[01:03:53]    |                    ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/ptr.rs:90:18
[01:03:53]    |
[01:03:53] 90 |     let mi: *mut ToString = &mut 3;
[01:03:53]    |                  ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/ptr.rs:93:21
[01:03:53]    |
[01:03:53] 93 |     let nci: *const ToString = null::<isize>();
[01:03:53]    |                     ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]   --> libcore/../libcore/tests/ptr.rs:96:19
[01:03:53]    |
[01:03:53] 96 |     let nmi: *mut ToString = null_mut::<isize>();
[01:03:53]    |                   ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/ptr.rs:143:24
[01:03:53]     |
[01:03:53] 143 |         let ci: *const ToString = &3;
[01:03:53]     |                        ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/ptr.rs:146:22
[01:03:53]     |
[01:03:53] 146 |         let mi: *mut ToString = &mut 3;
[01:03:53]     |                      ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/ptr.rs:149:25
[01:03:53]     |
[01:03:53] 149 |         let nci: *const ToString = null::<isize>();
[01:03:53]     |                         ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/ptr.rs:152:23
[01:03:53]     |
[01:03:53] 152 |         let nmi: *mut ToString = null_mut::<isize>();
[01:03:53]     |                       ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/ptr.rs:185:22
[01:03:53]     |
[01:03:53] 185 |         let mi: *mut ToString = &mut 3;
[01:03:53]     |                      ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    --> libcore/../libcore/tests/ptr.rs:188:23
[01:03:53]     |
[01:03:53] 188 |         let nmi: *mut ToString = null_mut::<isize>();
[01:03:53]     |                       ^^^^^^^^ help: use `dyn`: `dyn ToString`
[01:03:53] 
[01:03:53] error: trait objects without an explicit `dyn` are deprecated
[01:03:53]    |
[01:03:53]    |
[01:03:53] 84 |     let mut functions: [Box<Fn() -> Result<(), isize>>; 3] =
[01:03:53]    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Fn() -> Result<(), isize>`
[01:04:05] error: aborting due to 26 previous errors
[01:04:05] 
[01:04:05] error: Could not compile `core`.
[01:04:05] 
[01:04:05] 
[01:04:05] To learn more, run the command again with --verbose.
[01:04:05] 
[01:04:05] 
[01:04:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:04:05] 
[01:04:05] 
[01:04:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:05] Build completed unsuccessfully in 0:18:32
[01:04:05] Build completed unsuccessfully in 0:18:32
[01:04:05] make: *** [check] Error 1
[01:04:05] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09483ae0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06875a14:start=1531559483544392419,finish=1531559483552747059,duration=8354640
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0471dac8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d5be610
travis_time:start:0d5be610
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0029e4f6
$ dmesg | grep -i kill
