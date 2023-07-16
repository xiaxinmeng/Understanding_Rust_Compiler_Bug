plain
travis_time:start:test_compile-fail
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:01] 
[00:54:01] running 2311 tests
[00:54:06] ..............F..F..................................................................................
[00:54:19] ....................................................................................................
[00:54:19] ....................................................................................................
[00:54:26] ...........................................................................................FF......F
[00:54:32] F.F.....F...........i............................................................ii.iii.............
[00:54:46] .........i..............................i...........................................................
[00:54:53] ....................................................................................................
[00:54:59] ...........i........................................................................................
[00:55:06] ....................................................................................................
---
[00:57:09] failures:
[00:57:09] 
[00:57:09] ---- [compile-fail] compile-fail/array_const_index-0.rs stdout ----
[00:57:09]  
[00:57:09] error: /checkout/src/test/compile-fail/array_const_index-0.rs:12: unexpected warning: '12:1: 12:24: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: 1 unexpected errors found, 0 expected errors not found
[00:57:09] status: exit code: 101
[00:57:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/array_const_index-0.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/array_const_index-0.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/array_const_index-0.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:57:09] unexpected errors (from JSON output): [
[00:57:09]     Error {
[00:57:09]         line_num: 12,
[00:57:09]         kind: Some(
[00:57:09]             Warning
[00:57:09]         ),
[00:57:09]         msg: "12:1: 12:24: this constant cannot be used [const_err]"
[00:57:09] ]
[00:57:09] 
[00:57:09] 
[00:57:09] thread '[compile-fail] compile-fail/array_const_index-0.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[00:57:09] 
[00:57:09] ---- [compile-fail] compile-fail/array_const_index-1.rs stdout ----
[00:57:09]  
[00:57:09]  
[00:57:09] error: /checkout/src/test/compile-fail/array_const_index-1.rs:12: unexpected warning: '12:1: 12:21: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: 1 unexpected errors found, 0 expected errors not found
[00:57:09] status: exit code: 101
[00:57:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/array_const_index-1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/array_const_index-1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/array_const_index-1.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:57:09] unexpected errors (from JSON output): [
[00:57:09]     Error {
[00:57:09]         line_num: 12,
[00:57:09]         kind: Some(
[00:57:09]             Warning
[00:57:09]         ),
[00:57:09]         msg: "12:1: 12:21: this constant cannot be used [const_err]"
[00:57:09] ]
[00:57:09] 
[00:57:09] 
[00:57:09] thread '[compile-fail] compile-fail/array_const_index-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[00:57:09] ---- [compile-fail] compile-fail/const-err-early.rs stdout ----
[00:57:09]  
[00:57:09]  
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:13: unexpected error: '13:1: 13:33: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:16: unexpected error: '16:1: 16:33: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:18: unexpected error: '18:1: 18:29: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:20: unexpected error: '20:1: 20:37: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:22: unexpected error: '22:1: 22:28: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:13: expected error not found: E0080
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:16: expected error not found: E0080
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:18: expected error not found: E0080
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:20: expected error not found: E0080
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-early.rs:22: expected error not found: E0080
[00:57:09] 
[00:57:09] error: 5 unexpected errors found, 5 expected errors not found
[00:57:09] status: exit code: 101
[00:57:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-err-early.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err-early.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err-early.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:57:09] unexpected errors (from JSON output): [
[00:57:09]     Error {
[00:57:09]         line_num: 13,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "13:1: 13:33: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 16,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "16:1: 16:33: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 18,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "18:1: 18:29: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 20,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "20:1: 20:37: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 22,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "22:1: 22:28: this constant cannot be used [const_err]"
[00:57:09] ]
[00:57:09] 
[00:57:09] not found errors (from test file): [
[00:57:09]     Error {
[00:57:09]     Error {
[00:57:09]         line_num: 13,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "E0080"
[00:57:09]     Error {
[00:57:09]         line_num: 16,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "E0080"
[00:57:09]     Error {
[00:57:09]         line_num: 18,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "E0080"
[00:57:09]     Error {
[00:57:09]         line_num: 20,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "E0080"
[00:57:09]     Error {
[00:57:09]         line_num: 22,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "E0080"
[00:57:09] ]
[00:57:09] 
[00:57:09] 
[00:57:09] thread '[compile-fail] compile-fail/const-err-early.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[00:57:09] ---- [compile-fail] compile-fail/const-err-multi.rs stdout ----
[00:57:09]  
[00:57:09]  
[00:57:09] error: /checkout/src/test/compile-fail/const-err-multi.rs:13: unexpected error: '13:1: 13:33: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-multi.rs:17: unexpected error: '17:1: 17:21: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-multi.rs:19: unexpected error: '19:1: 19:27: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-multi.rs:21: unexpected error: '21:1: 21:26: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-multi.rs:17: expected error not found: E0080
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-multi.rs:19: expected error not found: E0080
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-err-multi.rs:21: expected error not found: E0080
[00:57:09] 
[00:57:09] error: 4 unexpected errors found, 3 expected errors not found
[00:57:09] status: exit code: 101
[00:57:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-err-multi.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err-multi.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err-multi.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:57:09] unexpected errors (from JSON output): [
[00:57:09]     Error {
[00:57:09]         line_num: 13,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "13:1: 13:33: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 17,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "17:1: 17:21: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 19,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "19:1: 19:27: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 21,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "21:1: 21:26: this constant cannot be used [const_err]"
[00:57:09] ]
[00:57:09] 
[00:57:09] not found errors (from test file): [
[00:57:09]     Error {
[00:57:09]     Error {
[00:57:09]         line_num: 17,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "E0080"
[00:57:09]     Error {
[00:57:09]         line_num: 19,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "E0080"
[00:57:09]     Error {
[00:57:09]         line_num: 21,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "E0080"
[00:57:09] ]
[00:57:09] 
[00:57:09] 
[00:57:09] thread '[compile-fail] compile-fail/const-err-multi.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[00:57:09] ---- [compile-fail] compile-fail/const-eval-overflow2.rs stdout ----
[00:57:09]  
[00:57:09]  
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:24: unexpected error: '24:1: 29:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:31: unexpected error: '31:1: 36:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:38: unexpected error: '38:1: 43:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:45: unexpected error: '45:1: 50:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:52: unexpected error: '52:1: 57:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:59: unexpected error: '59:1: 63:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:65: unexpected error: '65:1: 69:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:71: unexpected error: '71:1: 76:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:26: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:33: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:40: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:47: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:54: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:60: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:66: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2.rs:73: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: 8 unexpected errors found, 8 expected errors not found
[00:57:09] status: exit code: 101
[00:57:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-eval-overflow2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-eval-overflow2.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-eval-overflow2.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:57:09] unexpected errors (from JSON output): [
[00:57:09]     Error {
[00:57:09]         line_num: 24,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "24:1: 29:8: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 31,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "31:1: 36:8: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 38,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "38:1: 43:8: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 45,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:57:09]         ),
[00:57:09]         msg: "45:1: 50:8: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 52,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "52:1: 57:8: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 59,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "59:1: 63:8: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 65,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "65:1: 69:8: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 71,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "71:1: 76:8: this constant cannot be used [const_err]"
[00:57:09] ]
[00:57:09] 
[00:57:09] not found errors (from test file): [
[00:57:09]     Error {
[00:57:09]     Error {
[00:57:09]         line_num: 26,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 33,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 40,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 47,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 54,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 60,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 66,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 73,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09] ]
[00:57:09] 
[00:57:09] 
[00:57:09] thread '[compile-fail] compile-fail/const-eval-overflow2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[00:57:09] ---- [compile-fail] compile-fail/const-eval-overflow2b.rs stdout ----
[00:57:09]  
[00:57:09]  
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:24: unexpected error: '24:1: 29:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:31: unexpected error: '31:1: 36:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:38: unexpected error: '38:1: 43:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:45: unexpected error: '45:1: 50:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:52: unexpected error: '52:1: 57:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:59: unexpected error: '59:1: 63:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:65: unexpected error: '65:1: 69:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:71: unexpected error: '71:1: 76:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:26: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:33: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:40: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:47: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:54: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:60: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:66: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2b.rs:73: expected error not found: constant evaluation error
[00:57:09] 
[00:57:09] error: 8 unexpected errors found, 8 expected errors not found
[00:57:09] status: exit code: 101
[00:57:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-eval-overflow2b.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-eval-overflow2b.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-eval-overflow2b.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:57:09] unexpected errors (from JSON output): [
[00:57:09]     Error {
[00:57:09]         line_num: 24,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "24:1: 29:8: this constant cannot be used [const_err]"
[00:57:09]     Error {
[00:57:09]         line_num: 31,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "31:1: 36:8: this constant cannot be use00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 33,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 40,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 47,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 54,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 60,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 66,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09]     Error {
[00:57:09]         line_num: 73,
[00:57:09]         kind: Some(
[00:57:09]             Error
[00:57:09]             Error
[00:57:09]         ),
[00:57:09]         msg: "constant evaluation error"
[00:57:09] ]
[00:57:09] 
[00:57:09] 
[00:57:09] thread '[compile-fail] compile-fail/const-eval-overflow2b.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[00:57:09] ---- [compile-fail] compile-fail/const-eval-overflow2c.rs stdout ----
[00:57:09]  
[00:57:09]  
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2c.rs:24: unexpected error: '24:1: 29:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2c.rs:31: unexpected error: '31:1: 36:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2c.rs:38: unexpected error: '38:1: 43:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2c.rs:45: unexpected error: '45:1: 50:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2c.rs:52: unexpected error: '52:1: 57:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2c.rs:59: unexpected error: '59:1: 63:8: this constant cannot be used [const_err]'
[00:57:09] 
[00:57:09] error: /checkout/src/test/compile-fail/const-eval-overflow2c.rs:65: unexpected error: '65:1: 69:8: this constant cannot be used [const_err]'
---
[00:57:09] test result: FAILED. 2288 passed; 8 failed; 15 ignored; 0 measured; 0 filtered out
[00:57:09] 
[00:57:09] 
[00:57:09] 
[00:57:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:09] 
[00:57:09] 
[00:57:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:09] Build completed unsuccessfully in 0:17:17
[00:57:09] Build completed unsuccessfully in 0:17:17
[00:57:09] make: *** [check] Error 1
[00:57:09] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0042ceac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
