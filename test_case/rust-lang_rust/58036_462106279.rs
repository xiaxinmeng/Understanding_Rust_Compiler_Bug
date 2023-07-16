plain
travis_time:end:1215dede:start=1549776753127074756,finish=1549776756301676887,duration=3174602131
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:18:37]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `DiagnosticId`
[00:18:37]   --> src/librustc_resolve/error_reporting.rs:64:20
[00:18:37]    |
[00:18:37] 64 |         let code = DiagnosticId::Error(code.into());
[00:18:37]    |                    ^^^^^^^^^^^^ use of undeclared type or module `DiagnosticId`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:18:37]   --> src/librustc_resolve/error_reporting.rs:74:17
[00:18:37]    |
[00:18:37]    |
[00:18:37] 74 |                 Applicability::MaybeIncorrect,
[00:18:37]    |                 ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `DiagnosticId`
[00:18:37]   --> src/librustc_resolve/error_reporting.rs:81:22
[00:18:37]    |
[00:18:37]    |
[00:18:37] 81 |             err.code(DiagnosticId::Error("E0411".into()));
[00:18:37]    |                      ^^^^^^^^^^^^ use of undeclared type or module `DiagnosticId`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `DiagnosticId`
[00:18:37]   --> src/librustc_resolve/error_reporting.rs:90:22
[00:18:37]    |
[00:18:37]    |
[00:18:37] 90 |             err.code(DiagnosticId::Error("E0424".into()));
[00:18:37]    |                      ^^^^^^^^^^^^ use of undeclared type or module `DiagnosticId`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:18:37]    --> src/librustc_resolve/error_reporting.rs:149:21
[00:18:37]     |
[00:18:37] 149 |                     Applicability::MachineApplicable,
---
[00:18:37] 
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:18:37]    --> src/librustc_resolve/error_reporting.rs:204:17
[00:18:37]     |
[00:18:37] 204 |                 Applicability::MaybeIncorrect,
[00:18:37]     |                 ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:18:37]    --> src/librustc_resolve/error_reporting.rs:251:21
[00:18:37]     |
[00:18:37]     |
[00:18:37] 251 |                     Applicability::MaybeIncorrect,
[00:18:37]     |                     ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:18:37]    --> src/librustc_resolve/error_reporting.rs:266:25
[00:18:37]     |
[00:18:37]     |
[00:18:37] 266 |                         Applicability::MaybeIncorrect,
[00:18:37]     |                         ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:18:37]    --> src/librustc_resolve/error_reporting.rs:275:25
[00:18:37]     |
[00:18:37]     |
[00:18:37] 275 |                         Applicability::MaybeIncorrect,
[00:18:37]     |                         ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:18:37]    --> src/librustc_resolve/error_reporting.rs:360:41
[00:18:37]     |
[00:18:37]     |
[00:18:37] 360 |                                         Applicability::MaybeIncorrect
[00:18:37]     |                                         ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:18:37] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:18:37]    --> src/librustc_resolve/error_reporting.rs:378:37
[00:18:37]     |
[00:18:37]     |
[00:18:37] 378 |                                     Applicability::MaybeIncorrect,
[00:18:37]     |                                     ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:18:37] 
[00:18:38] error[E0412]: cannot find type `DiagnosticBuilder` in this scope
[00:18:38]    |
[00:18:38]    |
[00:18:38] 27 |     ) -> (DiagnosticBuilder<'a>, Vec<ImportSuggestion>) {
[00:18:38] help: possible candidates are found in other modules, you can import them into scope
[00:18:38]    |
[00:18:38] 1  | use rustc_errors::DiagnosticBuilder;
[00:18:38]    |
[00:18:38]    |
[00:18:38] 1  | use syntax::diagnostics::plugin::DiagnosticBuilder;
[00:18:38]    |
[00:18:38] 
[00:18:38] error[E0412]: cannot find type `DiagnosticBuilder` in this scope
[00:18:38]     |
[00:18:38] 235 |         err: &mut DiagnosticBuilder<'a>,
[00:18:38]     |                   ^^^^^^^^^^^^^^^^^ not found in this scope
[00:18:38] help: possible candidates are found in other modules, you can import them into scope
---
[00:19:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:18] expected success, got: exit code: 101
[00:19:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:18] Build completed unsuccessfully in 0:15:06
[00:19:18] make: *** [all] Error 1
[00:19:18] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:034e2d38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 05:52:04 UTC 2019
---
travis_time:end:087365c1:start=1549777925345554791,finish=1549777925351338930,duration=5784139
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ce5e5bb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3cdd533c
travis_time:start:3cdd533c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:033d6bc0
$ dmesg | grep -i kill
