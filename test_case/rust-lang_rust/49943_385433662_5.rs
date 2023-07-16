\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/ref-escapes-but-not-over-yield.rs","byte_start":882,"byte_end":884,"line_start":24,"line_end":24,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        a = &b;","highlight_start":13,"highlight_end":15}],"label":"borrowed value does not live long enough","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/ref-escapes-but-not-over-yield.rs","byte_start":939,"byte_end":940,"line_start":26,"line_end":26,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    };","highlight_start":5,"highlight_end":6}],"label":"borrowed value only lives until here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/ref-escapes-but-not-over-yield.rs","byte_start":824,"byte_end":940,"line_start":21,"line_end":26,"column_start":17,"column_end":6,"is_primary":false,"text":[{"text":"    let mut b = move |ror: aborting due to previous error\n\n"}
[00:47:47] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui (nll)] ui/generator/ref-escapes-but-not-over-yield.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:47:47] 
---
[00:47:47] test result: FAILED. 1370 passed; 3 failed; 7 ignored; 0 measured; 0 filtered out
[00:47:47] 
[00:47:47] 
[00:47:47] 
[00:47:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath4G  4.0K  7.4G   1% /dev
/dev/sda1        30G  9.8G   19G  35% /
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
---
60840 ./src/llvm-emscripten/lib
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53752 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-284je5yviillk
53748 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-284je5yviillk/s-f0lq29bbi9-erx9ij-yefbrhq8bkke
48604 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47832 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
46868 ./src/test
46820 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
