plain
2020-02-10T15:23:41.5392145Z 
2020-02-10T15:23:41.5392707Z ---- [ui] ui/recursion/recursion.rs stdout ----
2020-02-10T15:23:41.5392948Z diff of stderr:
2020-02-10T15:23:41.5393071Z 
2020-02-10T15:23:41.5393629Z + error: overflow while checking whether `Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>` requires drop
2020-02-10T15:23:41.5394140Z + 
2020-02-10T15:23:41.5394519Z 1 error: reached the recursion limit while instantiating `test::<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
2020-02-10T15:23:41.5395492Z 3    |
2020-02-10T15:23:41.5395621Z 
2020-02-10T15:23:41.5395776Z 10 LL | | }
2020-02-10T15:23:41.5395907Z 11    | |_^
---
2020-02-10T15:23:41.5396958Z 
2020-02-10T15:23:41.5397087Z 
2020-02-10T15:23:41.5397226Z The actual stderr differed from the expected stderr.
2020-02-10T15:23:41.5397639Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion/recursion.stderr
2020-02-10T15:23:41.5398048Z To update references, rerun the tests and pass the `--bless` flag
2020-02-10T15:23:41.5398463Z To only update this specific test, also pass `--test-args recursion/recursion.rs`
2020-02-10T15:23:41.5398792Z error: 1 errors occurred comparing output.
2020-02-10T15:23:41.5398947Z status: exit code: 1
2020-02-10T15:23:41.5398947Z status: exit code: 1
2020-02-10T15:23:41.5399815Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion/auxiliary" "-A" "unused"
2020-02-10T15:23:41.5400454Z ------------------------------------------
2020-02-10T15:23:41.5400623Z 
2020-02-10T15:23:41.5400949Z ------------------------------------------
2020-02-10T15:23:41.5401149Z stderr:
2020-02-10T15:23:41.5401149Z stderr:
2020-02-10T15:23:41.5401455Z ------------------------------------------
2020-02-10T15:23:41.5401852Z error: overflow while checking whether `Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>` requires drop
2020-02-10T15:23:41.5402297Z 
2020-02-10T15:23:41.5402771Z error: reached the recursion limit while instantiating `test::<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Cons<Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
2020-02-10T15:23:41.5403557Z    |
2020-02-10T15:23:41.5403557Z    |
2020-02-10T15:23:41.5403977Z LL | / fn test<T:Dot> (n:isize, i:isize, first:T, second:T) ->isize { //~ ERROR recursion limit
2020-02-10T15:23:41.5404197Z LL | |   match n {    0 => {first.dot(second)}
2020-02-10T15:23:41.5404377Z LL | |       // FIXME(#4287) Error message should be here. It should be
2020-02-10T15:23:41.5404539Z LL | |       // a type error to instantiate `test` at a type other than T.
2020-02-10T15:23:41.5404937Z LL | |     _ => {test (n-1, i+1, Cons {head:2*i+1, tail:first}, Cons{head:i*i, tail:second})}
2020-02-10T15:23:41.5405279Z LL | | }
2020-02-10T15:23:41.5405427Z    | |_^
2020-02-10T15:23:41.5405538Z 
2020-02-10T15:23:41.5405688Z error: aborting due to 2 previous errors
---
2020-02-10T15:23:41.5433040Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-10T15:23:41.5444950Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-10T15:23:41.5445017Z 
2020-02-10T15:23:41.5445223Z 
2020-02-10T15:23:41.5447355Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-10T15:23:41.5448125Z 
2020-02-10T15:23:41.5448928Z 
2020-02-10T15:23:41.5461596Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-10T15:23:41.5462115Z Build completed unsuccessfully in 1:23:06
2020-02-10T15:23:41.5462115Z Build completed unsuccessfully in 1:23:06
2020-02-10T15:23:41.5507022Z == clock drift check ==
2020-02-10T15:23:42.2414675Z   local time: Mon Feb 10 15:23:41 UTC 2020
2020-02-10T15:23:42.2420738Z   network time: Mon, 10 Feb 2020 15:23:41 GMT
2020-02-10T15:23:42.2421097Z == end clock drift check ==
2020-02-10T15:23:42.2421271Z 
2020-02-10T15:23:42.2466788Z ##[error]Bash exited with code '1'.
2020-02-10T15:23:42.2511738Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-10T15:23:42.2513392Z ==============================================================================
2020-02-10T15:23:42.2513469Z Task         : Get sources
2020-02-10T15:23:42.2513556Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
