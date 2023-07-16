plain
[00:04:58]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:59]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:05:11]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:05:11]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:05:12] error[E0252]: the name `Unpin` is defined multiple times
[00:05:12]    --> liballoc/pin.rs:104:20
[00:05:12]     |
[00:05:12] 99  | pub use core::marker::Unpin;
[00:05:12]     |         ------------------- previous import of the trait `Unpin` here
[00:05:12] ...
[00:05:12] 104 | use core::marker::{Unpin, Unsize};
[00:05:12]     |                    ^^^^^ `Unpin` reimported here
[00:05:12]     |
[00:05:12]     = note: `Unpin` must be defined only once in the type namespace of this module
[00:05:12] help: You can use `as` to change the binding name of the import
[00:05:12]     |
[00:05:12] 104 | use core::marker::{Unpin as OtherUnpin, Unsize};
[00:05:12] 
[00:05:12] warning: unused import: `Unpin`
[00:05:12] warning: unused import: `Unpin`
[00:05:12]    --> liballoc/pin.rs:104:20
[00:05:12] error: aborting due to previous error
[00:05:14] 
[00:05:14] For more information about this error, try `rustc --explain E0252`.
[00:05:14] error: Could not compile `alloc`.
[00:05:14] error: Could not compile `alloc`.
[00:05:14] 
[00:05:14] Caused by:
[00:05:14]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=269c5b575a825095 -C extra-filename=-269c5b575a825095 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-c1b9aa9d5447cc92.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-bd44783aadae9ca1.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ed6515dfcadbf7bb/out` (exit code: 1)
travis_time:end:109449de:start=1534854465387649075,finish=1534854780498112193,duration=315110463118

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:000e2c92
---
travis_time:end:0442c8cf:start=1534854781007329342,finish=1534854781017125335,duration=9795993
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04c397dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b7e3066
travis_time:start:0b7e3066
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:20d51720
$ dmesg | grep -i kill
