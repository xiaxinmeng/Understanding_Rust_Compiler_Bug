plain
travis_time:end:00532620:start=1546554357768963385,finish=1546554361203906509,duration=3434943124
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:01] tidy error: /checkout/src/librustc_codegen_ssa/mir/mod.rs:587: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_codegen_ssa/mir/mod.rs:588: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_codegen_ssa/mir/block.rs:590: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_codegen_ssa/mir/block.rs:591: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_codegen_ssa/base.rs:236: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:49: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:277: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:1134: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:1174: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:1175: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:1346: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:1347: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/lib.rs:3292: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_resolve/lib.rs:3293: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_codegen_llvm/back/write.rs:605: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_codegen_llvm/consts.rs:284: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/libcore/iter/mod.rs:1924: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/libcore/iter/iterator.rs:47: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/libcore/iter/iterator.rs:60: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/libcore/iter/iterator.rs:66: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/libcore/iter/iterator.rs:84: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_lint/lib.rs:310: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:288: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_mir/hair/pattern/_match.rs:1282: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_mir/hair/pattern/_match.rs:1283: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_mir/const_eval.rs:723: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_mir/interpret/operand.rs:454: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_mir/monomorphize/collector.rs:859: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustdoc/core.rs:512: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustdoc/clean/blanket_impl.rs:130: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/libsyntax/parse/parser.rs:7988: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/libsyntax_ext/test.rs:147: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc/ty/constness.rs:27: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc/middle/mem_categorization.rs:96: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc/traits/mod.rs:459: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc/infer/canonical/query_response.rs:51: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc/infer/error_reporting/nice_region_error/outlives_closure.rs:78: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc/infer/error_reporting/nice_region_error/outlives_closure.rs:95: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc/infer/error_reporting/nice_region_error/outlives_closure.rs:99: line longer than 100 chars
[00:03:01] tidy error: /checkout/src/librustc_typeck/astconv.rs:2176: line longer than 100 chars
[00:03:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:03:01] 
[00:03:01] 
[00:03:01] 
[00:03:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:01] 
[00:03:01] 
[00:03:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:01] Build completed unsuccessfully in 0:00:41
[00:03:01] Build completed unsuccessfully in 0:00:41
[00:03:01] Makefile:69: recipe for target 'tidy' failed
[00:03:01] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10d58a1c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan  3 22:29:11 UTC 2019
---
travis_time:end:21514100:start=1546554552052851296,finish=1546554552057135781,duration=4284485
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:144589b6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20281618
travis_time:start:20281618
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00e3e928
$ dmesg | grep -i kill
