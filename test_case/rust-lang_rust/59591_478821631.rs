plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:09718e33
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:44:51]    Compiling clap v2.32.0
[01:44:51] error[E0308]: mismatched types
[01:44:51]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tar-0.4.20/src/pax.rs:26:15
[01:44:51]    |
[01:44:51] 26 |         data: a.split(is_newline),
[01:44:51]    |               ^^^^^^^^^^^^^^^^^^^ expected fn pointer, found fn item
[01:44:51]    |
[01:44:51]    = note: expected type `std::needle::ext::Split<&[u8], core::slice::needles::ElemSearcher<for<'r> fn(&'r u8) -> bool>>`
[01:44:51]               found type `std::needle::ext::Split<&[u8], core::slice::needles::ElemSearcher<for<'r> fn(&'r u8) -> bool {pax::pax_extensions::is_newline}>>`
[01:44:51] error: aborting due to previous error
[01:44:51] 
[01:44:51] For more information about this error, try `rustc --explain E0308`.
[01:44:51] error: Could not compile `tar`.
---
travis_time:end:0faf80fa:start=1554172420518870241,finish=1554172420532343420,duration=13473179
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0916b180
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04e78cce
travis_time:start:04e78cce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a4060dc
$ dmesg | grep -i kill
