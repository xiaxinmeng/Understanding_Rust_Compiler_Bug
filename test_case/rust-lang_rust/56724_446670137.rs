plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:095f8fe6
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:01:19]    Compiling arena v0.0.0 (/checkout/src/libarena)
[01:01:20]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[01:01:26]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[01:02:39]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[01:02:43] error[E0433]: failed to resolve: use of undeclared type or module `mem`
[01:02:43]     --> src/librustc/dep_graph/graph.rs:1128:35
[01:02:43]      |
[01:02:43] 1128 |                     debug_assert!(mem::size_of::<DepNodeIndex>() == 4);
[01:02:43]      |                                   ^^^ use of undeclared type or module `mem`
[01:02:55] error[E0308]: mismatched types
[01:02:55] error[E0308]: mismatched types
[01:02:55]     --> src/librustc/dep_graph/graph.rs:1133:53
[01:02:55]      |
[01:02:55] 1133 |                         let data1 = _mm_loadu_si128(ptr);
[01:02:55]      |                                                     ^^^ expected struct `std::arch::x86_64::__m128i`, found i32
[01:02:55]      |
[01:02:55]      = note: expected type `*const std::arch::x86_64::__m128i`
[01:02:55]                 found type `*const i32`
[01:02:55] error[E0308]: mismatched types
[01:02:55] error[E0308]: mismatched types
[01:02:55]     --> src/librustc/dep_graph/graph.rs:1134:53
[01:02:55]      |
[01:02:55] 1134 |                         let data2 = _mm_loadu_si128(ptr.offset(4));
[01:02:55]      |                                                     ^^^^^^^^^^^^^ expected struct `std::arch::x86_64::__m128i`, found i32
[01:02:55]      |
[01:02:55]      = note: expected type `*const std::arch::x86_64::__m128i`
[01:02:55]                 found type `*const i32`
[01:03:13] error: aborting due to 3 previous errors
[01:03:13] 
[01:03:13] Some errors occurred: E0308, E0433.
[01:03:13] For more information about an error, try `rustc --explain E0308`.
---
travis_time:end:0adc0a96:start=1544635278227143545,finish=1544635278234094379,duration=6950834
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:039564c6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:026802f0
travis_time:start:026802f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:038270b2
$ dmesg | grep -i kill
