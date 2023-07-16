plain
2019-10-26T20:49:21.7345600Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-26T20:49:22.5397998Z ##[command]git config gc.auto 0
2019-10-26T20:49:22.5403088Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-26T20:49:22.5407521Z ##[command]git config --get-all http.proxy
2019-10-26T20:49:22.5409978Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65851/merge:refs/remotes/pull/65851/merge
---
2019-10-26T21:24:39.7950676Z    Compiling rustc-rayon v0.3.0
2019-10-26T21:24:45.0178023Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-10-26T21:24:47.8719737Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-10-26T21:24:50.7348059Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-26T21:24:51.3811325Z warning: function is never used: `query`
2019-10-26T21:24:51.3811736Z   --> src/librustc_macros/src/query.rs:16:26
2019-10-26T21:24:51.3812026Z    |
2019-10-26T21:24:51.3812337Z 16 |     syn::custom_keyword!(query);
2019-10-26T21:24:51.3812905Z    |
2019-10-26T21:24:51.3813203Z    = note: `#[warn(dead_code)]` on by default
2019-10-26T21:24:51.3813275Z 
2019-10-26T21:24:51.3813275Z 
2019-10-26T21:24:51.3813574Z warning: function is never used: `Keywords`
2019-10-26T21:24:51.3813869Z   --> src/librustc_macros/src/symbols.rs:13:26
2019-10-26T21:24:51.3814119Z    |
2019-10-26T21:24:51.3814440Z 13 |     syn::custom_keyword!(Keywords);
2019-10-26T21:24:51.3814790Z 
2019-10-26T21:24:51.3815054Z warning: function is never used: `Symbols`
2019-10-26T21:24:51.3815371Z   --> src/librustc_macros/src/symbols.rs:14:26
2019-10-26T21:24:51.3815606Z    |
2019-10-26T21:24:51.3815606Z    |
2019-10-26T21:24:51.3815912Z 14 |     syn::custom_keyword!(Symbols);
2019-10-26T21:24:51.3816976Z 
2019-10-26T21:25:02.8977874Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-26T21:25:11.4438884Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-26T21:25:34.8285086Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
---
2019-10-26T21:53:58.9143616Z .................................................................................................... 1600/9256
2019-10-26T21:54:04.8773508Z .................................................................................................... 1700/9256
2019-10-26T21:54:17.8943235Z ..........................................................i......................................... 1800/9256
2019-10-26T21:54:26.1668704Z .................................................................................................... 1900/9256
2019-10-26T21:54:41.0795735Z ..........................................iiiii..................................................... 2000/9256
2019-10-26T21:54:52.1866323Z .................................................................................................... 2200/9256
2019-10-26T21:54:54.9249816Z .................................................................................................... 2300/9256
2019-10-26T21:54:59.3099896Z .................................................................................................... 2400/9256
2019-10-26T21:55:23.6278282Z .................................................................................................... 2500/9256
---
2019-10-26T21:58:25.5960426Z ............................................i...............i....................................... 4800/9256
2019-10-26T21:58:35.9591413Z .................................................................................................... 4900/9256
2019-10-26T21:58:44.2514869Z .................................................................................................... 5000/9256
2019-10-26T21:58:51.4766430Z .................................................................................................... 5100/9256
2019-10-26T21:59:01.5650424Z .............................................ii.ii...........i...................................... 5200/9256
2019-10-26T21:59:11.5879198Z .................................................................................................... 5400/9256
2019-10-26T21:59:21.4733438Z .................................................................................................... 5500/9256
2019-10-26T21:59:29.2067314Z .......................i............................................................................ 5600/9256
2019-10-26T21:59:35.3198332Z .................................................................................................... 5700/9256
2019-10-26T21:59:35.3198332Z .................................................................................................... 5700/9256
2019-10-26T21:59:47.7970940Z .................................................................................................... 5800/9256
2019-10-26T21:59:59.5446999Z ....................ii...i..ii...........i.......................................................... 5900/9256
2019-10-26T22:00:22.0600305Z .................................................................................................... 6100/9256
2019-10-26T22:00:31.4631991Z .................................................................................................... 6200/9256
2019-10-26T22:00:31.4631991Z .................................................................................................... 6200/9256
2019-10-26T22:00:46.6466305Z ...........................................i..ii.................................................... 6300/9256
2019-10-26T22:01:10.2330412Z .................................................................................................... 6500/9256
2019-10-26T22:01:12.4905190Z .........i.......................................................................................... 6600/9256
2019-10-26T22:01:14.7989451Z ....................................................................................i............... 6700/9256
2019-10-26T22:01:17.6689974Z .................................................................................................... 6800/9256
---
2019-10-26T22:05:35.2274270Z 52    |
2019-10-26T22:05:35.2274425Z 53 LL |     let _x : Bar();
2019-10-26T22:05:35.2274595Z 54    |              ^^^^^ not a type
2019-10-26T22:05:35.2274731Z +    |
2019-10-26T22:05:35.2274867Z 55 help: use `=` if you meant to assign
2019-10-26T22:05:35.2275178Z 57 LL |     let _x = Bar();
2019-10-26T22:05:35.2275302Z 
2019-10-26T22:05:35.2275432Z 
2019-10-26T22:05:35.2288028Z The actual stderr differed from the expected stderr.
2019-10-26T22:05:35.2288028Z The actual stderr differed from the expected stderr.
2019-10-26T22:05:35.2288691Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy-ns2/privacy-ns2.stderr
2019-10-26T22:05:35.2289182Z To update references, rerun the tests and pass the `--bless` flag
2019-10-26T22:05:35.2290150Z To only update this specific test, also pass `--test-args privacy/privacy-ns2.rs`
2019-10-26T22:05:35.2290567Z error: 1 errors occurred comparing output.
2019-10-26T22:05:35.2290718Z status: exit code: 1
2019-10-26T22:05:35.2290718Z status: exit code: 1
2019-10-26T22:05:35.2291606Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy-ns2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy-ns2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy-ns2/auxiliary" "-A" "unused"
2019-10-26T22:05:35.2292229Z ------------------------------------------
2019-10-26T22:05:35.2292417Z 
2019-10-26T22:05:35.2292783Z ------------------------------------------
2019-10-26T22:05:35.2293408Z stderr:
---
2019-10-26T22:05:35.2297234Z    |     ^^^
2019-10-26T22:05:35.2297380Z    |
2019-10-26T22:05:35.2297759Z help: a unit struct with a similar name exists
2019-10-26T22:05:35.2297939Z    |
2019-10-26T22:05:35.2298199Z LL |     Baz(); //~ ERROR expected function, found trait `Bar`
2019-10-26T22:05:35.2298532Z help: possible better candidates are found in other modules, you can import them into scope
2019-10-26T22:05:35.2298673Z    |
2019-10-26T22:05:35.2298810Z LL | use foo1::Bar;
2019-10-26T22:05:35.2298966Z    |
---
2019-10-26T22:05:35.2300659Z 
2019-10-26T22:05:35.2300806Z error[E0573]: expected type, found function `Bar`
2019-10-26T22:05:35.2301326Z   --> /checkout/src/test/ui/privacy/privacy-ns2.rs:41:18
2019-10-26T22:05:35.2301517Z    |
2019-10-26T22:05:35.2301674Z LL |     let _x : Box<Bar>; //~ ERROR expected type, found function `Bar`
2019-10-26T22:05:35.2302015Z    |
2019-10-26T22:05:35.2302171Z help: possible better candidates are found in other modules, you can import them into scope
2019-10-26T22:05:35.2302316Z    |
2019-10-26T22:05:35.2302453Z LL | use foo1::Bar;
---
2019-10-26T22:05:35.2303387Z 
2019-10-26T22:05:35.2303538Z error[E0573]: expected type, found function `Bar`
2019-10-26T22:05:35.2303914Z   --> /checkout/src/test/ui/privacy/privacy-ns2.rs:42:14
2019-10-26T22:05:35.2304087Z    |
2019-10-26T22:05:35.2304228Z LL |     let _x : Bar(); //~ ERROR expected type, found function `Bar`
2019-10-26T22:05:35.2304662Z    |
2019-10-26T22:05:35.2304662Z    |
2019-10-26T22:05:35.2304803Z help: use `=` if you meant to assign
2019-10-26T22:05:35.2304936Z    |
2019-10-26T22:05:35.2305084Z LL |     let _x = Bar(); //~ ERROR expected type, found function `Bar`
2019-10-26T22:05:35.2305590Z help: possible better candidates are found in other modules, you can import them into scope
2019-10-26T22:05:35.2308265Z    |
2019-10-26T22:05:35.2308491Z LL | use foo1::Bar;
2019-10-26T22:05:35.2308633Z    |
---
2019-10-26T22:05:35.2309356Z 
2019-10-26T22:05:35.2309905Z error[E0573]: expected type, found function `Bar`
2019-10-26T22:05:35.2310462Z   --> /checkout/src/test/ui/privacy/privacy-ns2.rs:48:17
2019-10-26T22:05:35.2310826Z    |
2019-10-26T22:05:35.2310976Z LL |     let _x: Box<Bar>; //~ ERROR expected type, found function `Bar`
2019-10-26T22:05:35.2311280Z    |
2019-10-26T22:05:35.2311419Z help: a struct with a similar name exists
2019-10-26T22:05:35.2311560Z    |
2019-10-26T22:05:35.2311560Z    |
2019-10-26T22:05:35.2311717Z LL |     let _x: Box<Baz>; //~ ERROR expected type, found function `Bar`
2019-10-26T22:05:35.2312056Z help: possible better candidates are found in other modules, you can import them into scope
2019-10-26T22:05:35.2312200Z    |
2019-10-26T22:05:35.2312338Z LL | use foo1::Bar;
2019-10-26T22:05:35.2312475Z    |
---
2019-10-26T22:05:35.2314705Z 
2019-10-26T22:05:35.2314836Z error[E0603]: trait `Bar` is private
2019-10-26T22:05:35.2315287Z   --> /checkout/src/test/ui/privacy/privacy-ns2.rs:61:15
2019-10-26T22:05:35.2315484Z    |
2019-10-26T22:05:35.2315618Z LL |     use foo3::Bar;  //~ ERROR `Bar` is private
2019-10-26T22:05:35.2315882Z 
2019-10-26T22:05:35.2316034Z error[E0603]: trait `Bar` is private
2019-10-26T22:05:35.2316519Z   --> /checkout/src/test/ui/privacy/privacy-ns2.rs:65:15
2019-10-26T22:05:35.2316723Z    |
2019-10-26T22:05:35.2316723Z    |
2019-10-26T22:05:35.2316856Z LL |     use foo3::Bar;  //~ ERROR `Bar` is private
2019-10-26T22:05:35.2317118Z 
2019-10-26T22:05:35.2317248Z error[E0603]: trait `Bar` is private
2019-10-26T22:05:35.2317768Z   --> /checkout/src/test/ui/privacy/privacy-ns2.rs:72:16
2019-10-26T22:05:35.2317974Z    |
2019-10-26T22:05:35.2317974Z    |
2019-10-26T22:05:35.2318309Z LL |     use foo3::{Bar,Baz};  //~ ERROR `Bar` is private
2019-10-26T22:05:35.2318582Z 
2019-10-26T22:05:35.2318740Z error: aborting due to 8 previous errors
2019-10-26T22:05:35.2318879Z 
2019-10-26T22:05:35.2319022Z Some errors have detailed explanations: E0423, E0573, E0603.
---
2019-10-26T22:05:35.2322620Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-26T22:05:35.2323032Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-26T22:05:35.2327973Z 
2019-10-26T22:05:35.2328203Z 
2019-10-26T22:05:35.2330933Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-26T22:05:35.2331380Z 
2019-10-26T22:05:35.2331429Z 
2019-10-26T22:05:35.2338295Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-26T22:05:35.2338537Z Build completed unsuccessfully in 1:08:42
2019-10-26T22:05:35.2338537Z Build completed unsuccessfully in 1:08:42
2019-10-26T22:05:35.2404602Z == clock drift check ==
2019-10-26T22:05:35.2420032Z   local time: Sat Oct 26 22:05:35 UTC 2019
2019-10-26T22:05:35.5209267Z   network time: Sat, 26 Oct 2019 22:05:35 GMT
2019-10-26T22:05:35.5211296Z == end clock drift check ==
2019-10-26T22:05:36.3494701Z 
2019-10-26T22:05:36.3608332Z ##[error]Bash exited with code '1'.
2019-10-26T22:05:36.3649253Z ##[section]Starting: Checkout
2019-10-26T22:05:36.3651638Z ==============================================================================
2019-10-26T22:05:36.3651699Z Task         : Get sources
2019-10-26T22:05:36.3651768Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
