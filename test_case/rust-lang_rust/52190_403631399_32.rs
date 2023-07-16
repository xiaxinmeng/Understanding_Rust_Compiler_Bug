\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/first-edition/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/move-errors.rs","byte_start":607,"byte_end":609,"line_start":19,"line_end":19,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"    let b = *a;","highlight_start":13,"highlight_end":15}],"label":"cannot move out of borrowed content","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing this dereference operator","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/nll/move-errors.rs","byte_start":607,"byte_end":609,"line_start":19,"line_end":19,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"    let b = *a;","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"a","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/nll/move-errors.rs:19:13\n   |\nLL |     let b = *a;\n   |             ^^\n   |             |\n   |             cannot move out of borrowed content\n   |             help: consider removing this dereference operator: `a`\n\n"}
[00:39:51] thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/librustc_data_structures/indexed_vec.rs:505:32
[00:39:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:51] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:39:51] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[00:39:51] error: internal compiler error: unexpected panic
[00:39:51] 
[00:39:51] 
[00:39:51] note: the compiler unexpectedly panicked. this is a bug.
[00:39:51] 
[00:39:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:39:51] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:39:51] 
[00:39:51] 
[00:39:51] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:39:51] 
[00:39:51] ------------------------------------------
[00:39:51] 
[00:39:51] thread '[ui] ui/nll/move-errors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:39:51] thread '[ui] ui/nll/move-errors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:39:51] 
[00:39:51] ---- [ui] ui/nll/return-ref-mut-issue-46557.rs stdout ----
[00:39:51] diff of stderr:
[00:39:51] 
[00:39:51] - error[E0597]: borrowed value does not live long enough
[00:39:51] -   --> $DIR/return-ref-mut-issue-46557.rs:17:21
[00:39:51] -    |
[00:39:51] - LL |   fn gimme_static_mut() -> &'static mut u32 {
[00:39:51] -    |  ___________________________________________-
[00:39:51] - LL | |     let ref mut x = 1234543; //~ ERROR borrowed value does not live long enough [E0597]
[00:39:51] -    | |                     ^^^^^^^ temporary value does not live long enough
[00:39:51] - LL | |     x
[00:39:51] - LL | | }
[00:39:51] -    | | -
[00:39:51] -    | | |
[00:39:51] -    | |_temporary value only lives until here
[00:39:51] -    |   borrow later used here
[00:39:51] - error: aborting due to previous error
[00:39:51] - 
[00:39:51] - For more information about this error, try `rustc --explain E0597`.
[00:39:51] - 
[00:39:51] - 
[00:39:51] 
[00:39:51] 
[00:39:51] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/return-ref-mut-issue-46557/return-ref-mut-issue-46557.stderr`: No such file or directory (os error 2)
[00:39:51] 
[00:39:51] ---- [ui] ui/nll/ty-outlives/projection-no-regions-closure.rs stdout ----
[00:39:51] diff of stderr:
[00:39:51] 
[00:39:51] 
---
[00:39:51]     [ui] ui/nll/return-ref-mut-issue-46557.rs
[00:39:51]     [ui] ui/nll/ty-outlives/projection-implied-bounds.rs
[00:39:51]     [ui] ui/nll/ty-outlives/projection-no-regions-closure.rs
[00:39:51]     [ui] ui/nll/ty-outlives/projection-no-regions-fn.rs
[00:39:51]     [ui] ui/nll/ty-outlives/projection-ones  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:39:51] 
[00:39:51] 
[00:39:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:39:51] Build completed unsuccessfully in 0:01:51
[00:39:51] Build completed unsuccessfully in 0:01:51
[00:39:51] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0187a000
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:119cf17b:start=1531172934904931201,finish=1531172934911436046,duration=6504845
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13992d38
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b7ebdb8
$ dmesg | grep -i kill
