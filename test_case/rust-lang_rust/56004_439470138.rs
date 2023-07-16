plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:12840aeb
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:20:24]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:20:27] error[E0308]: mismatched types
[01:20:27]    --> librustdoc/config.rs:560:21
[01:20:27]     |
[01:20:27] 560 |     Ok(Externs::new(externs))
[01:20:27]     |                     |
[01:20:27]     |                     |
[01:20:27]     |                     expected enum `rustc_data_structures::sorted_map::HybridSortedMap`, found struct `std::collections::BTreeMap`
[01:20:27]     |                     help: try using a variant of the expected type: `rustc_data_structures::sorted_map::HybridSortedMap::Big(externs)`
[01:20:27]     |
[01:20:27]     = note: expected type `rustc_data_structures::sorted_map::HybridSortedMap<std::string::String, std::collections::BTreeSet<std::option::Option<std::string::String>>>`
[01:20:27]                found type `std::collections::BTreeMap<std::string::String, std::collections::BTreeSet<std::option::Option<std::string::String>>>`
[01:20:29] error: aborting due to previous error
[01:20:29] 
[01:20:29] For more information about this error, try `rustc --explain E0308`.
[01:20:29] error: Could not compile `rustdoc`.
---
travis_time:end:257cf00b:start=1542389564657194616,finish=1542389564665109730,duration=7915114
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:195d77d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02652a82
travis_time:start:02652a82
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f006ede
$ dmesg | grep -i kill
