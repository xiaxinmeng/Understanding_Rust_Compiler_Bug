plain
[00:47:35] ....................................................................................................
[00:47:38] ...................................................................................................i
[00:47:41] ....................................................................................................
[00:47:44] ....................................................................................................
[00:47:46] ................................................iiiiiiiii...........................................
[00:47:52] ....................................................................................................
[00:47:55] ....................................................................................................
[00:47:58] .............................i......................................................................
[00:48:01] ...............................i...............................................i..i.ii..............
---
[00:58:51] ------------------------------------------
[00:58:51] error[E0433]: failed to resolve. Use of undeclared type or module `codemap`
[00:58:51]   --> /checkout/src/test/run-fail-fulldeps/qquote.rs:31:44
[00:58:51]    |
[00:58:51] 31 |     let ps = syntax::parse::ParseSess::new(codemap::FilePathMapping::empty());
[00:58:51]    |                                            ^^^^^^^ Use of undeclared type or module `codemap`
[00:58:51] warning: unused import: `syntax::ast`
[00:58:51]   --> /checkout/src/test/run-fail-fulldeps/qquote.rs:20:5
[00:58:51]    |
[00:58:51] 20 | use syntax::ast;
---
[00:58:51]    |
[00:58:51] 23 | use syntax::symbol::Symbol;
[00:58:51]    |     ^^^^^^^^^^^^^^^^^^^^^^
[00:58:51] 
[00:58:51] warning: unused import: `syntax_pos::DUMMY_SP`
[00:58:51]    |
[00:58:51] 24 | use syntax_pos::DUMMY_SP;
[00:58:51]    |     ^^^^^^^^^^^^^^^^^^^^
[00:58:51] 
---
[00:58:51] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:58:51] 
[00:58:51] 
[00:58:51] 
[00:58:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/nativ

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:206cba79
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:259aab54:start=1534545642543049119,finish=1534545642553296370,duration=10247251
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19598955
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout
