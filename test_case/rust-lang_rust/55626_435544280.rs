plain
travis_time:end:00df9bc3:start=1541201148148720228,finish=1541201211740543880,duration=63591823652
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/bc342a475c09b6df8004d518382e6d5b6bcb49f7.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm-emscripten.tar.gz &&         curl -sSL -o download-src-llvm-emscripten.tar.gz https://github.com/kripken/emscripten-fastcomp/archive/272d3ff91b38eac051bdbaf6cf84db5c901ce2f8.tar.gz
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/jemalloc'
[00:00:00] Cleared directory 'src/libbacktrace'
---
[00:02:05] Successfully built b93c482bced4
[00:02:05] Successfully tagged rust-ci:latest
[00:02:05] Built container sha256:b93c482bced48c39fccb78e5116d5c636310a64824aff7a3ad58e868d39c29ea
[00:02:05] Uploading finished image to s3://rust-lang-ci-sccache2/docker/a417cc68142e4bf9128efe4949d1683dc75a696a66fcb9bd5fbca608a950d850ba9511a7420c7d0eae01f7ab8dd2fc3bf545f8bc696992231aa906370ca8d50a
[00:02:49] upload failed: - to s3://rust-lang-ci-sccache2/docker/a417cc68142e4bf9128efe4949d1683dc75a696a66fcb9bd5fbca608a950d850ba9511a7420c7d0eae01f7ab8dd2fc3bf545f8bc696992231aa906370ca8d50a Unable to locate credentials

[00:02:50] travis_time:end:196e9920:start=1541201282940948627,finish=1541201391085156684,duration=108144208057
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:50] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:48:17] .................................................................................................... 1400/4983
[00:48:19] .................................................................................i.................. 1500/4983
[00:48:22] ..................................................i................................................. 1600/4983
[00:48:26] .................................................................................................... 1700/4983
[00:48:30] ..................FF................................................................................ 1800/4983
[00:48:37] .................................................................................................... 2000/4983
[00:48:41] .................................................................................................... 2100/4983
[00:48:46] .................................................................................................... 2200/4983
[00:48:50] .................................................................................................... 2300/4983
---
[00:50:26] 1 error[E0668]: malformed inline assembly
[00:50:26] -   --> $DIR/inline-asm-bad-constraint.rs:29:9
[00:50:26] +   --> $DIR/inline-asm-bad-constraint.rs:31:9
[00:50:26] 3    |
[00:50:26] 4 LL |         asm!("" :"={rax"(rax)) //~ ERROR E0668
[00:50:26] 
[00:50:26] 6 
[00:50:26] 7 error[E0668]: malformed inline assembly
[00:50:26] -   ---
[00:50:26] -   ---
[00:50:26] diff of stderr:
[00:50:26] 
[00:50:26] 1 error[E0669]: invalid value for constraint in inline assembly
[00:50:26] +   --> $DIR/inline-asm-bad-operand.rs:31:24
[00:50:26] 3    |
[00:50:26] 3    |
[00:50:26] 4 LL |         asm!("" :: "r"("")); //~ ERROR E0669
[00:50:26] 
[00:50:26] 6 
[00:50:26] 6 
[00:50:26] 7 error[E0669]: invalid value for constraint in inline assembly
[00:50:26] +   --> $DIR/inline-asm-bad-operand.rs:36:32
[00:50:26] 9    |
[00:50:26] 9    |
[00:50:26] 10 LL |         asm!("ret" : : "{rdi}"(target)); //~ ERROR E0669
[00:50:26] 
[00:50:26] 12 
[00:50:26] 12 
[00:50:26] 13 error[E0669]: invalid value for constraint in inline assembly
[00:50:26] +   --> $DIR/inline-asm-bad-operand.rs:43:29
[00:50:26] 15    |
[00:50:26] 15    |
[00:50:26] 16 LL |     unsafe { asm!("" :: "i"(hello)) }; //~ ERROR E0669
[00:50:26] 
[00:50:26] 18 
[00:50:26] 18 
[00:50:26] 19 error[E0669]: invalid value for constraint in inline assembly
[00:50:26] +   --> $DIR/inline-asm-bad-operand.rs:51:38
[00:50:26] 21    |
[00:50:26] 21    |
[00:50:26] 22 LL |         asm!("movups $1, %xmm0"::"m"(arr)); //~ ERROR E0669
[00:50:26] 
[00:50:26] 24 
[00:50:26] 24 
[00:50:26] 25 error[E0669]: invalid value for constraint in inline assembly
[00:50:26] -   --> $DIR/inline-asm-bad-operand.rs:56:32
[00:50t" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-asm-bad-operand/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-asm-bad-operand/auxiliary" "-A" "unused"
[00:50:26] ------------------------------------------
[00:50:26] 
[00:50:26] ------------------------------------------
[00:50:26] stderr:
[00:50:26] stderr:
[00:50:26] ------------------------------------------
[00:50:26] {"message":"invalid value for constraint in inline assembly","code":{"code":"E0669","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/inline-asm-bad-operand.rs","byte_start":802,"byte_end":804,"line_start":31,"line_end":31,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"        asm!(\"\" :: \"r\"(\"\")); //~ ERROR E0669","highlight_start":24,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0669]: invalid value for constraint in inline assembly\n  --> /checkout/src/test/ui/inline-asm-bad-operand.rs:31:24\n   |\nLL |         asm!(\"\" :: \"r\"(\"\")); //~ ERROR E0669\n   |                        ^^\n\n"}
[00:50:26] {"message":"invalid value for constraint in inline assembly","code":{"code":"E0669","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/inline-asm-bad-operand.rs","byte_start":906,"byte_end":912,"line_start":36,"line_end":36,"column_start":32,"column_end":38,"is_pri---------------------------------------
[00:50:26] thread '[ui] ui/inline-asm-bad-operand.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:50:26] 
[00:50:26] 
[00:50:26] failures:
[00:50:26] failures:
[00:50:26]     [ui] ui/inline-asm-bad-constraint.rs
[00:50:26]     [ui] ui/inline-asm-bad-operand.rs
[00:50:26] 
[00:50:26] test result: FAILED. 4957 passed; 2 failed; 24 ignored; 0 measured; 0 filtered out
[00:50:26] 
[00:50:26] 
[00:50:26] 
[00:50:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--anbootstrap-zemjd6kcyh2u
134760 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6b65xhqpq-6w5r22-3qkaut9jf50qa
130756 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130752 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123680 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
112720 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
