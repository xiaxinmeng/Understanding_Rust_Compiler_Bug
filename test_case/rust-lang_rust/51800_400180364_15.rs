\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/no-args-non-move-async-closure.rs"edition-keywords-2018-2015-parsing.rs
[00:47:46]     [ui] ui/edition-keywords-2018-2018-parsing.rs
[00:47:46]     [ui] ui/feature-gate-async-await.rs
[00:47:46]     [ui] ui/lint-anon-param-edition.rs
[00:47:46]     [ui] ui/no-args-non-move-async-closure.rs
[00:47:46]     [ui] ui/no-args-non-move-async-closure.rs
[00:47:46]     [ui] ui/rust-2018/extern-crate-idiomatic-in-2018.rs
[00:47:46]     [ui] ui/suggestions/removing-extern-crate.rs
[00:47:46] 
[00:47:46] test result: FAILED. 1508 passed; 9 failed; 5 ignored; 0 measured; 0 filtered out
[00:47:46] 
[00:47:46] 
[00:47:46] 
[00:47:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--3321272 .
2430452 ./obj/build
1826584 ./obj/build/x86_64-unknown-linux-gnu
729204 ./src
549324 ./obj/build/x86_64-unknown-linux-gnu/stage0
---
143820 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
143816 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
138748 ./obj/build/bootstrap/debug/incremental
124176 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
124172 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f2c2ciz8yh-9afue1-1ndug1mpblsoe
107672 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
103608 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rus
