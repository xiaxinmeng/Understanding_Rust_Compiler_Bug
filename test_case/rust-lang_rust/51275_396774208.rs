plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:07] 
[01:00:07] running 89 tests
highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirOptimized(add_else_branch)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:110:1\n   |\nLL | / pub fn add_else_branch(x: bool) -> u32 {\nLL | |     let mut ret = 1;\nLL | |\nLL | |     if x {\n...  |\nLL | |     ret\nLL | | }\n   | |_^\n\n"}
[01:00:29] {"message":"`MirValidated(add_else_branch)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":2526,"byte_end":2646,"line_start":110,"line_end":119,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_else_branch(x: bool) -> u32 {","highlight_start":1,"highlight_end":41},{"text":"    let mut ret = 1;","highlight_start":1,"highlight_end":21},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    if x {","highlight_start":1,"highlight_end":11},{"text":"        ret = 2;","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    ret","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirValidated(add_else_branch)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_exf_let)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":4393,"byte_end":4541,"line_start":207,"line_end":216,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_else_branch_if_let(x: Option<u32>) -> u32 {","highlight_start":1,"highlight_end":55},{"text":"    let mut ret = 1;","highlight_start":1,"highlight_end":21},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    if let Some(x) = x {","highlight_start":1,"highlight_end":25},{"text":"        ret = x;","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    ret","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirOptimized(add_else_branch_if_let)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:207:1\n   |\nLL | / pub fn add_else_branch_if_let(x: Option<u32>) -> u32 {\nLL | |     let mut ret = 1;\nLL | |\nLL | |     if let Some(x) = x {\n...  |\nLL | |     ret\nLL | | }\n   | |_^\n\n"}
[01:00:29] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:00:29] ------------------------------------------
[01:00:29] 
[01:00:29] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:00:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:29] 
[01:00:29] ---- [incremental] incremental/hashes/trait_defs.rs stdout ----
[01:00:29] 
[01:00:29] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:00:29] status: exit code: 101
[01:00:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/trait_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/auxiliary"
[01:00:29] ------------------------------------------
[01:00:29] 
[01:00:29] ------------------------------------------
[01:00:29] stderr:
[01:00:29] stderr:
[01:00:29] ------------------------------------------
[01:00:29] {"message":"`Hir(TraitChangeModeSelfOwnToMut::method)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/trait_defs.rs","byte_start":7313,"byte_end":7335,"line_start":277,"line_end":277,6_64-unknown-linux-gnu/stage1-rustc/release/build
34840 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
34372 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
33884 ./src/llvm-emscripten/lib/Target
---
travis_time:end:26529e85:start=1528849055005141390,finish=1528849055012084695,duration=6943305
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:132a6624
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cf47ccd
$ dmesg | grep -i kill
