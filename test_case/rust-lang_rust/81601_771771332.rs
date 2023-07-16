plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.060 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.97s

 finished in 2.029 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished test [unoptimized + debuginfo] target(s) in 14.41s
     Running /checkout/obj/build/bootstrap/debug/deps/bootstrap-0b1f5c210906548f

running 17 tests
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::defaults::build_cross_compile ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::defaults::build_stage_0 ... ok
test builder::tests::defaults::build_stage_0 ... ok
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::defaults::doc_default ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::build_all ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::build_with_empty_host ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::dist_baseline ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::dist_only_cross_host ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::dist_with_empty_host ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::dist_with_hosts ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::dist_with_same_targets_and_hosts ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::dist_with_targets ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::dist_with_targets_and_hosts ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::doc_ci ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::test_docs ... FAILED
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
test builder::tests::dist::test_with_no_doc_stage0 ... ok

failures:


---- builder::tests::defaults::build_cross_compile stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9

---- builder::tests::defaults::build_default stdout ----
---- builder::tests::defaults::build_default stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::defaults::doc_default stdout ----
---- builder::tests::defaults::doc_default stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::build_all stdout ----
---- builder::tests::dist::build_all stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::build_with_empty_host stdout ----
---- builder::tests::dist::build_with_empty_host stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::dist_baseline stdout ----
---- builder::tests::dist::dist_baseline stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::dist_only_cross_host stdout ----
---- builder::tests::dist::dist_only_cross_host stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::dist_with_empty_host stdout ----
---- builder::tests::dist::dist_with_empty_host stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::dist_with_hosts stdout ----
---- builder::tests::dist::dist_with_hosts stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::dist_with_targets stdout ----
---- builder::tests::dist::dist_with_targets stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::doc_ci stdout ----
---- builder::tests::dist::doc_ci stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::test_docs stdout ----
---- builder::tests::dist::test_docs stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
---- builder::tests::dist::test_exclude stdout ----
Skipping Set({"src/tools/tidy"}) because it is excluded
Suite("src/test/ui") not skipped for "bootstrap::test::Ui" -- not in ["src/tools/tidy"]
Suite("src/test/ui") not skipped for "bootstrap::test::Ui" -- not in ["src/tools/tidy"]
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9

failures:
    builder::tests::defaults::build_cross_compile
    builder::tests::defaults::build_default
