plain
travis_time:end:3d9e10e0:start=1550158341153138694,finish=1550158343629805017,duration=2476666323
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
[01:08:27] 
[01:08:27] running 119 tests
[01:08:51] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:08:54] i......iii.i.....ii
[01:08:54] 
[01:08:54]  finished in 27.683
[01:08:54] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:01] 
[01:09:01] running 47 tests
[01:10:30] ........................F.....................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:12:26] failures:
[01:12:26] 
[01:12:26] ---- [run-pass] run-pass-fulldeps/issue-40001.rs stdout ----
[01:12:26] 
[01:12:26] 
[01:12:26] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
[01:12:26] status: exit code: 1
[01:12:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-40001/auxiliary"
[01:12:26] ------------------------------------------
[01:12:26] 
[01:12:26] ------------------------------------------
[01:12:26] stderr:
[01:12:26] stderr:
[01:12:26] ------------------------------------------
[01:12:26] {"message":"unused import: `syntax::ext::base::*`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":223,"byte_end":243,"line_start":11,"line_end":11,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"use syntax::ext::base::*;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":219,"byte_end":244,"line_start":11,"line_end":11,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"use syntax::ext::base::*;","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `syntax::ext::base::*`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:11:5\n   |\nLL | use syntax::ext::base::*;\n   |     ^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:12:26] {"message":"unused import: `syntax::symbol::Symbol`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":303,"byte_end":325,"line_start":13,"line_end":13,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"use syntax::symbol::Symbol;","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":299,"byte_end":326,"line_start":13,"line_end":13,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"use syntax::symbol::Symbol;","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `syntax::symbol::Symbol`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:13:5\n   |\nLL | use syntax::symbol::Symbol;\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:12:26] {"message":"unused import: `rustc::hir::map as hir_map`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":376,"byte_end":402,"line_start":17,"line_end":17,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"use rustc::hir::map as hir_map;","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":372,"byte_end":403,"line_start":17,"line_end":17,"column_start":1,"column_end":32,"is_primary":true,"text":[{"text":"use rustc::hir::map as hir_map;","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `rustc::hir::map as hir_map`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:17:5\n   |\nLL | use rustc::hir::map as hir_map;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:12:26] {"message":"unused import: `rustc::ty`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":503,"byte_end":512,"line_start":20,"line_end":20,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"use rustc::ty;","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs","byte_start":499,"byte_end":513,"line_start":20,"line_end":20,"column_start":1,"column_end":15,"is_primary":true,"text":[{"text":"use rustc::ty;","highlight_start":1,"highlight_end":15}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `rustc::ty`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:20:5\n   |\nLL | use rustc::ty;\n   |     ^^^^^^^^^\n\n"}
[01:12:26] {"message":"method `check_fn` has an incompatible type for trait","code":{"code":"E0053","explanation":"\nThe parameters of any trait method must match between a trait implementation\nand the trait definition.\n\nHere are a couple examples of this error:\n\n