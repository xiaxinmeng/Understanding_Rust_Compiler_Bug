plain
2019-09-27T05:58:40.2231833Z 7 note: lint level defined here
2019-09-27T05:58:40.2232399Z -   --> $DIR/lint-exceeding-bitshifts.rs:1:9
2019-09-27T05:58:40.2232786Z +   --> $DIR/lint-exceeding-bitshifts.rs:1:30
2019-09-27T05:58:40.2232883Z 9    |
2019-09-27T05:58:40.2232984Z 10 LL | #![deny(exceeding_bitshifts, const_err)]
2019-09-27T05:58:40.2233475Z +    |                              ^^^^^^^^^
2019-09-27T05:58:40.2233560Z 12 
2019-09-27T05:58:40.2233560Z 12 
2019-09-27T05:58:40.2233670Z 13 error: attempt to shift left with overflow
2019-09-27T05:58:40.2234113Z 
2019-09-27T05:58:40.2234193Z 101    |               ^^^^^^^^^^
2019-09-27T05:58:40.2234299Z 102 
2019-09-27T05:58:40.2234299Z 102 
2019-09-27T05:58:40.2234383Z 103 error: attempt to shift left with overflow
2019-09-27T05:58:40.2235131Z -    |
2019-09-27T05:58:40.2235131Z -    |
2019-09-27T05:58:40.2235460Z - LL |       let n = n << 8;
2019-09-27T05:58:40.2236116Z - 
2019-09-27T05:58:40.2236116Z - 
2019-09-27T05:58:40.2236482Z - error: attempt to shift left with overflow
2019-09-27T05:58:40.2236973Z 111    |
2019-09-27T05:58:40.2236973Z 111    |
2019-09-27T05:58:40.2237304Z 112 LL |       let n = 1u8 << -8;
2019-09-27T05:58:40.2238026Z 113    |               ^^^^^^^^^
2019-09-27T05:58:40.2238173Z 114 
2019-09-27T05:58:40.2238607Z - error: aborting due to 18 previous errors
2019-09-27T05:58:40.2238746Z + error: aborting due to 17 previous errors
2019-09-27T05:58:40.2238746Z + error: aborting due to 17 previous errors
2019-09-27T05:58:40.2238831Z 116 
2019-09-27T05:58:40.2238922Z 117 
2019-09-27T05:58:40.2238968Z 
2019-09-27T05:58:40.2239026Z 
2019-09-27T05:58:40.2239109Z The actual stderr differed from the expected stderr.
2019-09-27T05:58:40.2239613Z Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts/lint-exceeding-bitshifts.stderr
2019-09-27T05:58:40.2240025Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T05:58:40.2240468Z To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`
2019-09-27T05:58:40.2242140Z error: 1 errors occurred comparing output.
2019-09-27T05:58:40.2242241Z status: exit code: 1
2019-09-27T05:58:40.2242241Z status: exit code: 1
2019-09-27T05:58:40.2243468Z command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts/auxiliary" "-A" "unused"
2019-09-27T05:58:40.2244176Z ------------------------------------------
2019-09-27T05:58:40.2244245Z 
2019-09-27T05:58:40.2244589Z ------------------------------------------
2019-09-27T05:58:40.2244682Z stderr:
2019-09-27T05:58:40.2244682Z stderr:
2019-09-27T05:58:40.2245014Z ------------------------------------------
2019-09-27T05:58:40.2245111Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2245610Z    |
2019-09-27T05:58:40.2245610Z    |
2019-09-27T05:58:40.2245724Z LL |       let n = 1u8 << 8;   //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2246217Z    |
2019-09-27T05:58:40.2246313Z note: lint level defined here
2019-09-27T05:58:40.2246733Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:1:30
2019-09-27T05:58:40.2246860Z    |
2019-09-27T05:58:40.2246860Z    |
2019-09-27T05:58:40.2246940Z LL | #![deny(exceeding_bitshifts, const_err)]
2019-09-27T05:58:40.2247109Z 
2019-09-27T05:58:40.2247109Z 
2019-09-27T05:58:40.2247201Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2247689Z    |
2019-09-27T05:58:40.2247689Z    |
2019-09-27T05:58:40.2248038Z LL |       let n = 1u16 << 16; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2248243Z 
2019-09-27T05:58:40.2248243Z 
2019-09-27T05:58:40.2248336Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2248931Z    |
2019-09-27T05:58:40.2248931Z    |
2019-09-27T05:58:40.2249021Z LL |       let n = 1u32 << 32; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2249190Z 
2019-09-27T05:58:40.2249190Z 
2019-09-27T05:58:40.2249283Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2249789Z    |
2019-09-27T05:58:40.2249789Z    |
2019-09-27T05:58:40.2249901Z LL |       let n = 1u64 << 64; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2250054Z 
2019-09-27T05:58:40.2250054Z 
2019-09-27T05:58:40.2250148Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2250631Z    |
2019-09-27T05:58:40.2250631Z    |
2019-09-27T05:58:40.2250737Z LL |       let n = 1i8 << 8;   //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2251115Z 
2019-09-27T05:58:40.2251115Z 
2019-09-27T05:58:40.2251202Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2251740Z    |
2019-09-27T05:58:40.2251740Z    |
2019-09-27T05:58:40.2251849Z LL |       let n = 1i16 << 16; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2252018Z 
2019-09-27T05:58:40.2252018Z 
2019-09-27T05:58:40.2252093Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2252577Z    |
2019-09-27T05:58:40.2252577Z    |
2019-09-27T05:58:40.2252688Z LL |       let n = 1i32 << 32; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2252856Z 
2019-09-27T05:58:40.2252856Z 
2019-09-27T05:58:40.2252931Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2253434Z    |
2019-09-27T05:58:40.2253434Z    |
2019-09-27T05:58:40.2253557Z LL |       let n = 1i64 << 64; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2253727Z 
2019-09-27T05:58:40.2253727Z 
2019-09-27T05:58:40.2253929Z error: attempt to shift right with overflow
2019-09-27T05:58:40.2254444Z    |
2019-09-27T05:58:40.2254444Z    |
2019-09-27T05:58:40.2254534Z LL |       let n = 1u8 >> 8;   //~ ERROR: attempt to shift right with overflow
2019-09-27T05:58:40.2254705Z 
2019-09-27T05:58:40.2254705Z 
2019-09-27T05:58:40.2254801Z error: attempt to shift right with overflow
2019-09-27T05:58:40.2255284Z    |
2019-09-27T05:58:40.2255284Z    |
2019-09-27T05:58:40.2255390Z LL |       let n = 1u16 >> 16; //~ ERROR: attempt to shift right with overflow
2019-09-27T05:58:40.2255559Z 
2019-09-27T05:58:40.2255559Z 
2019-09-27T05:58:40.2255653Z error: attempt to shift right with overflow
2019-09-27T05:58:40.2256319Z    |
2019-09-27T05:58:40.2256319Z    |
2019-09-27T05:58:40.2256429Z LL |       let n = 1u32 >> 32; //~ ERROR: attempt to shift right with overflow
2019-09-27T05:58:40.2256598Z 
2019-09-27T05:58:40.2256598Z 
2019-09-27T05:58:40.2256676Z error: attempt to shift right with overflow
2019-09-27T05:58:40.2257204Z    |
2019-09-27T05:58:40.2257204Z    |
2019-09-27T05:58:40.2257314Z LL |       let n = 1u64 >> 64; //~ ERROR: attempt to shift right with overflow
2019-09-27T05:58:40.2257483Z 
2019-09-27T05:58:40.2257483Z 
2019-09-27T05:58:40.2257560Z error: attempt to shift right with overflow
2019-09-27T05:58:40.2258386Z    |
2019-09-27T05:58:40.2258386Z    |
2019-09-27T05:58:40.2258505Z LL |       let n = 1i8 >> 8;   //~ ERROR: attempt to shift right with overflow
2019-09-27T05:58:40.2258675Z 
2019-09-27T05:58:40.2258675Z 
2019-09-27T05:58:40.2258753Z error: attempt to shift right with overflow
2019-09-27T05:58:40.2259250Z    |
2019-09-27T05:58:40.2259250Z    |
2019-09-27T05:58:40.2259361Z LL |       let n = 1i16 >> 16; //~ ERROR: attempt to shift right with overflow
2019-09-27T05:58:40.2259532Z 
2019-09-27T05:58:40.2259532Z 
2019-09-27T05:58:40.2259609Z error: attempt to shift right with overflow
2019-09-27T05:58:40.2260093Z    |
2019-09-27T05:58:40.2260093Z    |
2019-09-27T05:58:40.2260204Z LL |       let n = 1i32 >> 32; //~ ERROR: attempt to shift right with overflow
2019-09-27T05:58:40.2260566Z 
2019-09-27T05:58:40.2260566Z 
2019-09-27T05:58:40.2260644Z error: attempt to shift right with overflow
2019-09-27T05:58:40.2261214Z    |
2019-09-27T05:58:40.2261214Z    |
2019-09-27T05:58:40.2261305Z LL |       let n = 1i64 >> 64; //~ ERROR: attempt to shift right with overflow
2019-09-27T05:58:40.2261476Z 
2019-09-27T05:58:40.2261476Z 
2019-09-27T05:58:40.2261568Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2262057Z    |
2019-09-27T05:58:40.2262057Z    |
2019-09-27T05:58:40.2262421Z LL |       let n = 1u8 << -8; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2262602Z 
2019-09-27T05:58:40.2262694Z error: aborting due to 17 previous errors
2019-09-27T05:58:40.2262750Z 
2019-09-27T05:58:40.2262792Z 
---
2019-09-27T05:58:40.2264062Z 7 note: lint level defined here
2019-09-27T05:58:40.2264412Z -   --> $DIR/lint-exceeding-bitshifts2.rs:1:9
2019-09-27T05:58:40.2264780Z +   --> $DIR/lint-exceeding-bitshifts2.rs:1:30
2019-09-27T05:58:40.2264874Z 9    |
2019-09-27T05:58:40.2264972Z 10 LL | #![deny(exceeding_bitshifts, const_err)]
2019-09-27T05:58:40.2265414Z +    |                              ^^^^^^^^^
2019-09-27T05:58:40.2265499Z 12 
2019-09-27T05:58:40.2265499Z 12 
2019-09-27T05:58:40.2265596Z 13 error: attempt to shift left with overflow
2019-09-27T05:58:40.2266030Z 
2019-09-27T05:58:40.2266084Z 
2019-09-27T05:58:40.2266186Z The actual stderr differed from the expected stderr.
2019-09-27T05:58:40.2266793Z Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts2/lint-exceeding-bitshifts2.stderr
2019-09-27T05:58:40.2266793Z Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts2/lint-exceeding-bitshifts2.stderr
2019-09-27T05:58:40.2267287Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T05:58:40.2267719Z To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts2.rs`
2019-09-27T05:58:40.2268279Z error: 1 errors occurred comparing output.
2019-09-27T05:58:40.2268390Z status: exit code: 1
2019-09-27T05:58:40.2268390Z status: exit code: 1
2019-09-27T05:58:40.2270841Z command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts2.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts2" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts2/auxiliary" "-A" "unused"
2019-09-27T05:58:40.2271548Z ------------------------------------------
2019-09-27T05:58:40.2271618Z 
2019-09-27T05:58:40.2271967Z ------------------------------------------
2019-09-27T05:58:40.2272082Z stderr:
2019-09-27T05:58:40.2272082Z stderr:
2019-09-27T05:58:40.2272398Z ------------------------------------------
2019-09-27T05:58:40.2272514Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2273002Z    |
2019-09-27T05:58:40.2273002Z    |
2019-09-27T05:58:40.2273097Z LL |       let n = 1u8 << (4+4); //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2273315Z    |
2019-09-27T05:58:40.2273391Z note: lint level defined here
2019-09-27T05:58:40.2273777Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts2.rs:1:30
2019-09-27T05:58:40.2274103Z    |
2019-09-27T05:58:40.2274103Z    |
2019-09-27T05:58:40.2274214Z LL | #![deny(exceeding_bitshifts, const_err)]
2019-09-27T05:58:40.2274382Z 
2019-09-27T05:58:40.2274382Z 
2019-09-27T05:58:40.2274458Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2275067Z    |
2019-09-27T05:58:40.2275067Z    |
2019-09-27T05:58:40.2275178Z LL |       let n = 1_isize << BITS; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2275354Z 
2019-09-27T05:58:40.2275354Z 
2019-09-27T05:58:40.2275429Z error: attempt to shift left with overflow
2019-09-27T05:58:40.2275922Z    |
2019-09-27T05:58:40.2275922Z    |
2019-09-27T05:58:40.2276034Z LL |       let n = 1_usize << BITS; //~ ERROR: attempt to shift left with overflow
2019-09-27T05:58:40.2276223Z 
2019-09-27T05:58:40.2276298Z error: aborting due to 3 previous errors
2019-09-27T05:58:40.2276379Z 
2019-09-27T05:58:40.2276424Z 
---
2019-09-27T05:58:40.2286374Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-27T05:58:40.2286558Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-27T05:58:40.2301037Z 
2019-09-27T05:58:40.2301330Z 
2019-09-27T05:58:40.2303942Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-27T05:58:40.2305039Z 
2019-09-27T05:58:40.2305234Z 
2019-09-27T05:58:40.2314214Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-27T05:58:40.2314696Z Build completed unsuccessfully in 1:31:00
2019-09-27T05:58:40.2314696Z Build completed unsuccessfully in 1:31:00
2019-09-27T05:58:40.2373411Z == clock drift check ==
2019-09-27T05:58:40.2391495Z   local time: Fri Sep 27 05:58:40 UTC 2019
2019-09-27T05:58:40.5053450Z   network time: Fri, 27 Sep 2019 05:58:40 GMT
2019-09-27T05:58:40.5053602Z == end clock drift check ==
2019-09-27T05:58:41.4878482Z ##[error]Bash exited with code '1'.
2019-09-27T05:58:41.4929867Z ##[section]Starting: Upload CPU usage statistics
2019-09-27T05:58:41.4937606Z ==============================================================================
2019-09-27T05:58:41.4937733Z Task         : Bash
2019-09-27T05:58:41.4937818Z Description  : Run a Bash script on macOS, Linux, or Windows
