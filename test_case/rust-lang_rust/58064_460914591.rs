plain
travis_time:end:0fae29fc:start=1549433966052672717,finish=1549433967098480868,duration=1045808151
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:45]    |
[00:04:45] 23 | use collections::CollectionAllocErr;
[00:04:45]    |     ^^^^^^^^^^^ did you mean `crate::collections`?
[00:04:45]    |
[00:04:45]    = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:04:45] error[E0432]: unresolved import `raw_vec`
[00:04:45]   --> src/liballoc/collections/vec_deque.rs:24:5
[00:04:45]    |
[00:04:45] 24 | use raw_vec::RawVec;
[00:04:45] 24 | use raw_vec::RawVec;
[00:04:45]    |     ^^^^^^^ did you mean `crate::raw_vec`?
[00:04:45]    |
[00:04:45]    = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:04:45] error[E0432]: unresolved import `vec`
[00:04:45]   --> src/liballoc/collections/vec_deque.rs:25:5
[00:04:45]    |
[00:04:45] 25 | use vec::Vec;
[00:04:45] 25 | use vec::Vec;
[00:04:45]    |     ^^^ did you mean `crate::vec`?
[00:04:45]    |
[00:04:45]    = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:04:45] error: hidden lifetime parameters in types are deprecated
[00:04:45]    --> src/liballoc/collections/vec_deque.rs:971:45
[00:04:45]     |
[00:04:45]     |
[00:04:45] 971 |     pub fn drain<R>(&mut self, range: R) -> Drain<T>
[00:04:45]     |                                             ^^^^^^^^ help: indicate the anonymous lifetime: `Drain<'_, T>`
[00:04:45] 
[00:04:45] error[E0207]: the type parameter `B` is not constrained by the impl trait, self type, or predicates
[00:04:45]      |
[00:04:45]      |
[00:04:45] 2584 |         impl<A: $Bound, B> PartialEq<$Rhs> for $Lhs
[00:04:45]      |                         ^ unconstrained type parameter
[00:04:45] ...
[00:04:45] 2600 | __impl_slice_eq1! { VecDeque<A>, Vec<B> }
[00:04:45]      | ----------------------------------------- in this macro invocation
[00:04:45] error: aborting due to 5 previous errors
[00:04:45] 
[00:04:45] Some errors occurred: E0207, E0432.
[00:04:45] For more information about an error, try `rustc --explain E0207`.
[00:04:45] For more information about an error, try `rustc --explain E0207`.
[00:04:45] error: Could not compile `alloc`.
[00:04:45] warning: build failed, waiting for other jobs to finish...
[00:04:45] error: build failed
[00:04:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:45] expected success, got: exit code: 101
[00:04:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:45] Build completed unsuccessfully in 0:00:36
[00:04:45] Makefile:18: recipe for target 'all' failed
[00:04:45] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13faa6a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 06:24:22 UTC 2019
---
travis_time:end:0a376961:start=1549434263675296141,finish=1549434263681419512,duration=6123371
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0daf3112
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b2e3ac8
travis_time:start:0b2e3ac8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a85b4df
$ dmesg | grep -i kill
