plain
2019-09-17T10:42:52.6564381Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T10:42:52.6760619Z ##[command]git config gc.auto 0
2019-09-17T10:42:52.6842595Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T10:42:52.6912091Z ##[command]git config --get-all http.proxy
2019-09-17T10:42:52.7069450Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64436/merge:refs/remotes/pull/64436/merge
---
2019-09-17T11:44:51.3546157Z .................................................................................................... 1500/9023
2019-09-17T11:44:57.3710281Z .................................................................................................... 1600/9023
2019-09-17T11:45:09.9500519Z ...............................................................i...............i.................... 1700/9023
2019-09-17T11:45:17.1088050Z .................................................................................................... 1800/9023
2019-09-17T11:45:32.5358273Z ......................................................iiiii......................................... 1900/9023
2019-09-17T11:45:43.7459483Z .................................................................................................... 2100/9023
2019-09-17T11:45:46.1860257Z .................................................................................................... 2200/9023
2019-09-17T11:45:49.4898025Z .................................................................................................... 2300/9023
2019-09-17T11:45:57.6711222Z .................................................................................................... 2400/9023
---
2019-09-17T11:48:53.9030837Z ...........................................i...............i........................................ 4700/9023
2019-09-17T11:49:04.4628079Z .................................................................................................... 4800/9023
2019-09-17T11:49:11.5473697Z .................................................................................................... 4900/9023
2019-09-17T11:49:21.2794381Z .................................................................................................... 5000/9023
2019-09-17T11:49:29.0248431Z ...........................ii.ii.................................................................... 5100/9023
2019-09-17T11:49:38.9135754Z .................................................................................................... 5300/9023
2019-09-17T11:49:49.2382288Z ...........................................................................................i........ 5400/9023
2019-09-17T11:49:57.4246577Z .................................................................................................... 5500/9023
2019-09-17T11:50:02.3658004Z .................................................................................................... 5600/9023
2019-09-17T11:50:02.3658004Z .................................................................................................... 5600/9023
2019-09-17T11:50:12.8387200Z ......................................................................................ii...i..ii.... 5700/9023
2019-09-17T11:50:38.1839111Z .................................................................................................... 5900/9023
2019-09-17T11:50:48.2323100Z .................................................................................................... 6000/9023
2019-09-17T11:50:48.2323100Z .................................................................................................... 6000/9023
2019-09-17T11:50:54.9233345Z ........................................................................................i..ii....... 6100/9023
2019-09-17T11:51:23.6217848Z .................................................................................................... 6300/9023
2019-09-17T11:51:27.6091400Z ...............................................i.................................................... 6400/9023
2019-09-17T11:51:29.7870317Z .................................................................................................... 6500/9023
2019-09-17T11:51:32.2606513Z ...................i................................................................................ 6600/9023
---
2019-09-17T11:56:05.3846656Z  finished in 5.086
2019-09-17T11:56:05.4029207Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-17T11:56:05.5611246Z 
2019-09-17T11:56:05.5611816Z running 150 tests
2019-09-17T11:56:08.7624403Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-17T11:56:10.7147828Z ..iiii..............i..........iiii.......ii......
2019-09-17T11:56:10.7149684Z 
2019-09-17T11:56:10.7153919Z  finished in 5.312
2019-09-17T11:56:10.7354832Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-17T11:56:10.8952882Z 
---
2019-09-17T11:56:12.9037664Z  finished in 2.168
2019-09-17T11:56:12.9231567Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-17T11:56:13.0764944Z 
2019-09-17T11:56:13.0765966Z running 9 tests
2019-09-17T11:56:13.0766905Z iiiiiiiii
2019-09-17T11:56:13.0767868Z 
2019-09-17T11:56:13.0768052Z  finished in 0.153
2019-09-17T11:56:13.0933862Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-17T11:56:13.2505622Z 
---
2019-09-17T11:56:31.1340985Z  finished in 18.040
2019-09-17T11:56:31.1518599Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-17T11:56:31.3093246Z 
2019-09-17T11:56:31.3093610Z running 123 tests
2019-09-17T11:56:54.6731390Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-17T11:56:59.1849452Z i.i.i......iii.i.....ii
2019-09-17T11:56:59.1851652Z 
2019-09-17T11:56:59.1865163Z  finished in 28.033
2019-09-17T11:56:59.1865767Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-17T11:56:59.1866077Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-17T12:10:58.8756491Z 
2019-09-17T12:10:58.8758109Z    Doc-tests core
2019-09-17T12:11:04.1405080Z 
2019-09-17T12:11:04.1406412Z running 2400 tests
2019-09-17T12:11:15.8427346Z ......iiiii......................................................................................... 100/2400
2019-09-17T12:11:27.1796322Z ...........................................................................ii....................... 200/2400
2019-09-17T12:11:39.3819234Z ......................................................................................F..........i.. 300/2400
2019-09-17T12:11:52.7789394Z .................................................................................................... 400/2400
2019-09-17T12:12:03.3901642Z ............................................i..i.................iiii............................... 500/2400
2019-09-17T12:12:23.9398774Z .................................................................................................... 700/2400
2019-09-17T12:12:34.6505943Z .................................................................................................... 800/2400
2019-09-17T12:12:45.2432042Z .................................................................................................... 900/2400
2019-09-17T12:12:55.8584475Z .................................................................................................... 1000/2400
---
2019-09-17T12:15:32.6901787Z 
2019-09-17T12:15:32.6902154Z failures:
2019-09-17T12:15:32.6902335Z 
2019-09-17T12:15:32.6903668Z ---- intrinsics.rs - intrinsics::transmute (line 846) stdout ----
2019-09-17T12:15:32.6903783Z error[E0596]: cannot borrow `v_clone` as mutable, as it is not declared as mutable
2019-09-17T12:15:32.6904420Z    |
2019-09-17T12:15:32.6904420Z    |
2019-09-17T12:15:32.6904468Z 35 |     let v_clone = std::mem::ManuallyDrop::new(v_clone);
2019-09-17T12:15:32.6904804Z    |         ------- help: consider changing this to be mutable: `mut v_clone`
2019-09-17T12:15:32.6904865Z 36 |     Vec::from_raw_parts(v_clone.as_mut_ptr() as *mut Option<&i32>,
2019-09-17T12:15:32.6904917Z    |                         ^^^^^^^ cannot borrow as mutable
2019-09-17T12:15:32.6905016Z error: aborting due to previous error
2019-09-17T12:15:32.6905061Z 
2019-09-17T12:15:32.6905317Z For more information about this error, try `rustc --explain E0596`.
2019-09-17T12:15:32.6905534Z Couldn't compile the test.
---
2019-09-17T12:15:32.6905980Z 
2019-09-17T12:15:32.6945347Z error: test failed, to rerun pass '--doc'
2019-09-17T12:15:32.6962413Z 
2019-09-17T12:15:32.6963304Z 
2019-09-17T12:15:32.6964249Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "core" "--" "--quiet"
2019-09-17T12:15:32.6965231Z 
2019-09-17T12:15:32.6965371Z 
2019-09-17T12:15:32.6977355Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-17T12:15:32.6977600Z Build completed unsuccessfully in 1:25:49
2019-09-17T12:15:32.6977600Z Build completed unsuccessfully in 1:25:49
2019-09-17T12:15:32.7036969Z == clock drift check ==
2019-09-17T12:15:32.7051679Z   local time: Tue Sep 17 12:15:32 UTC 2019
2019-09-17T12:15:32.8681083Z   network time: Tue, 17 Sep 2019 12:15:32 GMT
2019-09-17T12:15:32.8681160Z == end clock drift check ==
2019-09-17T12:15:33.5216535Z ##[error]Bash exited with code '1'.
2019-09-17T12:15:33.5257944Z ##[section]Starting: Checkout
2019-09-17T12:15:33.5259781Z ==============================================================================
2019-09-17T12:15:33.5259833Z Task         : Get sources
2019-09-17T12:15:33.5259894Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
