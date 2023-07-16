plain
[00:50:34] ....................................................................................................
[00:50:39] ...i..............................................................................i.................
[00:50:44] ....................................................................................................
[00:50:49] ....................................................................................................
[00:50:55] .......................................................................................F............
[00:51:02] failures:
[00:51:02] 
[00:51:02] ---- [ui (nll)] ui/suggestions/issue-51244.rs stdout ----
[00:51:02] diff of stderr:
[00:51:02] diff of stderr:
[00:51:02] 
[00:51:02] 2   --> $DIR/issue-51244.rs:13:5
[00:51:02] 3    |
[00:51:02] 4 LL |     let ref my_ref @ _ = 0;
[00:51:02] -    |         -------------- help: use a mutable reference instead: `ref mut my_ref @ _`
[00:51:02] +    |         -------------- help: consider changing this to be a mutable reference: `&mut ef my_ref @ _`
[00:51:02] 6 LL |     *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]
[00:51:02] 8 
[00:51:02] 
[00:51:02] 
[00:51:02] The actual stderr differed from the expected stderr.
[00:51:02] The actual stderr differed from the expected stderr.
[00:51:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/issue-51244.nll.stderr
[00:51:02] To update references, rerun the tests and pass the `--bless` flag
[00:51:02] To only update this specific test, also pass `--test-args suggestions/issue-51244.rs`
[00:51:02] error: 1 errors occurred comparing output.
[00:51:02] status: exit code: 101
[00:51:02] status: exit code: 101
[00:51:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/auxiliary" "-A" "unused"
[00:51:02] ------------------------------------------
[00:51:02] 
[00:51:02] ------------------------------------------
[00:51:02] stderr:
[00:51:02] stderr:
[00:51:02] ------------------------------------------
[00:51:02] {"message":"cannot assign to data in a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":511,"byte_end":522,"line_start":13,"line_end":13,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":487,"byte_end":501,"line_start":12,"line_end":12,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"    let ref my_ref @ _ = 0;","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":"&mut ef my_ref @ _","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/suggestions/issue-51244.rs:13:5\n   |\nLL |     let ref my_ref @ _ = 0;\n   |         -------------- help: consider changing this to be a mutabl0 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
51360 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
50792 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
50220 ./obj/build/x86_64-unknown-linux-gnu/test
---
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
34372 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
33884 ./src/llvm-emscripten/lib/Target
32936 ./.git/modules/src/libcompiler_builtins
32172 ./.git/modules/src/libcompil[0Ktravis_time:start:178d8b44
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2a5a37a8
$ dmesg | grep -i kill
