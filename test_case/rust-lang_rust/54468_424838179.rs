plain
[00:07:48]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:13:45]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:45]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:45]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:45] error: unexpected close delimiter: `}`
[00:13:45]    --> librustc_mir/borrow_check/nll/type_check/mod.rs:178:1
[00:13:45] 178 | }
[00:13:45]     | ^
[00:13:45] 
[00:13:45] error: aborting due to previous error
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148656 ./obj/build/bootstrap/debug/incremental
134216 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q
134212 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q/s-f566bw7892-edggz3-abll1u66rzrn
111072 ./src/llvm/test/CodeGen
104700 ./src/tools/lldb
98936 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
---
travis_time:end:2240325c:start=1537989599838922659,finish=1537989599843932918,duration=5010259
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03f545df
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10c2a9c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/
