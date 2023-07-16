plain
travis_time:end:00658f30:start=1540726071786761911,finish=1540726164927521186,duration=93140759275
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:47:19] .................................................................................................... 100/4972
[00:47:22] .................................................................................................... 200/4972
[00:47:25] .................................................................................................... 300/4972
[00:47:28] .................................................................................................... 400/4972
[00:47:31] .................................F.F................................................................ 500/4972
[00:47:39] .................................................................................................... 700/4972
[00:47:44] ...........................................................i...........i............................ 800/4972
[00:47:47] .............................................................................iiiii.................. 900/4972
[00:47:51] .................................................................................................... 1000/4972
---
[00:49:23] .................................................................................................... 4000/4972
[00:49:26] .................................................................................................... 4100/4972
[00:49:29] ......................................................................i............................. 4200/4972
[00:49:34] .................................................................................................... 4300/4972
[00:49:37] ........................................................F........................................... 4400/4972
[00:49:44] ..........................................i......................................................... 4600/4972
[00:49:48] .................................................................................................... 4700/4972
[00:49:50] .................................................................................................... 4800/4972
[00:49:53] .................................................................................................... 4900/4972
[00:49:53] .................................................................................................... 4900/4972
[00:49:55] ...........i............................................................
[00:49:55] failures:
[00:49:55] 
[00:49:55] ---- [ui] ui/borrowck/two-phase-nonrecv-autoref.rs#ast stdout ----
[00:49:55] 
[00:49:55] error in revision `ast`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:79: unexpected error: '79:11: 79:12: use of moved value: `f` [E0382]'
[00:49:55] 
[00:49:55] error in revision `ast`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:95: unexpected error: '95:11: 95:12: use of moved value: `f` [E0382]'
[00:49:55] 
[00:49:55] error in revision `ast`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:79: expected error not found: use of moved value: `*f`
[00:49:55] 
[00:49:55] error in revision `ast`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:95: expected error not found: use of moved value: `*f`
[00:49:55] 
[00:49:55] error in revision `ast`: 2 unexpected errors found, 2 expected errors not found
[00:49:55] status: exit code: 1
[00:49:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.ast/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.ast/auxiliary" "-A" "unused"
[00:49:55]     Error {
[00:49:55]         line_num: 79,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "79:11: 79:12: use of moved value: `f` [E0382]"
[00:49:55]     Error {
[00:49:55]         line_num: 95,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "95:11: 95:12: use of moved value: `f` [E0382]"
[00:49:55] ]
[00:49:55] 
[00:49:55] not found errors (from test file): [
[00:49:55]     Error {
[00:49:55]     Error {
[00:49:55]         line_num: 79,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "use of moved value: `*f`"
[00:49:55]     Error {
[00:49:55]         line_num: 95,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "use of moved value: `*f`"
[00:49:55] ]
[00:49:55] 
[00:49:55] thread '[ui] ui/borrowck/two-phase-nonrecv-autoref.rs#ast' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1358:13
[00:49:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:55] 
[00:49:55] ---- [ui] ui/borrowck/two-phase-nonrecv-autoref.rs#nll stdout ----
[00:49:55] 
[00:49:55] error in revision `nll`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:79: unexpected error: '79:11: 79:12: use of moved value: `f` [E0382]'
[00:49:55] 
[00:49:55] error in revision `nll`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:95: unexpected error: '95:11: 95:12: use of moved value: `f` [E0382]'
[00:49:55] 
[00:49:55] error in revision `nll`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:79: expected error not found: use of moved value: `*f`
[00:49:55] 
[00:49:55] error in revision `nll`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:95: expected error not found: cannot move a value of type
[00:49:55] 
[00:49:55] error in revision `nll`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:95: expected error not found: cannot move a value of type
[00:49:55] 
[00:49:55] error in revision `nll`: /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:95: expected error not found: use of moved value: `*f`
[00:49:55] 
[00:49:55] error in revision `nll`: 2 unexpected errors found, 4 expected errors not found
[00:49:55] status: exit code: 1
[00:49:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "two-phase-borrows" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/auxiliary" "-A" "unused"
[00:49:55]     Error {
[00:49:55]         line_num: 79,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "79:11: 79:12: use of moved value: `f` [E0382]"
[00:49:55]     Error {
[00:49:55]         line_num: 95,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "95:11: 95:12: use of moved value: `f` [E0382]"
[00:49:55] ]
[00:49:55] 
[00:49:55] not found errors (from test file): [
[00:49:55]     Error {
[00:49:55]     Error {
[00:49:55]         line_num: 79,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "use of moved value: `*f`"
[00:49:55]     Error {
[00:49:55]         line_num: 95,
[00:49:55]         kind: Some(
[00:49:55]             Error
---
[00:49:55]         line_num: 95,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "use of moved value: `*f`"
[00:49:55] ]
[00:49:55] 
[00:49:55] thread '[ui] ui/borrowck/two-phase-nonrecv-autoref.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1358:13
[00:49:55] 
[00:49:55] 
[00:49:55] ---- [ui] ui/span/borrowck-call-is-borrow-issue-12224.rs stdout ----
[00:49:55] 
[00:49:55] error: /checkout/src/test/ui/span/borrowck-call-is-borrow-issue-12224.rs:44: unexpected error: '44:5: 44:8: cannot borrow field `f.f` of immutable binding as mutable [E0596]'
[00:49:55] 
[00:49:55] error: /checkout/src/test/ui/span/borrowck-call-is-borrow-issue-12224.rs:44: expected error not found: cannot borrow `Box` content `*f.f` of immutable binding as mutable
[00:49:55] error: 1 unexpected errors found, 1 expected errors not found
[00:49:55] status: exit code: 1
[00:49:55] status: exit code: 1
[00:49:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/borrowck-call-is-borrow-issue-12224.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-call-is-borrow-issue-12224/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-call-is-borrow-issue-12224/auxiliary" "-A" "unused"
[00:49:55]     Error {
[00:49:55]         line_num: 44,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "44:5: 44:8: cannot borrow field `f.f` of immutable binding as mutable [E0596]"
[00:49:55] ]
[00:49:55] 
[00:49:55] not found errors (from test file): [
[00:49:55]     Error {
[00:49:55]     Error {
[00:49:55]         line_num: 44,
[00:49:55]         kind: Some(
[00:49:55]             Error
[00:49:55]         ),
[00:49:55]         msg: "cannot borrow `Box` content `*f.f` of immutable binding as mutable"
[00:49:55] ]
[00:49:55] 
[00:49:55] thread '[ui] ui/span/borrowck-call-is-borrow-issue-12224.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1358:13
[00:49:55] 
---
[00:49:55] 
[00:49:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:49:55] 
[00:49:55] 
[00:49:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnuout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06917076
travis_time:start:06917076
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0696c702
$ dmesg | grep -i kill
