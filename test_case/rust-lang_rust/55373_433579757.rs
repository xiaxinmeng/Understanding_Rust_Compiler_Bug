plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0043a3d8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:52:58]    Compiling fnv v1.0.6
[01:52:58]    Compiling termcolor v1.0.2
[01:52:59]    Compiling vec_map v0.8.1
[01:52:59]    Compiling crossbeam v0.3.2
[01:52:59] error: `$T:ty` is followed (through repetition) by itself, which is not allowed for `ty` fragments
[01:52:59]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-0.3.2/src/cache_padded.rs:66:32
[01:52:59]    |
[01:52:59] 66 | macro_rules! zeros_valid { ($( $T:ty )*) => ($(
[01:52:59]    |                                ^^^^^ this fragment is followed by itself without a valid separator
[01:52:59]    |
[01:52:59]    = note: allowed there are: `,`, `{`, `[`, `=>`, `>`, `=`, `:`, `;`, `|`, `as` or `where`
[01:52:59] help: add a valid separator for the repetition to be unambiguous, for example
[01:52:59]    |
[01:52:59] 66 | macro_rules! zeros_valid { ($( $T:ty ),*) => ($(
[01:52:59] 
[01:52:59]    Compiling strsim v0.7.0
[01:53:00] error: aborting due to previous error
[01:53:00] 
[01:53:00] 
[01:53:00] error: Could not compile `crossbeam`.
[01:53:27] error: build failed
[01:53:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:53:27] expected success, got: exit code: 101
[01:53:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
---
travis_time:end:11a75024:start=1540603177940197502,finish=1540603177953028354,duration=12830852
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:137ce6bf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ed90b9e
travis_time:start:1ed90b9e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1048a786
$ dmesg | grep -i kill
