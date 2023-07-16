\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-access-permissions.rs","byte_start":2082,"byte_end":2097,"line_start":66,"line_end":66,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"        let _y = &mut *foo_ref.f; //[ast]~ ERROR [E0389]","highlight_start":18,"highlight_end":33}],"label":"`foo_ref` is a `&` reference, so the data it refers to cannot be borrowed as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-access-permissions.rs","byte_start":2059,"byte_end":2063,"line_start":65,"line_end":65,"column_start":23,"column_end":27,"is_primary":true,"text":[{"text":"        let foo_ref = &foo;","highlight_start":23,"highlight_end":27}],"label":null,"suggested_replacement":"&mut foo","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow `*foo_ref.f` as mutable, as it is behind a `&` reference\n  --> /checkout/src/test/ui/borrowck/borrowck-access-permissions.rs:66:18\n   |\nLL |         let foo_ref = &foo;\n   |                       ---- help: consider changing this to be a mutable reference: `&mut foo`\nLL |         let _y = &mut *foo_ref.f; //[ast]~ ERROR [E0389]\n   |                  ^^^^^^^^^^^^^^^ `foo_ref` is a `&` reference, so the data it refers to cannot be borrowed as mutable\n\n"}
[00:51:19] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[00:51:19] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:51:19] ------------------------------------------
[00:51:19] 
[00:51:19] thread '[ui] ui/borrowck/borrowck-access-permissions.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:19] 
[00:51:19] ---- [ui] ui/nll/relate_tys/hr-fn-aaa-as-aba.rs stdout ----
[00:51:19] diff of stderr:
[00:51:19] 
[00:51:19] 4 LL |     let a: for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32 = make_it();
[00:51:19] 6 
[00:51:19] - error: higher-ranked subtype error
[00:51:19] -   --> $DIR/hr-fn-aaa-as-aba.rs:24:9
[00:51:19] -    |
[00:51:19] -    |
[00:51:19] - LL |     let a: for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32 = make_it();
[00:51:19] - 
[00:51:19] - error: aborting due to 2 previous errors
[00:51:19] + error: aborting due to previous error
[00:51:19] 14 
[00:51:19] 14 
[00:51:19] 15 
[00:51:19] 
[00:51:19] 
[] The actual stderr differed from the expected stderr.
[00:51:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/universe-violation/universe-violation.stderr
[00:51:19] To update references, rerun the tests and pass the `--bless` flag
[00:51:19] To only update this specific test, also pass `--test-args nll/relate_tys/universe-violation.rs`
[00:51:19] error: 1 errors occurred comparing output.
[00:51:19] status: exit code: 1
[00:51:19] status: exit code: 1
[00:51:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/relate_tys/universe-violation.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/universe-violation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zno-leak-check" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/universe-violation/auxiliary" "-A" "unused"
[00:51:19] ------------------------------------------
[00:51:19] 
[00:51:19] ------------------------------------------
[00:51:19] stderr:
[00:51:19] stderr:
[00:51:19] ------------------------------------------
[00:51:19] {"message":"higher-ranked subtype error","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/relate_tys/universe-violation.rs","byte_start":368,"byte_end":369,"line_start":15,"line_end":15,"column_start":31,"column_end":32,"is_primary":true,"text":[{"text":"    let b: fn(&u32) -> &u32 = a;","highlight_start":31,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: higher-ranked subtype error\n  --> /checkout/src/test/ui/nll/relate_tys/universe-violation.rs:15:31\n   |\nLL |     let b: fn(&u32) -> &u32 = a;\n   |                               ^\n\n"}
[00:51:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:19] ------------------------------------------
[00:51:19] 
[00:51:19] thread '[ui] ui/nll/relate_tys/universe-violation.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:19] 
---
[00:51:19] 
[00:51:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:51:19] 
[00:51:19] 
[00:51:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" 4620064 .
2368424 ./obj/build
1752036 ./obj/build/x86_64-unknown-linux-gnu
1185624 ./.git
1065464 ./src
---
151200 ./src/tools/clang
149128 ./src/llvm-emscripten/test
148964 ./obj/build/bootstrap/debug/incremental
134532 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134528 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f4j32t0se5-1cuox4y-29k09wkatlb53
130184 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
129880 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
129876 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
127264 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
---
travis_time:end:024ed8d6:start=1536182314691736281,finish=1536182314699153346,duration=7417065
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10dd7b2f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:163da20d
travis_time:start:163da20d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07a7208c
$ dmesg | grep -i kill
