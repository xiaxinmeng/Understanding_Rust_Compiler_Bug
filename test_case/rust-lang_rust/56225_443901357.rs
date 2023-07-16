plain
travis_time:end:0ed60d5e:start=1543874120488566041,finish=1543874124111038987,duration=3622472946
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:33] 
[00:55:33] running 120 tests
[00:55:36] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:55:36] ..ii.i.....iiii.....
[00:55:36] 
[00:55:36]  finished in 3.474
[00:55:36] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:51] 
[00:55:51] running 118 tests
[00:56:16] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:56:19] ......iii.i.....ii
[00:56:19] 
[00:56:19]  finished in 28.665
[00:56:19] travis_fold:end:test_debuginfo

---
[00:56:22] failures:
[00:56:22] 
[00:56:22] ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
[00:56:22] 
[00:56:22] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs" failed to compile: 
[00:56:22] status: exit code: 1
[00:56:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
[00:56:22] ------------------------------------------
[00:56:22] 
[00:56:22] ------------------------------------------
[00:56:22] stderr:
[00:56:22] stderr:
[00:56:22] ------------------------------------------
[00:56:22] {"message":"unused import: `LateLintPassObject`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs","byte_start":737,"byte_end":755,"line_start":22,"line_end":22,"column_start":69,"column_end":87,"is_primary":true,"text":[{"text":"use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};","highlight_start":69,"highlight_end":87}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `LateLintPassObject`\n  --> /checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs:22:69\n   |\nLL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};\n   |                                                                     ^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[00:56:22] {"message":"no field `name` on type `&rustc::hir::Item`","code":{"code":"E0609","explanation":"\nAttempted to access a non-existent field in a struct.\n\nErroneous code example:\n\n