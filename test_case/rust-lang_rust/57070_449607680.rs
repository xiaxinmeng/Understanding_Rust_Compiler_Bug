plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:11e1dac8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:39]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:40]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:50]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:00]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:04] error[E0425]: cannot find value `fingerprints` in this scope
[00:08:04]    --> src/librustc/dep_graph/graph.rs:512:44
[00:08:04]     |
[00:08:04] 512 |         if current_dep_graph.nodes.len() > fingerprints.len() {
[00:08:04]     |                                            ^^^^^^^^^^^^ help: try: `self.fingerprints`
[00:08:04] error[E0425]: cannot find value `fingerprints` in this scope
[00:08:04] error[E0425]: cannot find value `fingerprints` in this scope
[00:08:04]    --> src/librustc/dep_graph/graph.rs:513:13
[00:08:04]     |
[00:08:04] 513 |             fingerprints.resize(current_dep_graph.nodes.len(), Fingerprint::ZERO);
[00:08:04]     |             ^^^^^^^^^^^^ help: try: `self.fingerprints`
[00:08:04] error[E0425]: cannot find value `fingerprints` in this scope
[00:08:04] error[E0425]: cannot find value `fingerprints` in this scope
[00:08:04]    --> src/librustc/dep_graph/graph.rs:518:24
[00:08:04]     |
[00:08:04] 518 |             (dep_node, fingerprints[idx])
[00:08:04]     |                        ^^^^^^^^^^^^ help: try: `self.fingerprints`
[00:08:29] error: aborting due to 3 previous errors
[00:08:29] 
[00:08:29] For more information about this error, try `rustc --explain E0425`.
[00:08:29] error: Could not compile `rustc`.
---
travis_time:end:03e52ad8:start=1545527780357468614,finish=1545527780364368949,duration=6900335
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:236f3290
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25d9fa42
travis_time:start:25d9fa42
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12798bd0
$ dmesg | grep -i kill
