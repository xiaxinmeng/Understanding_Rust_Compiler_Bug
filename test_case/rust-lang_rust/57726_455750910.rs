plain
travis_time:end:09886fe0:start=1547870573569843708,finish=1547870575825267427,duration=2255423719
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:49] 
[01:12:49] running 118 tests
[01:13:17] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:13:22] ......iii.i.....ii
[01:13:22] 
[01:13:22]  finished in 33.650
[01:13:22] travis_fold:end:test_debuginfo

---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:23] 
[01:13:23] running 25 tests
[01:14:00] ....FFFFFFFFFFFF.........
[01:14:00] 
[01:14:00] ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
[01:14:00] 
[01:14:00] 
[01:14:00] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint_for_crate.rs" failed to compile: 
[01:14:00] status: exit code: 1
[01:14:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint_for_crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
[01:14:00] ------------------------------------------
[01:14:00] 
[01:14:00] ------------------------------------------
[01:14:00] stderr:
[01:14:00] stderr:
[01:14:00] ------------------------------------------
[01:14:00] {"message":"unused import: `LateLintPassObject`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/lint_for_crate.rs","byte_start":235,"byte_end":253,"line_start":10,"line_end":10,"column_start":69,"column_end":87,"is_primary":true,"text":[{"text":"use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};","highlight_start":69,"highlight_end":87}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `LateLintPassObject`\n  --> /checkout/src/test/ui-fulldeps/auxiliary/lint_for_crate.rs:10:69\n   |\nLL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};\n   |                                                                     ^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:14:00] {"message":"not all trait items implemented, missing: `name`","code":{"code":"E0046","explanation":"\nItems are missing in a trait implementation. Erroneous code example:\n\n