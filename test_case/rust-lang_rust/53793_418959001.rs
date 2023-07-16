plain
[00:47:29] ....................................................................................................
[00:47:32] ....................................................................................................
[00:47:34] ...............................i....................................................................
[00:47:37] ....................................................................................................
[00:47:40] ................................................................................iiiiiiiii...........
[00:47:45] ..ii................................................................................................
[00:47:49] ....................................................................................................
[00:47:51] .............................................................i......................................
[00:47:54] ....................................................................................................
---
[01:14:57] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0691 (line 11103) ... ok
[01:14:57] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0691 (line 11111) ... ok
[01:14:57] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0695 (line 11186) ... ok
[01:14:57] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0692 (line 11144) ... ok
[01:14:57] test /checkout/obj/build/x86_64-unknown-linux-7] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0309 (line 4764) stdout ----
[01:14:57] error[E0576]: cannot find associated type `Out` in trait `SomeTrait`
[01:14:57]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:4769:32
[01:14:57]   |
[01:14:57] 7 |     foo: <T as SomeTrait<'a>>::Out
[01:14:57]   |                                ^^^ not found in `SomeTrait`
[01:14:57] 
[01:14:57] error[E0046]: not all trait items implemented, missing: `Output`
[01:14:57]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:4776:1
[01:14:57] 11 |       type Output;
[01:14:57] 11 |       type Output;
[01:14:57]    |       ------------ `Output` from trait
[01:14:57] ...
[01:14:57] 14 | / impl<'a, T> SomeTrait<'a> for T
[01:14:57] 15 | | where
[01:14:57] 16 | |     T: 'a,
[01:14:57] 17 | | { }
[01:14:57]    | |___^ missing `Output` in implementation
[01:14:57] 
[01:14:57] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0309 (line 4764)' panicked at 'Some expected error codes were not found: ["E0309"]', librustdoc/test.rs:338:9
[01:14:57] 
[01:14:57] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0309 (line 4786) stdout ----
[01:14:57] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0309 (line 4786) stdout ----
[01:14:57] error[E0576]: cannot find associated type `Out` in trait `SomeTrait`
[01:14:57]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:4791:32
[01:14:57]   |
[01:14:57] 7 |     foo: <T as SomeTrait<'a>>::Out
travis_time:end:0319b4e4:start=1536202726597816542,finish=1536207224299014421,duration=4497701197879

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:014fa372
---
travis_time:end:221ececd:start=1536207226131903137,finish=1536207226139226282,duration=7323145
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13a6f7f6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0537bdf1
travis_time:start:0537bdf1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01d3265a
$ dmesg | grep -i kill
