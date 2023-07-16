plain
[00:46:05] ........................................i...........................................................
[00:46:08] ..............................i.....................................................................
[00:46:11] ....................................................................................................
[00:46:15] ....................................................................................................
[00:46:18] ..............F..................................i..................................................
[00:46:20] failures:
[00:46:20] 
[00:46:20] ---- [ui (nll)] ui/span/slice-borrow.rs stdout ----
[00:46:20] diff of stderr:
[00:46:20] diff of stderr:
[00:46:20] 
[00:46:20] 3    |
[00:46:20] 4 LL |         let x: &[isize] = &vec![1, 2, 3, 4, 5];
[00:46:20] 5    |                            ^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:46:20] - LL |         y = &x[1..];
[00:46:20] + ...
[00:46:20] 7 LL |     }
[00:46:20] 8    |     - temporary value only lives until here
[00:46:20] 9 LL |     y.use_ref();
[00:46:20] 
[00:46:20] The actual stderr differed from the expected stderr.
[00:46:20] The actual stderr differed from the expected stderr.
[00:46:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/slice-borrow.nll/slice-borrow.nll.stderr
[00:46:20] To update references, rerun the tests and pass the `--bless` flag
[00:46:20] To only update this specific test, also pass `--test-args span/slice-borrow.rs`
[00:46:20] error: 1 errors occurred comparing output.
[00:46:20] status: exit code: 1
[00:46:20] status: exit code: 1
[00:46:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/slice-borrow.rs" "--target=x86_64-unvec ( box [ $ ( $ x ) , * ] ) ) ; ( $ ( $ x : expr , ) * )","highlight_start":1,"highlight_end":48}],"label":"temporary value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/span/slice-borrow.rs","byte_start":587,"byte_end":606,"line_start":16,"line_end":16,"column_start":28,"column_end":47,"is_primary":false,"text":[{"text":"        let x: &[isize] = &vec![1, 2, 3, 4, 5];","highlight_start":28,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"vec!","def_site_span":{"file_name":"<vec macros>","byte_start":0,"byte_end":222,"line_start":1,"line_end":4,"column_start":1,"column_end":31,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"< [ _ ] > :: into_vec ( box [ $ ( $ x ) , * ] ) ) ; ( $ ( $ x : expr , ) * )","highlight_start":1,"highlight_end":77},{"text":"=> ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/src/test/ui/span/slice-borrow.rs","byte_start":693,"byte_end":694,"line_start":19,"line_end":19,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"temporary value only lives until here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/slice-borrow.rs","byte_start":699,"byte_end":700,"line_start":20,"line_end":20,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    y.use_ref();","highlight_start":5,"highlight_end":6}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: borrowed value does not live long enough\n  --> /checkout/src/test/ui/span/slice-borrow.rs:16:28\n   |\nLL |         let x: &[isize] = &vec![1, 2, 3, 4, 5];\n   |                            ^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough\n...\nLL |     }\n   |     - temporary value only lives until here\nLL |     y.use_ref();\n   |     - borrow later used here\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:46:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:20] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] thread '[ui (nll)] ui/span/slice-borrow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:46:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:20] 
[00:46:20] 
[00:46:20] ut/obj/build/bootstrap/debug/bootstrap test
[00:46:20] Build completed unsuccessfully in 0:02:49
[00:46:20] Makefile:58: recipe for target 'check' failed
[00:46:20] make: *** [check] Error 1
122228 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
121720 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
107600 ./src/llvm/test/CodeGen
107460 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
---
travis_time:end:04da7c60:start=1532076598496412296,finish=1532076598505261524,duration=8849228
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b1e8353
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c146f55
travis_time:start:1c146f55
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0351912e
$ dmesg | grep -i kill
