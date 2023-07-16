\n\nwhere `P1, ..., Pm` are the type parameters of the `impl` and `T0, ..., Tn`\nare types. One of the types `T0, ..., Tn` must be a local type (this is another\norphan rule, see the explanation for E0117). Let `i` be the smallest integer\nsuch that `Ti` is a local type. Then no type parameter can appear in any of the\n`Tj` for `j < i`.\n\nFor information on the design of the orphan rules, see [RFC 1023].\n\n[RFC 1023]: https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/e0119/issue-28981.rs","byte_start":502,"byte_end":525,"line_start":15,"line_end":15,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"impl<Foo> Deref for Foo { } //~ ERROR must be used","highlight_start":1,"highlight_end":24}],"label":"type parameter `Foo` must be used as the type parameter for some local type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"only traits defined in the current crate can be implemented for a type parameter","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0210]: type parameter `Foo` must be used as the type parameter for some local type (e.g. `MyStruct<Foo>`)\n  --> /checkout/src/test/ui/e0119/issue-28981.rs:15:1\n   |\nLL | impl<Foo> Deref for Foo { } //~ ERROR must be used\n   | ^^^^^^^^^^^^^^^^^^^^^^^ type parameter `Foo` must be used as the type parameter for some local type\n   |\n   = note: only traits defined in the current crate can be implemented for a type parameter\n\n"}
[00:54:33] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:54:33] {"message":"Some errors occurred: E0119, E0210.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0119, E0210.\n"}
[00:54:33] {"message":"For more information about an error, try `rustc --explain E0119`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0119`.\n"}
[00:54:33] ------------------------------------------
[00:54:33] 
[00:54:33] thread '[ui] ui/e0119/issue-28981.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:54:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:54:33] 
[00:54:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:54:33] 
[00:54:33] 
[00:54:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:33] 
[00:54:33] 
[00:54:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:33] Build completed unsuccessfully in 0:02:55
[00:54:33] Build completed unsuccessfully in 0:02:55
[00:54:33] make: *** [check] Error 1
[00:54:33] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1680d2d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
