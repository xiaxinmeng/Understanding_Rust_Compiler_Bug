plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:124757e2
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:39]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:49]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:59]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:45] error: unnecessary `unsafe` block
[00:08:45]   --> src/librustc/dep_graph/graph.rs:81:9
[00:08:45] 81 |         unsafe {
[00:08:45]    |         ^^^^^^ unnecessary `unsafe` block
[00:08:45]    |
[00:08:45]    = note: `-D unused-unsafe` implied by `-D warnings`
[00:08:45]    = note: `-D unused-unsafe` implied by `-D warnings`
[00:08:45] 
[00:08:45] error: unnecessary `unsafe` block
[00:08:45]   --> src/librustc/dep_graph/graph.rs:88:9
[00:08:45]    |
[00:08:45] 88 |         unsafe { DepNodeIndex { idx: value as usize } }
[00:08:45] 
[00:08:46] error: aborting due to 2 previous errors
[00:08:46] 
[00:08:46] error: Could not compile `rustc`.
---
travis_time:end:1e2cbf54:start=1545405949597706996,finish=1545405949605790339,duration=8083343
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0522dec6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:037c507b
travis_time:start:037c507b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:057749bf
$ dmesg | grep -i kill
