plain
travis_time:end:0c53cb56:start=1549808284890182275,finish=1549808385897884384,duration=101007702109
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:11:49]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:11:51] error[E0432]: unresolved import `const_eval`
[00:11:51]   --> src/librustc_mir/interpret/intern.rs:21:5
[00:11:51]    |
[00:11:51] 21 | use const_eval::{CompileTimeInterpreter, CompileTimeEvalContext};
[00:11:51]    |     ^^^^^^^^^^ did you mean `crate::const_eval`?
[00:11:51]    |
[00:11:51]    = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:11:51] 
[00:11:54] error[E0392]: parameter `'a` is never used
[00:11:54]   --> src/librustc_mir/interpret/intern.rs:23:27
[00:11:54]    |
[00:11:54] 23 | struct InternVisitor<'rt, 'a: 'rt, 'mir: 'rt, 'tcx: 'a+'rt+'mir> {
[00:11:54]    |                           ^^ unused type parameter
[00:11:54]    |
[00:11:54]    = help: consider removing `'a` or using a marker such as `std::marker::PhantomData`
[00:11:54] 
[00:11:54] error[E0392]: parameter `'mir` is never used
[00:11:54]   --> src/librustc_mir/interpret/intern.rs:23:36
[00:11:54]    |
[00:11:54] 23 | struct InternVisitor<'rt, 'a: 'rt, 'mir: 'rt, 'tcx: 'a+'rt+'mir> {
[00:11:54]    |                                    ^^^^ unused type parameter
[00:11:54]    |
[00:11:54]    = help: consider removing `'mir` or using a marker such as `std::marker::PhantomData`
[00:11:55] error: aborting due to 3 previous errors
[00:11:55] 
[00:11:55] Some errors occurred: E0392, E0432.
[00:11:55] For more information about an error, try `rustc --explain E0392`.
[00:11:55] For more information about an error, try `rustc --explain E0392`.
[00:11:55] error: Could not compile `rustc_mir`.
[00:11:55] warning: build failed, waiting for other jobs to finish...
[00:14:40] error: build failed
[00:14:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:40] expected success, got: exit code: 101
[00:14:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:40] Build completed unsuccessfully in 0:11:23
[00:14:40] make: *** [all] Error 1
[00:14:40] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ee4fb3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 14:34:36 UTC 2019
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1b39cc28
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access '/home/travis/Librarers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1dd2e398
$ dmesg | grep -i kill
