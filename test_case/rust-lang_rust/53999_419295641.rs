plain
[00:50:32] ....................................................................................................
[00:50:36] ....................................................................................................
[00:50:39] ........................................i...........................................................
[00:50:41] ....................................................................................................
[00:50:44] .........................................................................................iiiiiiiii..
[00:50:50] ...........ii.......................................................................................
[00:50:54] ....................................................................................................
[00:50:57] ......................................................................i.............................
[00:51:00] ....................................................................................................
---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:44] 
[00:58:44] running 30 tests
/unnecessary-extern-crate.rs:38:1\n   |\nLL | crate extern crate libc as b;\n   | ^^^^^\n   |\n   = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable\n\n"}
[00:59:06] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:59:06] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:59:06] ------------------------------------------
[00:59:06] 
[00:59:06] thread '[ui] ui-fulldeps/unnecessary-extern-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:59:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:59:06] test result: FAILED. 29 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:59:06] 
[00:59:06] 
[00:59:06] 
[00:59:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
