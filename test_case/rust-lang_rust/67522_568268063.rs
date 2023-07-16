plain
2019-12-22T13:17:10.4525960Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T13:17:10.4734359Z ##[command]git config gc.auto 0
2019-12-22T13:17:10.4821952Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T13:17:10.4876451Z ##[command]git config --get-all http.proxy
2019-12-22T13:17:10.5043404Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67522/merge:refs/remotes/pull/67522/merge
---
2019-12-22T14:17:17.6294482Z .................................................................................................... 1600/9427
2019-12-22T14:17:22.3490697Z .................................................................................................... 1700/9427
2019-12-22T14:17:32.8041271Z .....................................................................................i.............. 1800/9427
2019-12-22T14:17:40.3743626Z .................................................................................................... 1900/9427
2019-12-22T14:17:47.4651189Z ......................................................................iiiii......................... 2000/9427
2019-12-22T14:18:08.1511428Z .................................................................................................... 2200/9427
2019-12-22T14:18:10.6552686Z .................................................................................................... 2300/9427
2019-12-22T14:18:13.2458742Z .................................................................................................... 2400/9427
2019-12-22T14:18:26.8508419Z .................................................................................................... 2500/9427
---
2019-12-22T14:21:21.8757953Z .i...............i.................................................................................. 4900/9427
2019-12-22T14:21:32.4637698Z .................................................................................................... 5000/9427
2019-12-22T14:21:37.5550865Z .............................................i...................................................... 5100/9427
2019-12-22T14:21:47.8541478Z .................................................................................................... 5200/9427
2019-12-22T14:21:53.9215167Z ............ii.ii...........i....................................................................... 5300/9427
2019-12-22T14:22:03.2814909Z .................................................................................................... 5500/9427
2019-12-22T14:22:14.9221551Z ..............................................................................................i..... 5600/9427
2019-12-22T14:22:23.0020766Z .................................................................................................... 5700/9427
2019-12-22T14:22:28.2243656Z .................................................................................................... 5800/9427
2019-12-22T14:22:28.2243656Z .................................................................................................... 5800/9427
2019-12-22T14:22:37.9392398Z ..................................................................................ii...i..ii........ 5900/9427
2019-12-22T14:23:00.7767375Z .................................................................................................... 6100/9427
2019-12-22T14:23:08.9629385Z .................................................................................................... 6200/9427
2019-12-22T14:23:16.9552333Z .................................................................................................... 6300/9427
2019-12-22T14:23:16.9552333Z .................................................................................................... 6300/9427
2019-12-22T14:23:34.2164438Z .........i..ii...................................................................................... 6400/9427
2019-12-22T14:23:54.8993355Z .....................................................................................i.............. 6600/9427
2019-12-22T14:23:57.1305620Z .................................................................................................... 6700/9427
2019-12-22T14:23:59.3796005Z .....................................................................................i.............. 6800/9427
2019-12-22T14:24:02.1123886Z .................................................................................................... 6900/9427
---
2019-12-22T14:28:54.1880747Z diff of stderr:
2019-12-22T14:28:54.1880878Z 
2019-12-22T14:28:54.1881076Z 7    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1881238Z 8 
2019-12-22T14:28:54.1881662Z 9 error[E0004]: non-exhaustive patterns: type `&Void` is non-empty
2019-12-22T14:28:54.1882642Z +   --> $DIR/uninhabited-matches-feature-gated.rs:16:19
2019-12-22T14:28:54.1882836Z 11    |
2019-12-22T14:28:54.1883005Z 12 LL | enum Void {}
2019-12-22T14:28:54.1883005Z 12 LL | enum Void {}
2019-12-22T14:28:54.1883404Z 13    | ------------ `Void` defined here
2019-12-22T14:28:54.1883770Z 18    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1883923Z 19 
2019-12-22T14:28:54.1883923Z 19 
2019-12-22T14:28:54.1884345Z 20 error[E0004]: non-exhaustive patterns: type `(Void,)` is non-empty
2019-12-22T14:28:54.1885738Z +   --> $DIR/uninhabited-matches-feature-gated.rs:19:19
2019-12-22T14:28:54.1885948Z 22    |
2019-12-22T14:28:54.1886121Z 23 LL |     let _ = match x {};
2019-12-22T14:28:54.1886281Z 24    |                   ^
2019-12-22T14:28:54.1886281Z 24    |                   ^
2019-12-22T14:28:54.1886409Z 
2019-12-22T14:28:54.1886604Z 26    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1886753Z 27 
2019-12-22T14:28:54.1887184Z 28 error[E0004]: non-exhaustive patterns: type `[Void; 1]` is non-empty
2019-12-22T14:28:54.1888087Z +   --> $DIR/uninhabited-matches-feature-gated.rs:22:19
2019-12-22T14:28:54.1888294Z 30    |
2019-12-22T14:28:54.1888448Z 31 LL |     let _ = match x {};
2019-12-22T14:28:54.1888594Z 32    |                   ^
2019-12-22T14:28:54.1888594Z 32    |                   ^
2019-12-22T14:28:54.1888736Z 
2019-12-22T14:28:54.1888893Z 34    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1889278Z 35 
2019-12-22T14:28:54.1890188Z 36 error[E0004]: non-exhaustive patterns: `&[_, ..]` not covered
2019-12-22T14:28:54.1892094Z +   --> $DIR/uninhabited-matches-feature-gated.rs:25:19
2019-12-22T14:28:54.1892407Z 38    |
2019-12-22T14:28:54.1892555Z 39 LL |     let _ = match x {
2019-12-22T14:28:54.1892555Z 39 LL |     let _ = match x {
2019-12-22T14:28:54.1892739Z 40    |                   ^ pattern `&[_, ..]` not covered
2019-12-22T14:28:54.1893034Z 42    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1893176Z 43 
2019-12-22T14:28:54.1893651Z 44 error[E0004]: non-exhaustive patterns: `Err(_)` not covered
2019-12-22T14:28:54.1895932Z -   --> $DIR/uninhabited-matches-feature-gated.rs:32:19
---
2019-12-22T14:28:54.1901491Z 56    |         ^^^^^ pattern `Err(_)` not covered
2019-12-22T14:28:54.1901523Z 
2019-12-22T14:28:54.1901548Z 
2019-12-22T14:28:54.1901591Z The actual stderr differed from the expected stderr.
2019-12-22T14:28:54.1902055Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/uninhabited-matches-feature-gated.stderr
2019-12-22T14:28:54.1902324Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T14:28:54.1902643Z To only update this specific test, also pass `--test-args uninhabited/uninhabited-matches-feature-gated.rs`
2019-12-22T14:28:54.1902723Z error: 1 errors occurred comparing output.
2019-12-22T14:28:54.1902766Z status: exit code: 1
2019-12-22T14:28:54.1902766Z status: exit code: 1
2019-12-22T14:28:54.1903690Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uninhabited/uninhabited-matches-feature-gated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/auxiliary" "-A" "unused"
2019-12-22T14:28:54.1904043Z ------------------------------------------
2019-12-22T14:28:54.1904077Z 
2019-12-22T14:28:54.1904295Z ------------------------------------------
2019-12-22T14:28:54.1904356Z stderr:
---
2019-12-22T14:28:54.1905442Z    |                   ^ pattern `Err(_)` not covered
2019-12-22T14:28:54.1905484Z    |
2019-12-22T14:28:54.1905535Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1905587Z 
2019-12-22T14:28:54.1905837Z error[E0004]: non-exhaustive patterns: type `&Void` is non-empty
2019-12-22T14:28:54.1906206Z    |
2019-12-22T14:28:54.1906249Z LL | enum Void {}
2019-12-22T14:28:54.1906249Z LL | enum Void {}
2019-12-22T14:28:54.1906481Z    | ------------ `Void` defined here
2019-12-22T14:28:54.1906527Z ...
2019-12-22T14:28:54.1907082Z LL |     let _ = match x {}; //~ ERROR non-exhaustive
2019-12-22T14:28:54.1907186Z    |
2019-12-22T14:28:54.1907261Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1907298Z 
2019-12-22T14:28:54.1907298Z 
2019-12-22T14:28:54.1907622Z error[E0004]: non-exhaustive patterns: type `(Void,)` is non-empty
2019-12-22T14:28:54.1907975Z    |
2019-12-22T14:28:54.1907975Z    |
2019-12-22T14:28:54.1908222Z LL |     let _ = match x {}; //~ ERROR non-exhaustive
2019-12-22T14:28:54.1908553Z    |
2019-12-22T14:28:54.1908607Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1908642Z 
2019-12-22T14:28:54.1908642Z 
2019-12-22T14:28:54.1908972Z error[E0004]: non-exhaustive patterns: type `[Void; 1]` is non-empty
2019-12-22T14:28:54.1909615Z    |
2019-12-22T14:28:54.1909615Z    |
2019-12-22T14:28:54.1910498Z LL |     let _ = match x {}; //~ ERROR non-exhaustive
2019-12-22T14:28:54.1910987Z    |
2019-12-22T14:28:54.1911038Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1911092Z 
2019-12-22T14:28:54.1911092Z 
2019-12-22T14:28:54.1911407Z error[E0004]: non-exhaustive patterns: `&[_, ..]` not covered
2019-12-22T14:28:54.1911760Z    |
2019-12-22T14:28:54.1911989Z LL |     let _ = match x {   //~ ERROR non-exhaustive
2019-12-22T14:28:54.1911989Z LL |     let _ = match x {   //~ ERROR non-exhaustive
2019-12-22T14:28:54.1912039Z    |                   ^ pattern `&[_, ..]` not covered
2019-12-22T14:28:54.1912148Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:28:54.1912183Z 
2019-12-22T14:28:54.1912416Z error[E0004]: non-exhaustive patterns: `Err(_)` not covered
2019-12-22T14:28:54.1912696Z   --> /checkout/src/test/ui/uninhabited/uninhabited-matches-feature-gated.rs:33:19
---
2019-12-22T14:28:54.1913543Z    |
2019-12-22T14:28:54.1913599Z LL |     let Ok(x) = x;
2019-12-22T14:28:54.1913643Z    |         ^^^^^ pattern `Err(_)` not covered
2019-12-22T14:28:54.1913684Z    |
2019-12-22T14:28:54.1913748Z    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
2019-12-22T14:28:54.1915118Z help: you might want to use `if let` to ignore the variant that isn't matched
2019-12-22T14:28:54.1915197Z    |
2019-12-22T14:28:54.1915240Z LL |     if let Ok(x) = x { /* */ }
2019-12-22T14:28:54.1915281Z    |
---
2019-12-22T14:28:54.1918960Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-22T14:28:54.1919380Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-22T14:28:54.1919424Z 
2019-12-22T14:28:54.1919449Z 
2019-12-22T14:28:54.1927566Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-22T14:28:54.1928354Z 
2019-12-22T14:28:54.1928402Z 
2019-12-22T14:28:54.1976981Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-22T14:28:54.1977066Z Build completed unsuccessfully in 1:05:47
2019-12-22T14:28:54.1977066Z Build completed unsuccessfully in 1:05:47
2019-12-22T14:28:54.2028028Z == clock drift check ==
2019-12-22T14:28:54.2048071Z   local time: Sun Dec 22 14:28:54 UTC 2019
2019-12-22T14:28:54.7530183Z   network time: Sun, 22 Dec 2019 14:28:54 GMT
2019-12-22T14:28:54.7533610Z == end clock drift check ==
2019-12-22T14:28:55.5808340Z 
2019-12-22T14:28:55.5958854Z ##[error]Bash exited with code '1'.
2019-12-22T14:28:55.6002134Z ##[section]Starting: Checkout
2019-12-22T14:28:55.6003883Z ==============================================================================
2019-12-22T14:28:55.6003953Z Task         : Get sources
2019-12-22T14:28:55.6004000Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
