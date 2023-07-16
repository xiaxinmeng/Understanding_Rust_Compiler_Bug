\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-static-bound.rs","byte_start":1042,"byte_end":1064,"line_start":26,"line_end":26,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    static_id_indirect(&v); //[ll]~ ERROR explicit lifetime required ithread '[ui] ui/regions/regions-static-bound.rs#nll' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] failures:
[00:48:48]     [ui] ui/nll/closure-requirements/escape-argument-callee.rs
[00:48:48]     [ui] ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs
---
[00:48:48] 
[00:48:48] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:48:48] 
[00:48:48] 
[00:48:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "uitravis_time:end:103e95d0:start=1543872235140474500,finish=1543875164088868595,duration=2928948394095
travis_time:start:188ec392
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 22:12:44 UTC 2018
Mon, 03 Dec 2018 22:12:44 GMT
