plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:18fc6648
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:40:13]    Compiling tar v0.4.20
[01:40:14] error[E0308]: mismatched types
[01:40:14]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tar-0.4.20/src/pax.rs:26:15
[01:40:14]    |
[01:40:14] 26 |         data: a.split(is_newline),
[01:40:14]    |               ^^^^^^^^^^^^^^^^^^^ expected fn pointer, found fn item
[01:40:14]    |
[01:40:14]    = note: expected type `std::needle::ext::Split<&[u8], core::slice::needles::ElemSearcher<for<'r> fn(&'r u8) -> bool>>`
[01:40:14]               found type `std::needle::ext::Split<&[u8], core::slice::needles::ElemSearcher<for<'r> fn(&'r u8) -> bool {pax::pax_extensions::is_newline}>>`
[01:40:14] error: aborting due to previous error
[01:40:14] 
[01:40:14] For more information about this error, try `rustc --explain E0308`.
[01:40:14] error: Could not compile `tar`.
---
travis_time:end:10df6a8f:start=1554542409237312891,finish=1554542409248927428,duration=11614537
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:013c005c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:072fb8c0
travis_time:start:072fb8c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:26219f19
$ dmesg | grep -i kill
