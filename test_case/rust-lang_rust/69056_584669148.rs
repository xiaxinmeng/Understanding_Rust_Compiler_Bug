plain
2020-02-11T14:43:37.2234905Z 
2020-02-11T14:43:37.2235869Z ---- [ui] ui/recursion/recursion.rs stdout ----
2020-02-11T14:43:37.2237514Z diff of stderr:
2020-02-11T14:43:37.2237702Z 
2020-02-11T14:43:37.2239755Z - error: overflow while checking whether `Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>` requires drop
2020-02-11T14:43:37.2240644Z - 
2020-02-11T14:43:37.2241001Z 3 error: reached the recursion limit while instantiating `test::<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
2020-02-11T14:43:37.2241629Z 5    |
2020-02-11T14:43:37.2241674Z 
2020-02-11T14:43:37.2241754Z 12 LL | | }
2020-02-11T14:43:37.2241817Z 13    | |_^
---
2020-02-11T14:43:37.2242567Z 
2020-02-11T14:43:37.2242621Z 
2020-02-11T14:43:37.2242691Z The actual stderr differed from the expected stderr.
2020-02-11T14:43:37.2243109Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion/recursion.stderr
2020-02-11T14:43:37.2243417Z To update references, rerun the tests and pass the `--bless` flag
2020-02-11T14:43:37.2243751Z To only update this specific test, also pass `--test-args recursion/recursion.rs`
2020-02-11T14:43:37.2243894Z error: 1 errors occurred comparing output.
2020-02-11T14:43:37.2243974Z status: exit code: 1
2020-02-11T14:43:37.2243974Z status: exit code: 1
2020-02-11T14:43:37.2244894Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/recursion.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion/auxiliary" "-A" "unused"
2020-02-11T14:43:37.2245385Z ------------------------------------------
2020-02-11T14:43:37.2245452Z 
2020-02-11T14:43:37.2245683Z ------------------------------------------
2020-02-11T14:43:37.2245782Z stderr:
2020-02-11T14:43:37.2245782Z stderr:
2020-02-11T14:43:37.2246014Z ------------------------------------------
2020-02-11T14:43:37.2246375Z error: reached the recursion limit while instantiating `test::<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
2020-02-11T14:43:37.2247396Z    |
2020-02-11T14:43:37.2247396Z    |
2020-02-11T14:43:37.2247878Z LL | / fn test<T:Dot> (n:isize, i:isize, first:T, second:T) ->isize { //~ ERROR recursion limit
2020-02-11T14:43:37.2248003Z LL | |   match n {    0 => {first.dot(second)}
2020-02-11T14:43:37.2249400Z LL | |       // FIXME(#4287) Error message should be here. It should be
2020-02-11T14:43:37.2249507Z LL | |       // a type error to instantiate `test` at a type other than T.
2020-02-11T14:43:37.2249947Z LL | |     _ => {test (n-1, i+1, Cons {head:2*i+1, tail:first}, Cons{head:i*i, tail:second})}
2020-02-11T14:43:37.2250121Z LL | | }
2020-02-11T14:43:37.2250182Z    | |_^
2020-02-11T14:43:37.2250239Z 
2020-02-11T14:43:37.2250303Z error: aborting due to previous error
---
2020-02-11T14:43:37.2277659Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-11T14:43:37.2277999Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-11T14:43:37.2285229Z 
2020-02-11T14:43:37.2285379Z 
2020-02-11T14:43:37.2287723Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-11T14:43:37.2288433Z 
2020-02-11T14:43:37.2288474Z 
2020-02-11T14:43:37.2298678Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-02-11T14:43:37.2299034Z Build completed unsuccessfully in 1:25:07
2020-02-11T14:43:37.2299034Z Build completed unsuccessfully in 1:25:07
2020-02-11T14:43:37.2369278Z == clock drift check ==
2020-02-11T14:43:37.2396504Z   local time: Tue Feb 11 14:43:37 UTC 2020
2020-02-11T14:43:37.5388549Z   network time: Tue, 11 Feb 2020 14:43:37 GMT
2020-02-11T14:43:37.5395784Z == end clock drift check ==
2020-02-11T14:43:38.3650603Z 
2020-02-11T14:43:38.3726295Z ##[error]Bash exited with code '1'.
2020-02-11T14:43:38.3764629Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-11T14:43:38.3766415Z ==============================================================================
2020-02-11T14:43:38.3766494Z Task         : Get sources
2020-02-11T14:43:38.3766583Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
