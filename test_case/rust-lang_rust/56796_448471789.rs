plain
travis_time:end:00ef4094:start=1545193215939943394,finish=1545193284925386171,duration=68985442777
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:49:48] ....................i............................................................................... 2400/2928
[00:49:58] .................................................................................................... 2500/2928
[00:50:29] .................................................................................................... 2600/2928
[00:50:37] .................................................................................................... 2700/2928
[00:50:47] ..............F..................................................................................... 2800/2928
primary":true,"text":[{"text":"impl<T> Into<Box<T>> for Foo<T> {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(incoherent_fundamental_impls)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"conflicting implementation in crate `core`:\n- impl<T, U> std::convert::Into<U> for T\n  where U: std::convert::From<T>;","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"downstream crates may implement trait `std::convert::From<Foo<_>>` for type `std::boxed::Box<_>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: conflicting implementations of trait `std::convert::Into<std::boxed::Box<_>>` for type `Foo<_>`: (E0119)\n  --> /checkout/src/test/run-pass/try_from.rs:35:1\n   |\nLL | impl<T> Into<Box<T>> for Foo<T> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[deny(incoherent_fundamental_impls)] on by default\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:01] 
[00:51:01] 
[00:51:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:01] Build completed unsuccessfully in 0:09:28
[00:51:01] Build completed unsuccessfully in 0:09:28
[00:51:01] Makefile:58: recipe for target 'check' failed
[00:51:01] make: *** [check] Error 1
