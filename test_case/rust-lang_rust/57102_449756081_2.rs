\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-45157.rs","byte_start":817,"byte_end":823,"line_start":37,"line_end":37,"column_start":20,"column_end":26,"is_primary":true,"text":[{"text":"        let nref = &u.z.c;","highlight_start":20,"highlight_end":26}],"label":"immutable borrow occurs here (via `u.z.c`)","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-45157.rs","byte_start":765,"byte_end":775,"line_start":34,"line_end":34,"column_start":20,"column_end":30,"is_primary":false,"text":[{"text":"        let mref = &mut u.s.a;","highlight_start":20,"highlight_end":30}],"label":"mutable borrow occurs here (via `u.s.a`)","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-45157.rs","byte_start":952,"byte_end":956,"line_start":39,"line_end":39,"column_start":27,"column_end":31,"is_primary":false,"text":[{"text":"        println!(\"{} {}\", mref, nref)","highlight_start":27,"highlight_end":31}],"label":"mutable borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `u.z` (via `u.z.c`) as immutable because it is also borrowed as mutable (via `u.s.a`)\n  --> /checkout/src/test/ui/issues/issue-45157.rs:37:20\n   |\nLL |         let mref = &mut u.s.a;\n   |                    ---------- mutable borrow occurs here (via `u.s.a`)\n...\nLL |         let nref = &u.z.c;\n   |                    ^^^^^^ immutable borrow occurs here (via `u.z.c`)\nLL |         //~^ ERROR cannot borrow `u.z.c` as immutable because it is also borrowed as mutable [E0502]\nLL |         println!(\"{} {}\", mref, nref)\n   |                           ---- mutable borrow later used here\n\n"}
[00:56:05] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:05] {"message":"For more information about this error, try `rustc --explain E0502`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0502`.\n"}
[00:56:05] ------------------------------------------
[00:56:05] 
[00:56:05] thread '[ui] ui/issues/issue-45157.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:05] 
[00:56:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:505:22
[00:56:05] 
[00:56:05] 
[00:56:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/3750028 .
2540248 ./obj/build
1880440 ./obj/build/x86_64-unknown-linux-gnu
1119612 ./src
509768 ./obj/build/x86_64-unknown-linux-gnu/stage0
---
150160 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
150156 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
147764 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
144216 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144212 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7w662zkqk-b8sm1g-c95hburtu2sh
124972 ./obj/build-unknown-linux-gnu/release
57508 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
57504 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
56896 ./src/llvm/test/MC
---
24092 ./src/tools/lldb/packages/Python/lldbsuite
23856 ./src/tools/lldb/packages/Python/lldbsuite/test
23704 ./src/llvm-emscripten/test/tools
23524 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
23520 ./obrashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0053a728
travis_time:start:0053a728
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:025a455f
$ dmesg | grep -i kill
