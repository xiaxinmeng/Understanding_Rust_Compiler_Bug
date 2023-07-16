plain
[00:49:28] .............................................................................i...................... 4200/4593
[00:49:31] .................................................................................................... 4300/4593
[00:49:35] ..................................................................................................F. 4400/4593
[00:49:37] .................................................................................................... 4500/4593
 "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore_const_names/auxiliary" "-A" "unused"
[00:49:40] ------------------------------------------
[00:49:40] 
[00:49:40] ------------------------------------------
[00:49:40] stderr:
---
[00:49:40] 
[00:49:40] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:49:40] 
[00:49:40] 
[00:49:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "-

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00dcbd40
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
