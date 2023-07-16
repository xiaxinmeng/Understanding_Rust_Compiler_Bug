plain
travis_time:end:0b331f48:start=1549010267388568167,finish=1549010268330003662,duration=941435495
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:31]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:32] error: expected expression, found `i8`
[00:03:32]     --> src/libcore/num/mod.rs:1533:52
[00:03:32]      |
[00:03:32] 1533 |                   ((self ^ -1).wrapping_add(1), s == $SelfT::min_value())
[00:03:32] ...
[00:03:32] ...
[00:03:32] 2058 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
[00:03:32]      | |____________________________- in this macro invocation
[00:03:32] 
[00:03:32] error: expected expression, found `i16`
[00:03:32]     --> src/libcore/num/mod.rs:1533:52
[00:03:32]     --> src/libcore/num/mod.rs:1533:52
[00:03:32]      |
[00:03:32] 1533 |                   ((self ^ -1).wrapping_add(1), s == $SelfT::min_value())
[00:03:32] ...
[00:03:32] ...
[00:03:32] 2064 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:03:32] 2065 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]" }
[00:03:32] 
[00:03:32] error: expected expression, found `i32`
[00:03:32]     --> src/libcore/num/mod.rs:1533:52
[00:03:32]      |
[00:03:32]      |
[00:03:32] 1533 |                   ((self ^ -1).wrapping_add(1), s == $SelfT::min_value())
[00:03:32] ...
[00:03:32] ...
[00:03:32] 2070 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
[00:03:32] 2072 | |         "[0x12, 0x34, 0x56, 0x78]" }
[00:03:32]      | |____________________________________- in this macro invocation
[00:03:32] 
[00:03:32] error: expected expression, found `i64`
[00:03:32] error: expected expression, found `i64`
[00:03:32]     --> src/libcore/num/mod.rs:1533:52
[00:03:32]      |
[00:03:32] 1533 |                   ((self ^ -1).wrapping_add(1), s == $SelfT::min_value())
[00:03:32] ...
[00:03:32] ...
[00:03:32] 2077 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
[00:03:32] 2078 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
[00:03:32] 2079 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
[00:03:32]      | |_____________________________________________________________- in this macro invocation
[00:03:32] 
[00:03:32] error: expected expression, found `i128`
[00:03:32]     --> src/libcore/num/mod.rs:1533:52
[00:03:32]     --> src/libcore/num/mod.rs:1533:52
[00:03:32]      |
[00:03:32] 1533 |                   ((self ^ -1).wrapping_add(1), s == $SelfT::min_value())
[00:03:32] ...
[00:03:32] ...
[00:03:32] 2085 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
[00:03:32] 2086 | |         170141183460469231731687303715884105727, "", "", 16,
[00:03:32] 2088 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
[00:03:32] ...    |
[00:03:32] 2091 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
[00:03:32] 2092 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]" }
[00:03:32] 2092 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]" }
[00:03:32]      | |____________________________________________________________- in this macro invocation
[00:03:32] 
[00:03:32] error: expected expression, found `isize`
[00:03:32]     --> src/libcore/num/mod.rs:1533:52
[00:03:32]      |
[00:03:32] 1533 |                   ((self ^ -1).wrapping_add(1), s == $SelfT::min_value())
[00:03:32] ...
[00:03:32] ...
[00:03:32] 2113 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
[00:03:32] 2114 | |         12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
[00:03:32] 2115 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
[00:03:32]      | |_____________________________________________________________- in this macro invocation
[00:03:32] 
[00:03:36]    Compiling compiler_builtins v0.1.5
[00:03:36]    Compiling cmake v0.1.33
---
[00:03:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:49] expected success, got: exit code: 101
[00:03:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:49] Build completed unsuccessfully in 0:00:20
[00:03:49] make: *** [all] Error 1
[00:03:49] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:008a86ec
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 08:41:48 UTC 2019
---
travis_time:end:00136800:start=1549010509523914077,finish=1549010509528546641,duration=4632564
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0410ed53
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12dcc63b
travis_time:start:12dcc63b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23d19f40
$ dmesg | grep -i kill
