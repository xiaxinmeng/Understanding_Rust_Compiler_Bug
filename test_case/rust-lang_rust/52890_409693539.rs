plain
[00:57:27] travis_fold:start:test_stage1-term
travis_time:start:test_stage1-term
Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:27]    Compiling term v0.0.0 (file:///checkout/src/libterm)
[00:57:27] error: unused import: `test_get_dbpath_for_term`
[00:57:27]   --> libterm/terminfo/searcher.rs:80:4
[00:57:27] 80 | fn test_get_dbpath_for_term() {
[00:57:27]    |    ^^^^^^^^^^^^^^^^^^^^^^^^
[00:57:27]    |
[00:57:27]    = note: `-D unused-imports` implied by `-D warnings`
[00:57:27]    = note: `-D unused-imports` implied by `-D warnings`
[00:57:27] 
[00:57:27] error: unused import: `test_veclens`
[00:57:27]    --> libterm/terminfo/parser/compiled.rs:351:8
[00:57:27] 351 |     fn test_veclens() {
[00:57:27]     |        ^^^^^^^^^^^^
[00:57:27] 
[00:57:27] 
[00:57:27] error: unused import: `test_basic_setabf`
[00:57:27]    --> libterm/terminfo/parm.rs:545:8
[00:57:27] 545 |     fn test_basic_setabf() {
[00:57:27]     |        ^^^^^^^^^^^^^^^^^
[00:57:27] 
[00:57:27] 
[00:57:27] error: unused import: `test_multiple_int_constants`
[00:57:27]    --> libterm/terminfo/parm.rs:552:8
[00:57:27] 552 |     fn test_multiple_int_constants() {
[00:57:27]     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:57:27] 
[00:57:27] 
[00:57:27] error: unused import: `test_op_i`
[00:57:27]    --> libterm/terminfo/parm.rs:558:8
[00:57:27] 558 |     fn test_op_i() {
[00:57:27]     |        ^^^^^^^^^
[00:57:27] 
[00:57:27] error: unused import: `test_param_stack_failure_conditions`
---
[00:57:27]     |
[00:57:27] 618 |     fn test_push_bad_param() {
[00:57:27]     |        ^^^^^^^^^^^^^^^^^^^
[00:57:27] 
[00:57:27] error: unused import: `test_comparison_ops`
[00:57:27]    --> libterm/terminfo/parm.rs:623:8
[00:57:27] 623 |     fn test_comparison_ops() {
[00:57:27]     |        ^^^^^^^^^^^^^^^^^^^
[00:57:27] 
[00:57:27] 
[00:57:27] error: unused import: `test_conditionals`
[00:57:27]    --> libterm/terminfo/parm.rs:642:8
[00:57:27] 642 |     fn test_conditionals() {
[00:57:27]     |        ^^^^^^^^^^^^^^^^^
[00:57:27] 
[00:57:27] 
[00:57:27] error: unused import: `test_format`
[00:57:27]    --> libterm/terminfo/parm.rs:657:8
[00:57:27] 657 |     fn test_format() {
[00:57:27]     |        ^^^^^^^^^^^
[00:57:27] 
[00:57:28] error: aborting due to 10 previous errors
[00:57:28] error: aborting due to 10 previous errors
[00:57:28] 
[00:57:28] error: Could not compile `term`.
[00:57:28] To learn more, run the command again with --verbose.
[00:57:28] 
[00:57:28] 
[00:57:28] 
[00:57:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "term" "--" "--quiet"
[00:57:28] 
[00:57:28] 
[00:57:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:28] Build completed unsuccessfully in 0:15:35
[00:57:28] Build completed unsuccessfully in 0:15:35
[00:57:28] Makefile:58: recipe for target 'check' failed
[00:57:28] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0770fe80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
