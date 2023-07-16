plain
travis_time:end:0df20bac:start=1543253060223557388,finish=1543253061334229520,duration=1110672132
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:37] 
[00:47:37] running 5067 tests
[00:47:40] ..............................................................................................FF.... 100/5067
[00:47:43] .............................................FF.F................................................... 200/5067
[00:47:49] .................................................................................................... 400/5067
[00:47:49] .................................................................................................... 400/5067
[00:47:52] ..........FF....FFFF................................................................................ 500/5067
[00:47:59] .................................................................................................... 700/5067
[00:47:59] .................................................................................................... 700/5067
[00:48:05] ...F..................................F.........................................................i... 800/5067
[00:48:09] ........iF.......................................................................................... 900/5067
[00:48:12] ...............iiiii................................................................................ 1000/5067
[00:48:15] .............................................................................................FFFFFFF 1100/5067
[00:48:17] F................................................................................................... 1200/5067
[00:48:19] ..................................................................F.F............................... 1300/5067
[00:48:22] ..........................................F......................................................... 1400/5067
[00:48:24] .........................F.........F.............................................F.................. 1500/5067
[00:48:27] ..............i............FF......................................................i................ 1600/5067
[00:48:31] ..............................................FF.................................................... 1700/5067
[00:48:37] .................................................................................................... 1900/5067
[00:48:37] .................................................................................................... 1900/5067
[00:48:40] ......................i.............................................F............................... 2000/5067
[00:48:44] .................................................................................................... 2100/5067
[00:48:49] ....................................F............................................................... 2200/5067
[00:48:56] .................................................................................................... 2400/5067
[00:48:56] .................................................................................................... 2400/5067
[00:49:00] ......F.....................................................................................F....... 2500/5067
[00:49:04] .................................................................................................... 2600/5067
[00:49:08] ..................F.FF.FFFFFFF...................................................................... 2700/5067
[00:49:11] ...............................F...................F.....F....FF...F........F....................... 2800/5067
[00:49:14] ......................................FFF..............................F............................ 2900/5067
[00:49:18] .................................................................................................... 3000/5067
[00:49:21] ..................................................................i.......F......................... 3100/5067
[00:49:24] .................................................................................................... 3200/5067
[00:49:28] .........F...................ii..i..ii.............................................................. 3300/5067
[00:49:32] .............................................................F..FFF...FFFF..FFF.FF.................. 3400/5067
[00:49:35] F...........F......................................F........F....................................... 3500/5067
[00:49:38] ..........ii........................................................................F............... 3600/5067
[00:49:40] ............................i...........................F........................................... 3700/5067
[00:49:42] .................................................................................................... 3900/5067
[00:49:46] .................................................................................................... 4000/5067
[00:49:49] .................................................................................................... 4100/5067
[00:49:49] .................................................................................................... 4100/5067
[00:49:52] ................................F..................................F...........F.................... 4200/5067
[00:49:55] .............................................i...........................F...............FFF........ 4300/5067
[00:50:01] .F...........................................F...F.................................................. 4400/5067
[00:50:07] .................................................................................................... 4600/5067
[00:50:07] .................................................................................................... 4600/5067
[00:50:11] ...........F..F................i....................FFF............................................. 4700/5067
[00:50:15] .................................................................................................... 4800/5067
[00:50:17] ................................................F................................................... 4900/5067
[00:50:22] ......i........................F...................................
[00:50:22] 
[00:50:22] ---- [ui] ui/associated-types/associated-types-overridden-binding.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs:14: unexpected error: '14:1: 14:30: type annotations required: cannot resolve `<Self as std::iter::Iterator>::Item == i32` [E0284]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associateunknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 16,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "16:39: 16:60: type mismatch resolving `<std::vec::IntoIter<u32> as std::iter::Iterator>::Item == i32` [E0271]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/associated-types/associated-types-overridden-binding-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/await-keyword/2015-edition-warning.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/await-keyword/2015-edition-warning.rs:7: unexpected error: '7:13: 7:18: `await` is a keyword in the 2018 edition [keyword_idents]'
[00:50:22] error: /checkout/src/test/ui/await-keyword/2015-edition-warning.rs:7: unexpected warning: '7:13: 7:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!'
[00:50:22] 
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/await-keyword/2015-edition-warning.rs:8: unexpected error: '8:20: 8:25: `await` is a keyword in the 2018 edition [keyword_idents]'
[00:50:22] error: /checkout/src/test/ui/await-keyword/   line_num: 8,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         ),
[00:50:22]         msg: "8:20: 8:25: `await` is a keyword in the 2018 edition [keyword_idents]"
[00:50:22]     Error {
[00:50:22]         line_num: 8,
[00:50:22]         kind: Some(
[00:50:22]             Warning
---
[00:50:22]         line_num: 11,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "11:16: 11:21: `await` is a keyword in the 2018 edition [keyword_idents]"
[00:50:22]     Error {
[00:50:22]         line_num: 11,
[00:50:22]         kind: Some(
[00:50:22]             Warning
---
[00:50:22]         line_num: 11,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "11:23: 11:28: `await` is a keyword in the 2018 edition [keyword_idents]"
[00:50:22]     Error {
[00:50:22]         line_num: 11,
[00:50:22]         kind: Some(
[00:50:22]             Warning
[00:50:22]             Warning
[00:50:22]         ),
[00:50:22]         msg: "11:23: 11:28: this was previously accepted by the compiler but is being phased out; it will becomerror {
[00:50:22]         line_num: 5,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "5:13: 5:18: `await` is a keyword in the 2018 edition [E0721]"
[00:50:22]     Error {
[00:50:22]         line_num: 6,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "6:20: 6:25: `await` is a keyword in the 2018 edition [E0721]"
[00:50:22]     Error {
[00:50:22]         line_num: 9,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "9:22: 9:27: `await` is a keyword in the 2018 edition [E0721]"
[00:50:22]     Error {
[00:50:22]         line_num: 9,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "9:29: 9:34: `await` is a keyword in the 2018 edition [E0721]"
[00:50:22]     Error {
[00:50:22]         line_num: 12,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "12:11: 12:16: `await` is a keyword in the 2018 edition [E0721]"
[00:50:22]     Error {
[00:50:22]         line_num: 12,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "12:19: 12:24: `await` is a keyword in the 2018 edition [E0721]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/await-keyword/2018-edition-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] thread '[ui] ui/await-keyword/2018-edition-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00d-mutation-of-moved-out-with-mut.rs:25: unexpected error: '25:31: 25:34: use of moved value: `t.0` [E0382]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs:25: unexpected error: '25:36: 25:39: use of moved value: `t.1` [E0382]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs:33: unexpected error: '33:31: 33:34: use of moved value: `u.0` [E0382]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs:33: unexpected error: '33:36: 33:39: use of moved value: `u.1` [E0382]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs:41: unexpected error: '41:31: 41:34: use of moved value: `v.x` [E0382]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs:41: unexpected error: '41:36: 41:39: use of moved value: `v.y` [E0382]'
[00:50:22] 
[00:50:22] error in revision `ast`: 6 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54499-field-mutation-of-mov    Error
[00:50:22]         ),
[00:50:22]         msg: "41:36: 41:39: use of moved value: `v.y` [E0382]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs#nll stdout ----
[00:50:22] 
[00:50:22] error in revision `nll`: /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs:23: unexpected error: '23:9: 23:19: assign to part of moved value: `t` [E0382]'
[00:50:22] 
[00:50:22] error in revision `nll`: /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs:31: unexpected error: '31:9: 31:19: assign to part of moved value: `u` [E0382]'
[00:50:22] 
[00:50:22] error in revision `nll`: /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs:39: unexpected error: '39:9: 39:19: assign to part of moved value: `v` [E0382]'
[00:50:22] 
[00:50:22] error in revision `nll`: 3 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=migrate" "-Z" "two-phase-borrows" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.nll/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 23,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "23:9: 23:19: assign to part of moved value: `t` [E0382]"
[00:50:22]     Error {
[00:50:22]         line_num: 31,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "31:9: 31:19: assign to part of moved value: `u` [E0382]"
[00:50:22]     Error {
[00:50:22]         line_num: 39,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "39:9: 39:19: assign to part of moved value: `v` [E0382]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/borrowck/issue-54499-field-mutation-of-moved-out-with-mut.rs#nll' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat.rs:16: unexpected error: '16:13: 16:19: cannot move out of borrowed content [E0507]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 16,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "16:13: 16:19: cannot move out of borrowed content [E0507]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs#ast stdout ----
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:21: unexpected error: '21:22: 21:45: closure cannot assign to immutable argument `x` [E0595]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:29: unexpected error: '29:22: 29:45: closure cannot assign to immutable argument `x` [E0595]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:40: unexpected error: '40:9: 40:11: closure cannot assign to immutable argument `x` [E0595]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:43: unexpected error: '43:9: 43:11: closure cannot assign to immutable argument `x` [E0595]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:46: unexpected error: '46:9: 46:11: closure cannot assign to immutable argument `x` [E0595]'
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:49: unexpected error: '49:9: 49:11: closure cannot assign to immutable argument `x` [E0595]'
[00:50:22] 
[00:50:22] error in revision `ast`: 6 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.ast/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=ast" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/tesnt `x` [E0595]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs#migrate stdout ----
[00:50:22] 
[00:50:22] error in revision `migrate`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:21: unexpected error: '21:46: 21:51: cannot assign to `x`, as it is not declared as mutable [E0594]'
[00:50:22] 
[00:50:22] error in revision `migrate`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:30: unexpected error: '30:50: 30:55: cannot assign to `x`, as it is not declared as mutable [E0594]'
[00:50:22] 
[00:50:22] error in revision `migrate`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:40: unexpected error: '40:14: 40:22: cannot assign to `x`, as it is not declared as mutable [E0594]'
[00:50:22] 
[00:50:22] error in revision `migrate`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:43: unexpected error: '43:14: 43:21: cannot assign to `x.0`, as `x` is not declared as mutable [E0594]'
[00:50:22] 
[00:50:22] error in revision `migrate`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:46: unexpected error: '46:14: 46:20: cannot borrow `x` as mutable, as it is not declared as mutable [E0596]'
[00:50:22] 
[00:50:22] error in revision `migrate`: /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:49: unexpected     kind: Some(
[00:50:22]         ),
[00:50:22]         ),
[00:50:22]         msg: "49:14: 49:22: cannot borrow `x.0` as mutable, as `x` is not declared as mutable [E0596]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs#nll' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs:31: unexpected error: '31:27: 31:38: borrowed value does not live long enough [E0597]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 31,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]        s value will cause an error [const_err]"
[00:50:22]     Error {
[00:50:22]         line_num: 17,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "17:1: 17:63: any use of this value will cause an error [const_err]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/consts/const-int-unchecked.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/custom_test_frameworks/mismatch.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/custom_test_frameworks/mismatch.rs:19: unexpected error: '19:1: 19:18: the trait bound `test::TestDescAndFn: example_runner::Testable` is not satisfied [E0277]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/mismatch.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 19,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:   }
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/error-codes/E0161.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] ---- [ui] ui/error-codes/E0161.rs#astul stdout ----
[00:50:22] 
[00:50:22] error in revision `astul`: /checkout/src/test/ui/error-codes/E0161.rs:32: unexpected error: '32:5: 32:11: cannot move a value of type [i32]: the size of [i32] cannot be statically determined [E0161]'
[00:50:22] 
[00:50:22] error in revision `astul`: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0161.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "astul" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0161.astul/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0161.astul/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 32,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "32:5: 32:11: cannot move a value of type [i32]: the size of [i32] cannot be statically determined [E0161]"
[00:50:22] ]
[00:50:22] 
[00:50:22] 
[00:50:22] thread '[ui] ui/error-codes/E0161.rs#astul' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:e of type [i32]: the size of [i32] cannot be statically determined [E0161]'
[00:50:22] 
[00:50:22] error in revision `nll`: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0161.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0161.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0161.nll/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 32,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "32:9: 32:11: cannot move a value of type [i32]: the size of [i32] cannot be statically determined [E0161]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/error-codes/E0161.rs#nll' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/error-codes/E0161.rs#nllul stdout ----
[00:50:22] 
[00:50:22] error in revision `nllul`: /checkout/src/test/ui/error-codes/E0161.rs:32: unexpected error: '32:5: 32:11: cannot move a value of type [i32]: the size of [i32] cannot be statically determined [E0161]'
[00:50:22] 
[00:50:22] error in revision `nllul`: 1 unexpected errors found, 0 expectobj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0718/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 14,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "14:1: 14:16: `arc` language item must be applied to a struct [E0718]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/error-codes/E0718.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/error-codes/E0719.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/error-codes/E0719.rs:11: unexpected error: '11:33: 11:43: the value of the associated type `Item` (from the trait `std::iter::Iterator`) is already specified [E0719]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/error-codes/E0719.rs:15: unexpected error: '15:38: 15:49: the value of the associated type `Item` (from the trait `std::iter::Iterator`) is already specified [E0719]'
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0719.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gn/test/ui/feature-gate-underscore_const_names/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-underscore_const_names/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 17,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "17:1: 22:3: naming constants with `_` is unstable (see issue #54912) [E0658]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/feature-gate-underscore_const_names.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/feature-gates/feature-gate-impl_trait_in_bindings.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/feature-gates/feature-gate-impl_trait_in_bindings.rs:16: unexpected error: '16:15: 16:19: expected expression, found keyword `impl`'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/feature-gates/feature-gate-impl_trait_in_bindings.rs:11: unexpected error: '11:12: 11:21: `impl Trait` not allowed outside of function and inherent method return types [E0562]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/feature-gates/feature-gate-impl_trait_in_bindings.rs:13: unexpected error: '13:13: 13:22: `impl Trait` not allowed outside of function and inherent method return types [E0562]'
[00:50:22] error: 3 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:2untest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/feature-gates/feature-gate-linker-flavor.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/feature-gates/feature-gate-linker-flavor.rs:16: unexpected error: '16:1: 16:8: attribute must be applied to a `static` variable'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-linker-flavor.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-linker-flavor/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-linker-flavor/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 16,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "16:1: 16:8: attribute must be applied to a `static` variable"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/feature-gates/feature-gate-linker-flavor.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/feature-gates/feature-gate-trait-alias.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/feature-gates/feature-gate-trait-alias.rs:11: unexpected error: '11:1: 11:21: trait aliases are experimental (see issue #41517) [E0658]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-trait-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trait-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trait-alias/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 11,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "11:1: 11:21: trait aliases are experimental (see issue #41517) [E0658]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/feature-gates/feature-gate-trait-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/generator/generator-region-requirements.rs#ast stdout ----
[00:50:22] 
[00:50:22] error in revision `ast`: /checkout/src/test/ui/generator/generator-region-requirements.rs:15: unexpected error: '15:51: 15:52: explicit lifetime required in the type of `x` [E0621]'
[00:50:22] 
[00:50:22] error in revision `ast`: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/generator-region-requirements.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.ast/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.ast/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 15,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "15:51: 15:52: explicit lifetime required in the type of `x` [E0621]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/generator/generator-region-requirements.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/generator/generator-region-requirements.rs#nll stdout ----
[00:50:22] 
[00:50:22] error in revision `nll`: /checkout/src/test/ui/generator/generator-region-requirements.rs:15: unexpected error: '15:51: 15:52: explicit lifetime required in the type of `x` [E0621]'
[00:50:22] 
[00:50:22] error in revision `nll`: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/generator-region-requirements.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.nll/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 15,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "15:51: 15:52: explicit lifetime required in the type of `x` [E0621]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/generator/generator-region-requirements.rs#nll' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/impl-trait/bindings.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/impl-trait/bindings.rs:14: unexpected error: '14:29: 14:30: can't capture dynamic environment in a fn item [E0434]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/impl-trait/bindings.rs:19: unexpected error: '19:33: 19:34: can't capture dynamic environment in a fn item [E0434]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/impl-trait/bindings.rs:25: unexpected error: '25:33: 25:34: can't capture dynamic environment in a :50:22]     },
[00:50:22]         line_num: 31,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         ),
[00:50:22]         msg: "31:33: 31:34: can\'t capture dynamic environment in a fn item [E0434]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/impl-trait/bindings.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/impl-trait/bindings-opaque.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/impl-trait/bindings-opaque.rs:20: unexpected error: '20:17: 20:27: no method named `count_ones` found for type `impl std::marker::Copy` in the current scope [E0599]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/impl-trait/bindings-opaque.rs:21: unexpected error: '21:17: 21:27: no method named `count_ones` found for type `impl std::marker::Copy` in the current scope [E0599]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/impl-trait/bindings-opaque.rs:22: unexpected error: '22:17: 22:27: no method named `count_ones` found for type `impl std::marker::Copy` in the current scope [E0599]'
[00:50:22] error: 3 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bindings-opaque.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings-opaque/a" "-Crpath" "-O" "-Zunstable-options" "-Lnanknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39175.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 24,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "24:39: 24:43: no method named `exec` found for type `&mut std::process::Command` in the current scope [E0599]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/issues/issue-39175.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/issues/issue-45829/import-self.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-45829/import-self.rs:18: unexpected error: '18:12: 18:16: expected identifier, found keyword `self`'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-45829/import-self.rs:20: unexpected error: '20:5: 20:14: `self` imports are only allowed within a { } list [E0429]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-45829/import-self.rs:16: unexpected error: '16:11: 16:15: the name `foo` is defined multiple times [E0255]'
[00:50:22] error: /checkout/src/test/ui/issues/e_num: 23,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         ),
[00:50:22]         msg: "23:11: 23:20: the name `A` is defined multiple times [E0252]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/issues/issue-45829/import-self.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/issues/issue-45829/issue-45829.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-45829/issue-45829.rs:16: unexpected error: '16:14: 16:20: the name `A` is defined multiple times [E0252]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45829/issue-45829.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45829/issue-45829/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45829/issue-45829/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 16,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "16:14: 16:20: the name `A` is defined multiple times [E0252]"
[00:50:22] ]
[00:50:22] 
[00:50:22] 
[00:50:22] thre error: /checkout/src/test/ui/issues/issue-45829/rename-extern-vs-use.rs:18: unexpected error: '18:1: 18:35: the name `bar` is defined multiple times [E0254]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45829/rename-extern-vs-use.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45829/rename-extern-vs-use/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45829/rename-extern-vs-use/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 18,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "18:1: 18:35: the name `bar` is defined multiple times [E0254]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/issues/issue-45829/rename-extern-vs-use.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/issues/issue-45829/rename-extern-with-tab.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-45829/rename-extern-with-tab.rs:15: unexpected error: '15:1: 15:51: the name `issue_45829_a` is defined multiple times [E0259]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45829/rename-extern-with-tab.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45829/rename-extern-with-tab/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45829/rename-extern-with-tab/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 15,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "15:1: 15:51: the name `issue_45829_a` is defined multiple times [E0259]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/issues/issue-45829/rename-extern-with-tab.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/issues/issue-45829/rename-extern.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-45829/rename-extern.rs:15: unexpected error: '15:1: 15:45: the name `issue_45829_a` is defined multiple times [E0259]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45829/rename-extern.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45829/rename-extern/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45829/rename-extern/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 15,
---
[00:50:22] ---- [ui] ui/issues/issue-54348.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-54348.rs:3: unexpected error: 0:22] ---- [ui] ui/issues/issue-55796.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-55796.rs:16: unexpected error: '16:9: 16:56: cannot infer an appropriate lifetime due to conflicting requirements [E0495]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/issues/issue-55796.rs:20: unexpected error: '20:9: 20:55: cannot infer an appropriate lifetime due to conflicting requirements [E0495]'
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55796.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55796/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55796/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 16,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "16:9: 16:56: cannot infer an appropriate lifetime due to conflicting requirements [E0495]"
[00:50:22]     Error {
[00:50:22]         line_num: 20,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "20:9: 20:55: cannot infer an appropriate lifetime due to conflicting renteger}; 2]` is not an iterator [E0277]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/iterators/array.rs:4: unexpected error: '4:14: 4:15: `[{integer}; 2]` is not an iterator [E0277]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/iterators/array.rs:5: unexpected error: '5:14: 5:24: `[{float}; 2]` is not an iterator [E0277]'
[00:50:22] error: 3 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/array.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 2,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "2:14: 2:20: `[{integer}; 2]` is not an iterator [E0277]"
[00:50:22]     Error {
[00:50:22]         line_num: 4,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "4:14: 4:15: `[{integer}; 2]` is not an iterator [E0277]"
[00:50:22]     Error {
[00:50:22]         line_num: 5,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]::RangeInclusive<{integer}>; 1]` is not an iterator [E0277]"
[00:50:22]     Error {
[00:50:22]         line_num: 4,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "4:14: 4:19: `[std::ops::RangeFrom<{integer}>; 1]` is not an iterator [E0277]"
[00:50:22]     Error {
[00:50:22]         line_num: 5,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "5:14: 5:19: `[std::ops::RangeTo<{integer}>; 1]` is not an iterator [E0277]"
[00:50:22]     Error {
[00:50:22]         line_num: 6,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "6:14: 6:20: `[std::ops::RangeToInclusive<{integer}>; 1]` is not an iterator [E0277]"
[00:50:22]     Error {
[00:50:22]         line_num: 9,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "9:14: 9:26: `[std::ops::Range<{integer}>; 1]` is not an iterator [E0277]"
[00:50:22]     Error {
[00:50:22]         line_num: 11,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "11:14: 11:28: `[std::ops::Range<{integer}>; 1]` is not an iterator [E0277]"
[00:50:22]     Error {
[00:50:22]         line_num: 12,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "12:14: 12:26: `[std::ops::Range<{integer}>; 2]` is not an iterator [E0277]"
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "17:10: 17:14: mismatched types [E0308]"
[00:50:22]     },
---
[00:50:22] thread '[ui] ui/mismatched_types/numeric-literal-cast.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] ---- [ui] ui/nll/issue-52086.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-52086.rs:20: unexpected error: '20:10: 20:17: cannot move out of an `Rc` [E0507]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-52086.rs:23: unexpected error: '23:10: 23:17: cannot move out of an `Arc` [E0507]'
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52086.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52086/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52086/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 20,
[00:50:22]         line_num: 20,
22] error: /checkout/src/test/ui/nll/issue-52534-1.rs:45: unexpected error: '45:5: 45:7: cannot return reference to local variable `x` [E0515]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-52534-1.rs:50: unexpected error: '50:5: 50:7: cannot return reference to local variable `x` [E0515]'
[00:50:22] error: 8 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52534-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534-1/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 19,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "19:9: 19:11: cannot return reference to local variable `x` [E0515]"
[00:50:22]     Error {
[00:50:22]         line_num: 25,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "25:5: 25:7: cannot return reference to local variable `x` [E0515]"
[00:50:22]     Error {
[00:50:22]         line_num: 30,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]    heckout/src/test/ui/nll/issue-52534-2.rs:19: unexpected error: '19:9: 19:15: `x` does not live long enough [E0597]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52534-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534-2/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 19,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "19:9: 19:15: `x` does not live long enough [E0597]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/nll/issue-52534-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/nll/issue-52534.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-52534.rs:22: unexpected error: '22:14: 22:15: `x` does not live long enough [E0597]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-52534.rs:27: unexpected error: '27:26: 27:27: `y` does not live long enough [E0597]'
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52534.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 22,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "22:14: 22:15: `x` does not live long enough [E0597]"
[00:50:22]     Error {
[00:50:22]         line_num: 27,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "27:26: 27:27: `y` does not live long enough [E0597]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/nll/issue-52534.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/nll/issue-52669.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-52669.rs:25: unexpected error: '25:5: 25:8: borrow of moved value: `a.b` [E0382]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52669.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52669/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52669/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 25,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "25:5: 25:8: borrow of moved value: `a.b` [E0382]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/nll/issue-52669.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/nll/issue-53040.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-53040.rs:15: unexpected error: '15:8: 15:14: captured variable cannot escape `FnMut` closure body'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-53040.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53040/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53040/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 15,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "15:8: 15:14: captured variable cannot escape `FnMut` closure body"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/nll/issue-53040.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/nll/issue-53807.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-53807.rs:14: unexpected error: '14:30: 14:35: use of partially moved value: `maybe` [E0382]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-53807.rs:14: unexpected error: '14:21: 14:26: use of moved value: `(maybe as std::prelude::v1::Some).0` [E0382]'
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-53807.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53807/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53807/auxiliary" "-A" "unused"
sed"
[00:50:22] unexpected errors (from JSON output): [
[00:50:22]     Error {
[00:50:22]         line_num: 7,
[00:50:22]         line_num: 7,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "7:30: 7:37: `_thing1` does not live long enough [E0597]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/nll/issue-54382-use-span-of-tail-of-block.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/nll/issue-54556-niconii.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-54556-niconii.rs:22: unexpected error: '22:20: 22:27: `counter` does not live long enough [E0597]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-54556-niconii.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-54556-niconii/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-54556-niconii/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 22,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "22:20: 22:27: `counter` does not live long enough [E0:22]     },
[00:50:22]         line_num: 20,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         ),
[00:50:22]         msg: "20:56: 20:59: `_t1` does not live long enough [E0597]"
[00:50:22]     Error {
[00:50:22]         line_num: 24,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "24:56: 24:59: `_t1` does not live long enough [E0597]"
[00:50:22]     Error {
[00:50:22]         line_num: 30,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "30:56: 30:59: `_t1` does not live long enough [E0597]"
[00:50:22]     Error {
[00:50:22]         line_num: 32,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "32:56: 32:59: `_t1` does not live long enough [E0597]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/nll/issue-54556-used-vs-unused-tails.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/nll/issue-55394.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/issue-55394.rs:21: unexpected error: '21:9: 21:20: unsatisfied lifetime constraints'
[00:50:22] 
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55394.rs" "--target=x86_64-unknown-linux-gnu" "-nll/move-subpaths-moves-root/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 16,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "16:10: 16:11: use of moved value: `x` [E0382]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/nll/move-subpaths-moves-root.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/nll/relate_tys/universe-violation.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/nll/relate_tys/universe-violation.rs:15: unexpected error: '15:31: 15:32: higher-ranked subtype error'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/relate_tys/universe-violation.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/universe-violation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zno-leak-check" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/universe-violation/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 15,
[00:50:22]         kind: Some(
[00:50:22]             Error
---
[00:50:22] error: /checkout/src/test/ui/nll/user-annotations/issue-54124.rs:4: unexpected error: '4:22: 4:23: unsatisfied lifetime constraints'
[00:50:22] 
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/issue-54124.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-54124/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-54124/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 4,
[00:50:22]         kind: Some(
[00:50:22]             Error
---
[00:50:22]         line_num: 4,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "4:22: 4:23: unsatisfied lifetime constraints"
[00:50:22] error: /checkout/src/test/ui/regions/regions-struct-not-wf.rs:31: unexpected error: '31:5: 31:29: the parameter type `T` may not live long enough [E0309]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/regions/regions-struct-not-wf.rs:35: unexpected error: '35:5: 35:26: in type `&'a &'b T`, reference has a longer lifetime than the data it references [E0491]'
[00:50:22] error: 3 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-struct-not-wf.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-struct-not-wf/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-struct-not-wf/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 23,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "23:5: 23:22: the parameter type `T` may not live long enough [E0309]"
[00:50:22]     Error {
[00:50:22]         line_num: 31,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "31:5: 31:29: the parameter type `T` may not live long enough [E0309]"
[00:50:22]     Error {
[00:50:22]     Error {
u/test/ui/rfc-2093-infer-outlives/regions-struct-not-wf/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/regions-struct-not-wf/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 23,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "23:5: 23:22: the parameter type `T` may not live long enough [E0309]"
[00:50:22]     Error {
[00:50:22]         line_num: 31,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "31:5: 31:29: the parameter type `T` may not live long enough [E0309]"
[00:50:22]     Error {
[00:50:22]         line_num: 35,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "35:5: 35:26: in type `&\'a &\'b T`, reference has a longer lifetime than the data it references [E0491]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/rfc-2093-infer-outlives/regions-struct-not-wf.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-feature-gate.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-feature-gate.rs:4: unexpected error: '4:5: 4:13: macro dbg! is unstable (see issue #54306) [E0658]'
[00:50:22] 
[00:50:22] errostc" "/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 8,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "8:23: 8:37: `NotDebug` doesn\'t implement `std::fmt::Debug` [E0277]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs:11: unexpected error: '11:18: 11:19: use of moved value: `a` [E0382]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs:11: unexpected error: '11:13: 11:20: use of moved value: `a` [E0382]'
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" d errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/generic-associated-types-where/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/generic-associated-types-where/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 11,
[00:50:22]         kind: Some(
[00:50:22]             Warning
[00:50:22]             Warning
[00:50:22]         ),
[00:50:22]         msg: "11:12: 11:36: the feature `generic_associated_types` is incomplete and may cause the compiler to crash"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/rfc1598-generic-associated-types/generic-associated-types-where.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/rust-2018/local-path-suggestions-2015.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/rust-2018/local-path-suggestions-2015.rs:34: unexpected error: '34:5: 34:11: unresolved import `foobar` [E0432]'
[00:50:22] error: 1 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/local-path-suggestions-2015.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/local-path-suggestions-2015/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "baz" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/local-path-suggestions-2015/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 34,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "34:5: 34:11: unresolved import `foobar` [E0432]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/rust-2018/local-path-suggestions-2015.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/rust-2018/trait-import-suggestions.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:32: unexpected error: '32:11: 32:17: no method named `foobar` found for type `u32` in the current scope [E0599]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:38: unexpected error: '38:7: 38:10: no method named `bar` found for type `u32` in the current scope [E0599]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:39: unexpected error: '39:7: 39:10: no method named `baz` found for type `u32` in the current scope [E0599]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:40: unexpected error: '40:13: 40:26: no function or associated item named `from_str` found for type `u32` in the current scope [E0599]'
[00:50:22] error: 4 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/trait-import-suggestions.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "trait-import-suggestions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 32,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "32:11: 32:17: no method named `foobar` found for type `u32` in the current scope [E0599]"
[00:50:22]     Error {
[00:50:22]         line_num: 38,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "38:7: 38:10: no method named `bar` found for type `u32` in the current scope [E0599]"
[00:50:22]     Error {
[00:50:22]         line_num: 39,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "39:7: 39:10: no method named `baz` found for type `u32` in the current scope [E0599]"
[00:50:22]     Error {
[00:50:22]         line_num: 40,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "40:13: 40:26: no function or associated item named `from_str` found for type `u32` in the current scope [E0599]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/rust-2018/trait-import-suggestions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
[00:50:22] 
[00:50:22] ---- [ui] ui/suggestions/use-type-argument-instead-of-assoc-type.rs stdout ----
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/suggestions/use-type-argument-instead-of-assoc-type.rs:6: unexpected error: '6:42: 6:47: wrong number of type arguments: expected 2, found 4 [E0107]'
[00:50:22] 
[00:50:22] error: /checkout/src/test/ui/suggestions/use-type-argument-instead-of-assoc-type.rs:6: unexpected error: '6:26: 6:64: the value of the associated types `A` (from the trait `T`), `C` (from the trait `T`) must be specified [E0191]'
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/use-type-argument-instead-of-assoc-type.rs" "--target=x86_64-unknown-0:50:22] 
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/use-type-argument-instead-of-assoc-type.rs" "--target=x86_64-unknown-0:50:22] 
[00:50:22] error: /checkout/src/test/ui/suggestions/suggest-variants.rs:14: unexpected error: '14:34: 14:47: no variant `Rombus` on enum `Shape`'
[00:50:22] error: 3 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-variants.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-variants/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-variants/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 12,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "12:34: 12:49: no variant `Squareee` on enum `Shape`"
[00:50:22]     Error {
[00:50:22]         line_num: 13,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "13:34: 13:46: no variant `Circl` on enum `Shape`"
[00:50:22]     Error {
[00:50:22]         line_num: 14,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "14:34: 14:47: no variant `Rombus` on enum `Shape`"
[00:50:22] ]
[00:50:22] 
[00:50:22] 
[00:50:22] thread '[trait.rs:29: unexpected error: '29:5: 29:27: the parameter type `T` may not live long enough [E0309]'
[00:50:22] error: 2 unexpected errors found, 0 expected errors not found
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-outlives-ty-in-fn-or-trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-outlives-ty-in-fn-or-trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-outlives-ty-in-fn-or-trait/auxiliary" "-A" "unused"
[00:50:22]     Error {
[00:50:22]         line_num: 19,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "19:5: 19:26: the parameter type `T` may not live long enough [E0309]"
[00:50:22]     Error {
[00:50:22]         line_num: 29,
[00:50:22]         kind: Some(
[00:50:22]             Error
[00:50:22]             Error
[00:50:22]         ),
[00:50:22]         msg: "29:5: 29:27: the parameter type `T` may not live long enough [E0309]"
[00:50:22] ]
[00:50:22] 
[00:50:22] thread '[ui] ui/wf/wf-outlives-ty-in-fn-or-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:50:22] 
---
[00:50:22] test result: FAILED. 4948 passed; 95 failed; 24 ignored; 0 measured; 0 filtered out
[00:50:22] 
[00:50:22] 
[00:50:22] 
[00:50:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/3664536 .
2359192 ./obj/build
1712852 ./obj/build/x86_64-unknown-linux-gnu
1161940 ./src
503828 ./obj/build/x86_64-unknown-linux-gnu/stage0
---
141388 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
141384 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
138440 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
134660 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134656 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f71cpnyfne-68yaly-18zp7vy2v2s65
130768 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130764 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-ores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0eac1bec
travis_time:start:0eac1bec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16880ffc
$ dmesg | grep -i kill
