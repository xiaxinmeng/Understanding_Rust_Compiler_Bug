plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:105dd9d1
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:14:56]    Compiling unicase v2.3.0
[01:14:57]    Compiling owning_ref v0.3.3
[01:14:57]    Compiling minifier v0.0.30
[01:14:57]    Compiling smallvec v0.6.7
[01:14:57] error: local ambiguity: multiple parsing options: built-in NTs expr ('else_cond') or 1 other option.
[01:14:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.30/src/css/token.rs:564:17
[01:14:57]     |
[01:14:57] 564 |                 let Ok(op) = Operator::try_from(c) => {
[01:14:57] 
[01:14:57] error: aborting due to previous error
[01:14:57] 
[01:14:57] error: Could not compile `minifier`.
---
travis_time:end:0b2418a8:start=1557955996767683942,finish=1557955996782308675,duration=14624733
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e748ea7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10ee3136
travis_time:start:10ee3136
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0dd3e628
$ dmesg | grep -i kill
