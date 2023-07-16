\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32829-2.rs","byte_start":1858,"byte_end":1865,"line_start":84,"line_end":84,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        valid();","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(const_let)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: statements in statics are unstable (see issue #48821)\n  --> /checkout/src/test/ui/issues/issue-32829-2.rs:84:9\n   |\nLL |         valid();\n   |         ^^^^^^^\n   |\n   = help: add #![feature(const_let)] to the crate attributes to enable\n\n"}
[00:48:54] {"message":"aborting due to 12 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 12 previous errors\n\n"}
[00:48:54] {"message":"Some errors occurred: E0015, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0015, E0658.\n"}
[00:48:54] {"message":"For more information about an error, try `rustc --explain E0015`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0015`.\n"}
[00:48:54] ------------------------------------------
[00:48:54] 
[00:48:54] thread '[ui] ui/issues/issue-32829-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:48:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:54] 
[00:48:54] ---- [ui] ui/issues/issue-38875/issue_38875.rs stdout ----
[00:48:54] 
[00:48:54] error: aux-build `/checkout/src/test/ui/issues/issue-38875/auxiliary/issue-38875-b.rs` source not found
[00:48:54] thread '[ui] ui/issues/issue-38875/issue_38875.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2022:9
[00:48:54] ---- [ui] ui/issues/issue-41652/issue_41652.rs stdout ----
[00:48:54] 
[00:48:54] 
[00:48:54] error: aux-build `/checkout/src/test/ui/issues/issue-41652/auxiliary/issue-41652-b.rs` source not found
[00:48:54] thread '[ui] ui/issues/issue-41652/issue_41652.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2022:9
[00:48:54] ---- [ui] ui/issues/issue-43189.rs stdout ----
[00:48:54] 
[00:48:54] 
[00:48:54] error: aux-build `/checkout/src/test/ui/issues/auxiliary/xcrate-issue-43189-a.rs` source not found
[00:48:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:48:54] 
[00:48:54] ---- [ui] ui/issues/issue-45829/rename-extern-vs-use.rs stdout ----
[00:48:54] 
[00:48:54] 
[00:48:54] error: aux-build `/checkout/src/test/ui/issues/issue-45829/auxiliary/issue-45829-b.rs` source not found
[00:48:54] 
[00:48:54] ---- [ui] ui/issues/issue-45829/rename-extern-with-tab.rs stdout ----
[00:48:54] 
[00:48:54] 
[00:48:54] error: aux-build `/checkout/src/test/ui/issues/issue-45829/auxiliary/issue-45829-a.rs` source not found
[00:48:54] thread '[ui] ui/issues/issue-45829/rename-extern-with-tab.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2022:9
[00:48:54] ---- [ui] ui/issues/issue-45829/rename-extern.rs stdout ----
[00:48:54] 
[00:48:54] 
[00:48:54] error: aux-build `/checkout/src/test/ui/issues/issue-45829/auxiliary/issue-45829-a.rs` source not found
[00:48:54] 
[00:48:54] ---- [ui] ui/issues/issue-45829/rename-use-vs-extern.rs stdout ----
[00:48:54] 
[00:48:54] 
[00:48:54] error: aux-build `/checkout/src/test/ui/issues/issue-45829/auxiliary/issue-45829-b.rs` source not found
[00:48:54] 
[00:48:54] 
[00:48:54] failures:
[00:48:54]     [ui] ui/issues/issue-32829-2.rs
---
travis_time:end:1668dc0e:start=1541393518471625118,finish=1541393518476357247,duration=4732129
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16fd9920
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16b35cb2
travis_time:start:16b35cb2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0344b314
$ dmesg | grep -i kill
