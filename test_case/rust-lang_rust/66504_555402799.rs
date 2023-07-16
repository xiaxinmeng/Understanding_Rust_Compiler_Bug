plain
2019-11-19T07:54:05.8008451Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-19T07:54:05.8191456Z ##[command]git config gc.auto 0
2019-11-19T07:54:05.8252797Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-19T07:54:05.8323143Z ##[command]git config --get-all http.proxy
2019-11-19T07:54:05.8490709Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66504/merge:refs/remotes/pull/66504/merge
---
2019-11-19T08:51:18.0812795Z .................................................................................................... 1500/9253
2019-11-19T08:51:23.8869933Z .................................................................................................... 1600/9253
2019-11-19T08:51:32.3393842Z .................................................................................................... 1700/9253
2019-11-19T08:51:40.9950583Z ........i........................................................................................... 1800/9253
2019-11-19T08:51:47.3508669Z .............................................................................................iiiii.. 1900/9253
2019-11-19T08:52:07.6806601Z .................................................................................................... 2100/9253
2019-11-19T08:52:09.9313745Z .................................................................................................... 2200/9253
2019-11-19T08:52:12.3414972Z .................................................................................................... 2300/9253
2019-11-19T08:52:18.1071319Z .................................................................................................... 2400/9253
---
2019-11-19T08:55:50.4170729Z .................................................................................................... 5400/9253
2019-11-19T08:56:00.3597051Z ...............................................................................i.................... 5500/9253
2019-11-19T08:56:07.7866640Z .................................................................................................... 5600/9253
2019-11-19T08:56:13.9334675Z .................................................................................................... 5700/9253
2019-11-19T08:56:23.5584778Z .................................................................ii...i..ii...........i............. 5800/9253
2019-11-19T08:56:44.5175372Z .................................................................................................... 6000/9253
2019-11-19T08:56:52.5043377Z .................................................................................................... 6100/9253
2019-11-19T08:56:52.5043377Z .................................................................................................... 6100/9253
2019-11-19T08:56:56.6101135Z ......................................................................................i..ii......... 6200/9253
2019-11-19T08:57:22.7713192Z .................................................................................................... 6400/9253
2019-11-19T08:57:27.6630860Z ......................................................i............................................. 6500/9253
2019-11-19T08:57:29.7046254Z .................................................................................................... 6600/9253
2019-11-19T08:57:32.1529095Z ...........................................i........................................................ 6700/9253
---
2019-11-19T09:02:18.3636630Z 
2019-11-19T09:02:18.3636654Z 
2019-11-19T09:02:18.3636698Z The actual stderr differed from the expected stderr.
2019-11-19T09:02:18.3637173Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/issue-39018.stderr
2019-11-19T09:02:18.3637396Z To update references, rerun the tests and pass the `--bless` flag
2019-11-19T09:02:18.3638155Z To only update this specific test, also pass `--test-args span/issue-39018.rs`
2019-11-19T09:02:18.3638247Z error: 1 errors occurred comparing output.
2019-11-19T09:02:18.3638294Z status: exit code: 1
2019-11-19T09:02:18.3638294Z status: exit code: 1
2019-11-19T09:02:18.3639113Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-39018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/auxiliary" "-A" "unused"
2019-11-19T09:02:18.3639452Z ------------------------------------------
2019-11-19T09:02:18.3639485Z 
2019-11-19T09:02:18.3639695Z ------------------------------------------
2019-11-19T09:02:18.3639757Z stderr:
2019-11-19T09:02:18.3639757Z stderr:
2019-11-19T09:02:18.3639964Z ------------------------------------------
2019-11-19T09:02:18.3640016Z error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-11-19T09:02:18.3640262Z   --> /checkout/src/test/ui/span/issue-39018.rs:2:22
2019-11-19T09:02:18.3640314Z    |
2019-11-19T09:02:18.3640362Z LL |     let x = "Hello " + "World!";
2019-11-19T09:02:18.3640615Z    |             -------- ^ -------- &str
2019-11-19T09:02:18.3640683Z    |             |        |
2019-11-19T09:02:18.3640745Z    |             |        `+` cannot be used to concatenate two `&str` strings
2019-11-19T09:02:18.3640797Z    |             &str
2019-11-19T09:02:18.3640858Z    |
2019-11-19T09:02:18.3640937Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3641017Z    |
2019-11-19T09:02:18.3641068Z LL |     let x = "Hello ".to_owned() + "World!";
2019-11-19T09:02:18.3641304Z 
2019-11-19T09:02:18.3641304Z 
2019-11-19T09:02:18.3641364Z error[E0369]: binary operation `+` cannot be applied to type `World`
2019-11-19T09:02:18.3641894Z   --> /checkout/src/test/ui/span/issue-39018.rs:8:26
2019-11-19T09:02:18.3641932Z    |
2019-11-19T09:02:18.3641984Z LL |     let y = World::Hello + World::Goodbye;
2019-11-19T09:02:18.3642166Z    |             ------------ ^ -------------- World
2019-11-19T09:02:18.3642265Z    |             World
2019-11-19T09:02:18.3642300Z    |
2019-11-19T09:02:18.3642300Z    |
2019-11-19T09:02:18.3642339Z    = note: an implementation of `std::ops::Add` might be missing for `World`
2019-11-19T09:02:18.3642373Z 
2019-11-19T09:02:18.3642425Z error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-11-19T09:02:18.3642618Z   --> /checkout/src/test/ui/span/issue-39018.rs:11:22
2019-11-19T09:02:18.3642656Z    |
2019-11-19T09:02:18.3642708Z LL |     let x = "Hello " + "World!".to_owned();
2019-11-19T09:02:18.3642903Z    |             -------- ^ ------------------- std::string::String
2019-11-19T09:02:18.3642944Z    |             |        |
2019-11-19T09:02:18.3642986Z    |             |        `+` cannot be used to concatenate a `&str` with a `String`
2019-11-19T09:02:18.3643045Z    |             &str
2019-11-19T09:02:18.3643078Z    |
2019-11-19T09:02:18.3643242Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3643366Z    |
2019-11-19T09:02:18.3643404Z LL |     let x = "Hello ".to_owned() + &"World!".to_owned();
2019-11-19T09:02:18.3643486Z 
2019-11-19T09:02:18.3643486Z 
2019-11-19T09:02:18.3643525Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-19T09:02:18.3643751Z   --> /checkout/src/test/ui/span/issue-39018.rs:26:16
2019-11-19T09:02:18.3643804Z    |
2019-11-19T09:02:18.3643841Z LL |     let _ = &a + &b; //~ ERROR binary operation
2019-11-19T09:02:18.3644020Z    |             -- ^ -- &std::string::String
2019-11-19T09:02:18.3644077Z    |             |  |
2019-11-19T09:02:18.3644117Z    |             |  `+` cannot be used to concatenate two `&str` strings
2019-11-19T09:02:18.3644156Z    |             &std::string::String
2019-11-19T09:02:18.3644190Z    |
2019-11-19T09:02:18.3644264Z help: String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3644313Z    |
2019-11-19T09:02:18.3644349Z LL |     let _ = a + &b; //~ ERROR binary operation
2019-11-19T09:02:18.3644426Z 
2019-11-19T09:02:18.3644426Z 
2019-11-19T09:02:18.3644464Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-19T09:02:18.3644676Z   --> /checkout/src/test/ui/span/issue-39018.rs:27:16
2019-11-19T09:02:18.3644713Z    |
2019-11-19T09:02:18.3644749Z LL |     let _ = &a + b; //~ ERROR binary operation
2019-11-19T09:02:18.3644923Z    |             -- ^ - std::string::String
2019-11-19T09:02:18.3644977Z    |             |  |
2019-11-19T09:02:18.3645017Z    |             |  `+` cannot be used to concatenate a `&str` with a `String`
2019-11-19T09:02:18.3645056Z    |             &std::string::String
2019-11-19T09:02:18.3645108Z    |
2019-11-19T09:02:18.3645169Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3645223Z    |
2019-11-19T09:02:18.3645274Z LL |     let _ = a + &b; //~ ERROR binary operation
2019-11-19T09:02:18.3645332Z 
2019-11-19T09:02:18.3645386Z error[E0277]: cannot add `std::string::String` to `std::string::String`
2019-11-19T09:02:18.3645581Z   --> /checkout/src/test/ui/span/issue-39018.rs:29:15
2019-11-19T09:02:18.3645619Z    |
2019-11-19T09:02:18.3645619Z    |
2019-11-19T09:02:18.3645654Z LL |     let _ = a + b; //~ ERROR E0277
2019-11-19T09:02:18.3645713Z    |               ^ no implementation for `std::string::String + std::string::String`
2019-11-19T09:02:18.3645789Z    = help: the trait `std::ops::Add` is not implemented for `std::string::String`
2019-11-19T09:02:18.3645833Z 
2019-11-19T09:02:18.3645833Z 
2019-11-19T09:02:18.3645878Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-19T09:02:18.3646077Z   --> /checkout/src/test/ui/span/issue-39018.rs:30:15
2019-11-19T09:02:18.3646130Z    |
2019-11-19T09:02:18.3646166Z LL |     let _ = e + b; //~ ERROR binary operation
2019-11-19T09:02:18.3646342Z    |             - ^ - std::string::String
2019-11-19T09:02:18.3646381Z    |             | |
2019-11-19T09:02:18.3646437Z    |             | `+` cannot be used to concatenate a `&str` with a `String`
2019-11-19T09:02:18.3646477Z    |             &std::string::String
2019-11-19T09:02:18.3646510Z    |
2019-11-19T09:02:18.3646582Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3646630Z    |
2019-11-19T09:02:18.3646849Z LL |     let _ = e.to_owned() + &b; //~ ERROR binary operation
2019-11-19T09:02:18.3647012Z 
2019-11-19T09:02:18.3647012Z 
2019-11-19T09:02:18.3647054Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-19T09:02:18.3647883Z   --> /checkout/src/test/ui/span/issue-39018.rs:31:15
2019-11-19T09:02:18.3647936Z    |
2019-11-19T09:02:18.3647981Z LL |     let _ = e + &b; //~ ERROR binary operation
2019-11-19T09:02:18.3648259Z    |             - ^ -- &std::string::String
2019-11-19T09:02:18.3648307Z    |             | |
2019-11-19T09:02:18.3648355Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-19T09:02:18.3648404Z    |             &std::string::String
2019-11-19T09:02:18.3648462Z    |
2019-11-19T09:02:18.3648528Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3648599Z    |
2019-11-19T09:02:18.3648663Z LL |     let _ = e.to_owned() + &b; //~ ERROR binary operation
2019-11-19T09:02:18.3648747Z 
2019-11-19T09:02:18.3648747Z 
2019-11-19T09:02:18.3648811Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-19T09:02:18.3649050Z   --> /checkout/src/test/ui/span/issue-39018.rs:32:15
2019-11-19T09:02:18.3649095Z    |
2019-11-19T09:02:18.3649155Z LL |     let _ = e + d; //~ ERROR binary operation
2019-11-19T09:02:18.3649356Z    |             - ^ - &str
2019-11-19T09:02:18.3649403Z    |             | |
2019-11-19T09:02:18.3649450Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-19T09:02:18.3649515Z    |             &std::string::String
2019-11-19T09:02:18.3649557Z    |
2019-11-19T09:02:18.3649632Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3649708Z    |
2019-11-19T09:02:18.3649760Z LL |     let _ = e.to_owned() + d; //~ ERROR binary operation
2019-11-19T09:02:18.3649851Z 
2019-11-19T09:02:18.3649851Z 
2019-11-19T09:02:18.3649898Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-19T09:02:18.3650136Z   --> /checkout/src/test/ui/span/issue-39018.rs:33:15
2019-11-19T09:02:18.3650197Z    |
2019-11-19T09:02:18.3650241Z LL |     let _ = e + &d; //~ ERROR binary operation
2019-11-19T09:02:18.3650443Z    |             - ^ -- &&str
2019-11-19T09:02:18.3650488Z    |             | |
2019-11-19T09:02:18.3650553Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-19T09:02:18.3650602Z    |             &std::string::String
2019-11-19T09:02:18.3650643Z    |
2019-11-19T09:02:18.3650734Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3650800Z    |
2019-11-19T09:02:18.3650847Z LL |     let _ = e.to_owned() + &d; //~ ERROR binary operation
2019-11-19T09:02:18.3650938Z 
2019-11-19T09:02:18.3650938Z 
2019-11-19T09:02:18.3650983Z error[E0369]: binary operation `+` cannot be applied to type `&&str`
2019-11-19T09:02:18.3651388Z   --> /checkout/src/test/ui/span/issue-39018.rs:34:16
2019-11-19T09:02:18.3651592Z    |
2019-11-19T09:02:18.3651630Z LL |     let _ = &c + &d; //~ ERROR binary operation
2019-11-19T09:02:18.3651795Z    |             -- ^ -- &&str
2019-11-19T09:02:18.3651884Z    |             &&str
2019-11-19T09:02:18.3651917Z    |
2019-11-19T09:02:18.3651973Z    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2019-11-19T09:02:18.3652000Z 
2019-11-19T09:02:18.3652000Z 
2019-11-19T09:02:18.3652141Z error[E0369]: binary operation `+` cannot be applied to type `&&str`
2019-11-19T09:02:18.3652366Z   --> /checkout/src/test/ui/span/issue-39018.rs:35:16
2019-11-19T09:02:18.3652491Z    |
2019-11-19T09:02:18.3652526Z LL |     let _ = &c + d; //~ ERROR binary operation
2019-11-19T09:02:18.3652712Z    |             -- ^ - &str
2019-11-19T09:02:18.3652976Z    |             &&str
2019-11-19T09:02:18.3653013Z    |
2019-11-19T09:02:18.3653069Z    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2019-11-19T09:02:18.3653096Z 
2019-11-19T09:02:18.3653096Z 
2019-11-19T09:02:18.3653134Z error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-11-19T09:02:18.3653551Z   --> /checkout/src/test/ui/span/issue-39018.rs:36:15
2019-11-19T09:02:18.3653609Z    |
2019-11-19T09:02:18.3653647Z LL |     let _ = c + &d; //~ ERROR binary operation
2019-11-19T09:02:18.3653823Z    |             - ^ -- &&str
2019-11-19T09:02:18.3653878Z    |             | |
2019-11-19T09:02:18.3653928Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-19T09:02:18.3653968Z    |             &str
2019-11-19T09:02:18.3654153Z    |
2019-11-19T09:02:18.3654211Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3654262Z    |
2019-11-19T09:02:18.3654317Z LL |     let _ = c.to_owned() + &d; //~ ERROR binary operation
2019-11-19T09:02:18.3654383Z 
2019-11-19T09:02:18.3654383Z 
2019-11-19T09:02:18.3654421Z error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-11-19T09:02:18.3654646Z   --> /checkout/src/test/ui/span/issue-39018.rs:37:15
2019-11-19T09:02:18.3654686Z    |
2019-11-19T09:02:18.3654725Z LL |     let _ = c + d; //~ ERROR binary operation
2019-11-19T09:02:18.3654911Z    |             - ^ - &str
2019-11-19T09:02:18.3654958Z    |             | |
2019-11-19T09:02:18.3654999Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-19T09:02:18.3655061Z    |             &str
2019-11-19T09:02:18.3655096Z    |
2019-11-19T09:02:18.3655153Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-19T09:02:18.3655221Z    |
2019-11-19T09:02:18.3655260Z LL |     let _ = c.to_owned() + d; //~ ERROR binary operation
2019-11-19T09:02:18.3655325Z 
2019-11-19T09:02:18.3655378Z error: aborting due to 14 previous errors
2019-11-19T09:02:18.3655404Z 
2019-11-19T09:02:18.3655442Z Some errors have detailed explanations: E0277, E0369.
---
2019-11-19T09:02:18.3680157Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-19T09:02:18.3680239Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-19T09:02:18.3701891Z 
2019-11-19T09:02:18.3701959Z 
2019-11-19T09:02:18.3705826Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-19T09:02:18.3706123Z 
2019-11-19T09:02:18.3706151Z 
2019-11-19T09:02:18.3706577Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-19T09:02:18.3706643Z Build completed unsuccessfully in 1:02:17
2019-11-19T09:02:18.3706643Z Build completed unsuccessfully in 1:02:17
2019-11-19T09:02:18.3764414Z == clock drift check ==
2019-11-19T09:02:18.3778161Z   local time: Tue Nov 19 09:02:18 UTC 2019
2019-11-19T09:02:18.9251608Z   network time: Tue, 19 Nov 2019 09:02:18 GMT
2019-11-19T09:02:18.9251730Z == end clock drift check ==
2019-11-19T09:02:19.7911794Z 
2019-11-19T09:02:19.7971746Z ##[error]Bash exited with code '1'.
2019-11-19T09:02:19.8000219Z ##[section]Starting: Checkout
2019-11-19T09:02:19.8001981Z ==============================================================================
2019-11-19T09:02:19.8002026Z Task         : Get sources
2019-11-19T09:02:19.8002079Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
