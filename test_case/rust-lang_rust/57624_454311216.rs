plain
[01:22:06]    Compiling cargo_metadata v0.6.2
[01:22:26]    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[01:22:26] [RUSTC-TIMING] toml test:false 20.014
[01:22:28] [RUSTC-TIMING] cargo_metadata test:false 22.053
[01:22:29] error[E0432]: unresolved import `syntax::tokenstream::ThinTokenStream`
[01:22:29]    |
[01:22:29]    |
[01:22:29] 17 | use syntax::tokenstream::{ThinTokenStream, TokenStream};
[01:22:29]    |                           ^^^^^^^^^^^^^^^ no `ThinTokenStream` in `tokenstream`. Did you mean to use `TokenStream`?
[01:22:29] 
[01:22:31] error[E0432]: unresolved import `syntax::tokenstream::ThinTokenStream`
[01:22:31]    |
[01:22:31]    |
[01:22:31] 17 | use syntax::tokenstream::{ThinTokenStream, TokenStream};
[01:22:31]    |                           ^^^^^^^^^^^^^^^ no `ThinTokenStream` in `tokenstream`. Did you mean to use `TokenStream`?
[01:22:33] error: aborting due to previous error
[01:22:33] 
[01:22:33] For more information about this error, try `rustc --explain E0432`.
[01:22:33] error: Could not compile `clippy_lints`.
---
[01:26:53]    Compiling curl v0.4.19
[01:26:58] [RUSTC-TIMING] rustc_data_structures test:false 6.076
[01:26:59] [RUSTC-TIMING] rustfix test:false 6.674
[01:26:59]    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[01:27:04] error[E0432]: unresolved import `syntax::tokenstream::ThinTokenStream`
[01:27:04]    |
[01:27:04]    |
[01:27:04] 17 | use syntax::tokenstream::{ThinTokenStream, TokenStream};
[01:27:04]    |                           ^^^^^^^^^^^^^^^ no `ThinTokenStream` in `tokenstream`. Did you mean to use `TokenStream`?
[01:27:04]    Compiling rand_chacha v0.1.0
[01:27:05] [RUSTC-TIMING] ignore test:false 32.632
[01:27:06]    Compiling rand_pcg v0.1.1
[01:27:06]    Compiling rand v0.6.1
---
travis_time:end:21960ca1:start=1547541927863218265,finish=1547541927870996669,duration=7778404
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0db14231
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:013c8f7e
travis_time:start:013c8f7e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2212edc5
$ dmesg | grep -i kill
