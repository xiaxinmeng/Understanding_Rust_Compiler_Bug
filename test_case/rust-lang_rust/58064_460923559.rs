plain
travis_time:end:09301e51:start=1549437069516193859,finish=1549437070536253633,duration=1020059774
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:52]    |
[00:04:52] 23 | use collections::CollectionAllocErr;
[00:04:52]    |     ^^^^^^^^^^^ did you mean `crate::collections`?
[00:04:52]    |
[00:04:52]    = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:04:52] error[E0432]: unresolved import `raw_vec`
[00:04:52]   --> src/liballoc/collections/vec_deque.rs:24:5
[00:04:52]    |
[00:04:52] 24 | use raw_vec::RawVec;
[00:04:52] 24 | use raw_vec::RawVec;
[00:04:52]    |     ^^^^^^^ did you mean `crate::raw_vec`?
[00:04:52]    |
[00:04:52]    = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:04:52] error[E0432]: unresolved import `vec`
[00:04:52]   --> src/liballoc/collections/vec_deque.rs:25:5
[00:04:52]    |
[00:04:52] 25 | use vec::Vec;
[00:04:52] 25 | use vec::Vec;
[00:04:52]    |     ^^^ did you mean `crate::vec`?
[00:04:52]    |
[00:04:52]    = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:04:52] error: hidden lifetime parameters in types are deprecated
[00:04:52]    --> src/liballoc/collections/vec_deque.rs:971:45
[00:04:52]     |
[00:04:52]     |
[00:04:52] 971 |     pub fn drain<R>(&mut self, range: R) -> Drain<T>
[00:04:52]     |                                             ^^^^^^^^ help: indicate the anonymous lifetime: `Drain<'_, T>`
[00:04:52] 
[00:04:52] error[E0207]: the type parameter `B` is not constrained by the impl trait, self type, or predicates
[00:04:52]      |
[00:04:52]      |
[00:04:52] 2584 |         impl<A: $Bound, B> PartialEq<$Rhs> for $Lhs
[00:04:52]      |                         ^ unconstrained type parameter
[00:04:52] ...
[00:04:52] 2600 | __impl_slice_eq1! { VecDeque<A>, Vec<B> }
[00:04:52]      | ----------------------------------------- in this macro invocation
[00:04:52] error: aborting due to 5 previous errors
[00:04:52] 
[00:04:52] Some errors occurred: E0207, E0432.
[00:04:52] For more information about an error, try `rustc --explain E0207`.
[00:04:52] For more information about an error, try `rustc --explain E0207`.
[00:04:52] error: Could not compile `alloc`.
[00:04:52] warning: build failed, waiting for other jobs to finish...
[00:04:53] error: build failed
[00:04:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:53] expected success, got: exit code: 101
[00:04:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:53] Build completed unsuccessfully in 0:00:37
[00:04:53] Makefile:18: recipe for target 'all' failed
[00:04:53] make: *** [all] Error 1
46352 ./src/llvm-emscripten/test/MC
45344 ./src/test/ui
43744 ./src/llvm-project/compiler-rt
42332 ./src/llvm-project/llvm/lib/Target
---
travis_time:end:10d3b9c2:start=1549437375320322081,finish=1549437375326563135,duration=6241054
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02a61740
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01453848
travis_time:start:01453848
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5

