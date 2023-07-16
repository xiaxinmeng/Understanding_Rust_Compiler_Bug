plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0a072a42
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:03:53]    Compiling toml v0.4.6
[00:03:53]    Compiling serde_json v1.0.31
[00:04:01]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:04:34]     Finished dev [unoptimized] target(s) in 1m 06s
[00:04:34] failed to parse TOML configuration 'config.toml': invalid type: string "1", expected u32 for key `rust.codegen-units-std`
[00:04:34] Build completed unsuccessfully in 0:01:28
[00:04:34] make: *** [prepare] Error 1
[00:04:35] Command failed. Attempt 2/5:
[00:04:35]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:35]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:35] failed to parse TOML configuration 'config.toml': invalid type: string "1", expected u32 for key `rust.codegen-units-std`
[00:04:35] Build completed unsuccessfully in 0:00:00
[00:04:35] make: *** [prepare] Error 1
[00:04:37] Command failed. Attempt 3/5:
[00:04:37]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:37]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:37] failed to parse TOML configuration 'config.toml': invalid type: string "1", expected u32 for key `rust.codegen-units-std`
[00:04:37] Build completed unsuccessfully in 0:00:00
[00:04:37] make: *** [prepare] Error 1
[00:04:40] Command failed. Attempt 4/5:
[00:04:41]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:41]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:41] failed to parse TOML configuration 'config.toml': invalid type: string "1", expected u32 for key `rust.codegen-units-std`
[00:04:41] Build completed unsuccessfully in 0:00:00
[00:04:41] make: *** [prepare] Error 1
[00:04:45] Command failed. Attempt 5/5:
[00:04:45]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:45]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:45] failed to parse TOML configuration 'config.toml': invalid type: string "1", expected u32 for key `rust.codegen-units-std`
[00:04:45] Build completed unsuccessfully in 0:00:00
[00:04:45] make: *** [prepare] Error 1
[00:04:45] The command has failed after 5 attempts.
travis_time:end:0c1d07fc:start=1540297112928164652,finish=1540297398733819004,duration=285805654352
---
travis_time:end:14670954:start=1540297399169557079,finish=1540297399177338390,duration=7781311
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2adff231
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07e41598
travis_time:start:07e41598
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02dd237c
$ dmesg | grep -i kill
