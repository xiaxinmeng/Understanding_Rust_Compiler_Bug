plain
travis_time:end:06f21b4c:start=1547875687958733691,finish=1547875688840566040,duration=881832349
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
[01:15:48] 
[01:15:48] running 118 tests
[01:16:13] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:16:17] ......iii.i.....ii
[01:16:17] 
[01:16:17]  finished in 28.442
[01:16:17] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:57] 
[01:16:57] running 60 tests
[01:19:04] ...........................F...............................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:21:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:21:30] failures:
[01:21:30] 
[01:21:30] ---- [run-pass] run-pass-fulldeps/issue-40001.rs stdout ----
[01:21:30] ---- [run-pass] run-pass-fulldeps/issue-40001.rs stdout ----
[01:21:30] 
[01:21:30] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
[01:21:30] status: exit code: 1
[01:21:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-40001/auxiliary"
[01:21:30] ------------------------------------------
[01:21:30] 
[01:21:30] ------------------------------------------
[01:21:30] stderr:
[01:21:30] stderr:
[01:21:30] ------------------------------------------
[01:21:30] {"message":"unused import: `syntax::ext::base::*`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":223,"byte_end":243,"line_start":11,"line_end":11,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"use syntax::ext::base::*;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `syntax::ext::base::*`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:11:5\n   |\nLL | use syntax::ext::base::*;\n   |     ^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:21:30] {"message":"unused import: `syntax::symbol::Symbol`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":303,"byte_end":325,"line_start":13,"line_end":13,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"use syntax::symbol::Symbol;","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `syntax::symbol::Symbol`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:13:5\n   |\nLL | use syntax::symbol::Symbol;\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:21:30] {"message":"unused import: `rustc::hir::map as hir_map`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":376,"byte_end":402,"line_start":17,"line_end":17,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"use rustc::hir::map as hir_map;","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `rustc::hir::map as hir_map`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:17:5\n   |\nLL | use rustc::hir::map as hir_map;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:21:30] {"message":"unused import: `rustc::ty`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":503,"byte_end":512,"line_start":20,"line_end":20,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"use rustc::ty;","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `rustc::ty`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:20:5\n   |\nLL | use rustc::ty;\n   |     ^^^^^^^^^\n\n"}
[01:21:30] {"message":"not all trait items implemented, missing: `name`","code":{"code":"E0046","explanation":"\nItems are missing in a trait implementation. Erroneous code example:\n\n