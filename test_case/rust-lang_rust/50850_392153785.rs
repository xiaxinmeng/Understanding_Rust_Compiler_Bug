plain

[00:04:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:45] tidy error: /checkout/src/libcore/future.rs: incorrect license
[00:04:45] tidy error: /checkout/src/librustc_metadata/encoder.rs:1240: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustdoc/html/render.rs:2606: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/libsyntax/visit.rs:230: trailing whitespace
[00:04:45] tidy error: /checkout/src/libsyntax/test.rs:130: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/libsyntax/parse/parser.rs:3315: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/libsyntax/print/pprust.rs:2161: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:300: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:832: trailing whitespace
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:833: trailing whitespace
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:839: TODO is deprecated; use FIXME
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:1788: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:1801: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:1806: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:1842: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:1856: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:2927: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc/hir/lowering.rs:3252: line longer than 100 chars
[00:04:46] tidy error: /checkout/src/libcore/future.rs:2: mismatches to corresponding lang feature in: ["tracking issue"]
[00:04:46] tidy error: /checkout/src/libcore/future.rs:5: mismatches to corresponding lang feature in: ["tracking issue"]
[00:04:46] tidy error: /checkout/src/libstd/lib.rs:394: mismatches to corresponding lang feature in: ["tracking issue"]
[00:04:46] Expected a gate test for the feature 'async_await'.
[00:04:46] tidy error: Found 1 features without a gate test.
[00:04:46] Hint: create a failing test file named 'feature-gate-async_await.rs'
[00:04:46]       in the 'ui' test suite, with its failures due to
[00:04:46]       missing usage of #![feature(async_await)].
[00:04:46] Hint: If you already have such a test and don't want to rename it,
[00:04:46]       you can also add a // gate-test-async_await line to the test file.
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:01:49
[00:04:47] Build completed unsuccessfully in 0:01:49
[00:04:47] make: *** [tidy] Error 1
[00:04:47] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14703aaa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
