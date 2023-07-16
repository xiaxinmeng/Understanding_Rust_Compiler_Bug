\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/ctfe/issue-37550.rs","byte_start":597,"byte_end":601,"line_start":19,"line_end":19,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    let x = || t;","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(const_let)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: statements in constant functions are unstable (see issue #48821)\n  --> /checkout/src/test/run-pass/ctfe/issue-37550.rs:19:13\n   |\nLL |     let x = || t;\n   |             ^^^^\n   |\n   = help: add #![feature(const_let)] to the crate attributes to enable\n\n"}
[01:00:10] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:00:10] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] thread '[run-pass] run-pass/ctfe/issue-37550.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:10] 
---
[01:00:10] test result: FAILED. 2874 passed; 2 failed; 10 ignored; 0 measured; 0 filtered out
[01:00:10] 
[01:00:10] 
[01:00:10] 
[01:00:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "statravis_time:end:05e0f16a:start=1543244855514386408,finish=1543248465863742996,duration=3610349356588
travis_time:start:028e9d18
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 26 16:07:45 UTC 2018
Mon, 26 Nov 2018 16:07:45 GMT
