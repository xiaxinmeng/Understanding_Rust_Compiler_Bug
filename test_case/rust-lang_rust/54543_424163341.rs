plain
[00:44:23] travis_fold:end:stage2-rustdoc

[00:44:23] travis_time:end:stage2-rustdoc:start=1537833488550055619,finish=1537833488826169518,duration=276113899

[00:44:23] error: option `--index-page` argument must be a file
[00:44:23] 
[00:44:23] 
[00:44:23] 
[00:44:23] command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "--html-after-content" "/checkout/src/doc/footer.inc" "--html-before-content" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc/version_info.html" "--html-in-header" "/checkout/src/doc/favicon.inc" "--index-page" "src/doc/index.md" "--markdown-playground-url" "https://play.rust-lang.org/" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "/checkout/src/doc/rust.md" "--markdown-css" "rust.css"
[00:44:23] 
[00:44:23] 
[00:44:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:44:23] Build completed unsuccessfully in 0:04:09
[00:44:23] Build completed unsuccessfully in 0:04:09
[00:44:23] make: *** [all] Error 1
[00:44:23] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c2763fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
149112 ./src/llvm-emscripten/test
148852 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
148580 ./obj/build/bootstrap/debug/incremental
134140 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q
134136 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q/s-f545tnz7cs-y11ee0-3093t8bfyy2gb
