plain
2019-11-18T01:54:14.1891902Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T01:54:14.2062739Z ##[command]git config gc.auto 0
2019-11-18T01:54:14.2133744Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T01:54:14.2199206Z ##[command]git config --get-all http.proxy
2019-11-18T01:54:14.2334726Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66504/merge:refs/remotes/pull/66504/merge
---
2019-11-18T02:54:49.6053365Z .................................................................................................... 1500/9251
2019-11-18T02:54:55.9577472Z .................................................................................................... 1600/9251
2019-11-18T02:55:04.8281076Z .................................................................................................... 1700/9251
2019-11-18T02:55:14.1529629Z ........i........................................................................................... 1800/9251
2019-11-18T02:55:21.0872504Z .............................................................................................iiiii.. 1900/9251
2019-11-18T02:55:43.4873658Z .................................................................................................... 2100/9251
2019-11-18T02:55:46.0414069Z .................................................................................................... 2200/9251
2019-11-18T02:55:48.6059563Z .................................................................................................... 2300/9251
2019-11-18T02:55:54.8943751Z .................................................................................................... 2400/9251
---
2019-11-18T02:59:43.5953685Z .................................................................................................... 5400/9251
2019-11-18T02:59:54.3913692Z ...............................................................................i.................... 5500/9251
2019-11-18T03:00:02.4454898Z .................................................................................................... 5600/9251
2019-11-18T03:00:09.0534422Z .................................................................................................... 5700/9251
2019-11-18T03:00:19.7906592Z .................................................................ii...i..ii...........i............. 5800/9251
2019-11-18T03:00:42.7780640Z .................................................................................................... 6000/9251
2019-11-18T03:00:51.2714638Z .................................................................................................... 6100/9251
2019-11-18T03:00:51.2714638Z .................................................................................................... 6100/9251
2019-11-18T03:00:55.5512298Z ....................................................................................i..ii........... 6200/9251
2019-11-18T03:01:23.5945714Z .................................................................................................... 6400/9251
2019-11-18T03:01:28.5464232Z ....................................................i............................................... 6500/9251
2019-11-18T03:01:30.8443829Z .................................................................................................... 6600/9251
2019-11-18T03:01:33.3285865Z .........................................i.......................................................... 6700/9251
---
2019-11-18T03:06:29.9126413Z -    = note: expected type `&str`
2019-11-18T03:06:29.9126867Z -               found type `std::string::String`
2019-11-18T03:06:29.9127103Z +    = help: the trait `std::ops::Add` is not implemented for `std::string::String`
2019-11-18T03:06:29.9127288Z 78 
2019-11-18T03:06:29.9127455Z 79 error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-18T03:06:29.9127874Z 80   --> $DIR/issue-39018.rs:30:15
2019-11-18T03:06:29.9128212Z 182 
2019-11-18T03:06:29.9128389Z 183 error: aborting due to 14 previous errors
2019-11-18T03:06:29.9128540Z 184 
2019-11-18T03:06:29.9128988Z - Some errors have detailed explanations: E0308, E0369.
---
2019-11-18T03:06:29.9130478Z 
2019-11-18T03:06:29.9130610Z 
2019-11-18T03:06:29.9130806Z The actual stderr differed from the expected stderr.
2019-11-18T03:06:29.9131300Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/issue-39018.stderr
2019-11-18T03:06:29.9131779Z To update references, rerun the tests and pass the `--bless` flag
2019-11-18T03:06:29.9132602Z To only update this specific test, also pass `--test-args span/issue-39018.rs`
2019-11-18T03:06:29.9132995Z error: 1 errors occurred comparing output.
2019-11-18T03:06:29.9133173Z status: exit code: 1
2019-11-18T03:06:29.9133173Z status: exit code: 1
2019-11-18T03:06:29.9134177Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-39018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/auxiliary" "-A" "unused"
2019-11-18T03:06:29.9134870Z ------------------------------------------
2019-11-18T03:06:29.9135049Z 
2019-11-18T03:06:29.9135501Z ------------------------------------------
2019-11-18T03:06:29.9135701Z stderr:
2019-11-18T03:06:29.9135701Z stderr:
2019-11-18T03:06:29.9136100Z ------------------------------------------
2019-11-18T03:06:29.9136324Z error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-11-18T03:06:29.9136900Z   --> /checkout/src/test/ui/span/issue-39018.rs:2:22
2019-11-18T03:06:29.9137152Z    |
2019-11-18T03:06:29.9137350Z LL |     let x = "Hello " + "World!";
2019-11-18T03:06:29.9137787Z    |             -------- ^ -------- &str
2019-11-18T03:06:29.9138025Z    |             |        |
2019-11-18T03:06:29.9138196Z    |             |        `+` cannot be used to concatenate two `&str` strings
2019-11-18T03:06:29.9138378Z    |             &str
2019-11-18T03:06:29.9138552Z    |
2019-11-18T03:06:29.9138745Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9138938Z    |
2019-11-18T03:06:29.9139110Z LL |     let x = "Hello ".to_owned() + "World!";
2019-11-18T03:06:29.9139413Z 
2019-11-18T03:06:29.9139413Z 
2019-11-18T03:06:29.9139597Z error[E0369]: binary operation `+` cannot be applied to type `World`
2019-11-18T03:06:29.9140043Z   --> /checkout/src/test/ui/span/issue-39018.rs:8:26
2019-11-18T03:06:29.9140249Z    |
2019-11-18T03:06:29.9140411Z LL |     let y = World::Hello + World::Goodbye;
2019-11-18T03:06:29.9140852Z    |             ------------ ^ -------------- World
2019-11-18T03:06:29.9141235Z    |             World
2019-11-18T03:06:29.9141386Z    |
2019-11-18T03:06:29.9141386Z    |
2019-11-18T03:06:29.9141544Z    = note: an implementation of `std::ops::Add` might be missing for `World`
2019-11-18T03:06:29.9141698Z 
2019-11-18T03:06:29.9141861Z error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-11-18T03:06:29.9142365Z   --> /checkout/src/test/ui/span/issue-39018.rs:11:22
2019-11-18T03:06:29.9142589Z    |
2019-11-18T03:06:29.9142749Z LL |     let x = "Hello " + "World!".to_owned();
2019-11-18T03:06:29.9145999Z    |             -------- ^ ------------------- std::string::String
2019-11-18T03:06:29.9149318Z    |             |        |
2019-11-18T03:06:29.9149801Z    |             |        `+` cannot be used to concatenate a `&str` with a `String`
2019-11-18T03:06:29.9150127Z    |             &str
2019-11-18T03:06:29.9150380Z    |
2019-11-18T03:06:29.9150676Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9150938Z    |
2019-11-18T03:06:29.9151176Z LL |     let x = "Hello ".to_owned() + &"World!".to_owned();
2019-11-18T03:06:29.9151663Z 
2019-11-18T03:06:29.9151663Z 
2019-11-18T03:06:29.9152731Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-18T03:06:29.9153497Z   --> /checkout/src/test/ui/span/issue-39018.rs:26:16
2019-11-18T03:06:29.9153735Z    |
2019-11-18T03:06:29.9153922Z LL |     let _ = &a + &b; //~ ERROR binary operation
2019-11-18T03:06:29.9154381Z    |             -- ^ -- &std::string::String
2019-11-18T03:06:29.9154594Z    |             |  |
2019-11-18T03:06:29.9154783Z    |             |  `+` cannot be used to concatenate two `&str` strings
2019-11-18T03:06:29.9154964Z    |             &std::string::String
2019-11-18T03:06:29.9155115Z    |
2019-11-18T03:06:29.9155318Z help: String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9155503Z    |
2019-11-18T03:06:29.9155672Z LL |     let _ = a + &b; //~ ERROR binary operation
2019-11-18T03:06:29.9156003Z 
2019-11-18T03:06:29.9156003Z 
2019-11-18T03:06:29.9156192Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-18T03:06:29.9156725Z   --> /checkout/src/test/ui/span/issue-39018.rs:27:16
2019-11-18T03:06:29.9156955Z    |
2019-11-18T03:06:29.9157124Z LL |     let _ = &a + b; //~ ERROR binary operation
2019-11-18T03:06:29.9157744Z    |             -- ^ - std::string::String
2019-11-18T03:06:29.9158018Z    |             |  |
2019-11-18T03:06:29.9158496Z    |             |  `+` cannot be used to concatenate a `&str` with a `String`
2019-11-18T03:06:29.9163048Z    |             &std::string::String
2019-11-18T03:06:29.9163324Z    |
2019-11-18T03:06:29.9163525Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9163740Z    |
2019-11-18T03:06:29.9163903Z LL |     let _ = a + &b; //~ ERROR binary operation
2019-11-18T03:06:29.9164583Z 
2019-11-18T03:06:29.9164754Z error[E0277]: cannot add `std::string::String` to `std::string::String`
2019-11-18T03:06:29.9165368Z   --> /checkout/src/test/ui/span/issue-39018.rs:29:15
2019-11-18T03:06:29.9165595Z    |
2019-11-18T03:06:29.9165595Z    |
2019-11-18T03:06:29.9165772Z LL |     let _ = a + b; //~ ERROR mismatched types
2019-11-18T03:06:29.9165957Z    |               ^ no implementation for `std::string::String + std::string::String`
2019-11-18T03:06:29.9166281Z    = help: the trait `std::ops::Add` is not implemented for `std::string::String`
2019-11-18T03:06:29.9166446Z 
2019-11-18T03:06:29.9166446Z 
2019-11-18T03:06:29.9166606Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-18T03:06:29.9167074Z   --> /checkout/src/test/ui/span/issue-39018.rs:30:15
2019-11-18T03:06:29.9167283Z    |
2019-11-18T03:06:29.9167448Z LL |     let _ = e + b; //~ ERROR binary operation
2019-11-18T03:06:29.9167894Z    |             - ^ - std::string::String
2019-11-18T03:06:29.9168125Z    |             | |
2019-11-18T03:06:29.9168296Z    |             | `+` cannot be used to concatenate a `&str` with a `String`
2019-11-18T03:06:29.9168482Z    |             &std::string::String
2019-11-18T03:06:29.9169552Z    |
2019-11-18T03:06:29.9169920Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9170153Z    |
2019-11-18T03:06:29.9170323Z LL |     let _ = e.to_owned() + &b; //~ ERROR binary operation
2019-11-18T03:06:29.9170665Z 
2019-11-18T03:06:29.9170665Z 
2019-11-18T03:06:29.9170856Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-18T03:06:29.9171446Z   --> /checkout/src/test/ui/span/issue-39018.rs:31:15
2019-11-18T03:06:29.9171687Z    |
2019-11-18T03:06:29.9172439Z LL |     let _ = e + &b; //~ ERROR binary operation
2019-11-18T03:06:29.9172972Z    |             - ^ -- &std::string::String
2019-11-18T03:06:29.9173230Z    |             | |
2019-11-18T03:06:29.9173416Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-18T03:06:29.9173589Z    |             &std::string::String
2019-11-18T03:06:29.9173744Z    |
2019-11-18T03:06:29.9173945Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9174119Z    |
2019-11-18T03:06:29.9174276Z LL |     let _ = e.to_owned() + &b; //~ ERROR binary operation
2019-11-18T03:06:29.9174568Z 
2019-11-18T03:06:29.9174568Z 
2019-11-18T03:06:29.9174766Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-18T03:06:29.9175215Z   --> /checkout/src/test/ui/span/issue-39018.rs:32:15
2019-11-18T03:06:29.9175437Z    |
2019-11-18T03:06:29.9175596Z LL |     let _ = e + d; //~ ERROR binary operation
2019-11-18T03:06:29.9176011Z    |             - ^ - &str
2019-11-18T03:06:29.9176223Z    |             | |
2019-11-18T03:06:29.9176560Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-18T03:06:29.9176777Z    |             &std::string::String
2019-11-18T03:06:29.9176933Z    |
2019-11-18T03:06:29.9177118Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9177304Z    |
2019-11-18T03:06:29.9177461Z LL |     let _ = e.to_owned() + d; //~ ERROR binary operation
2019-11-18T03:06:29.9177775Z 
2019-11-18T03:06:29.9177775Z 
2019-11-18T03:06:29.9177941Z error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-11-18T03:06:29.9178444Z   --> /checkout/src/test/ui/span/issue-39018.rs:33:15
2019-11-18T03:06:29.9178658Z    |
2019-11-18T03:06:29.9178832Z LL |     let _ = e + &d; //~ ERROR binary operation
2019-11-18T03:06:29.9179231Z    |             - ^ -- &&str
2019-11-18T03:06:29.9179449Z    |             | |
2019-11-18T03:06:29.9179704Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-18T03:06:29.9179874Z    |             &std::string::String
2019-11-18T03:06:29.9180055Z    |
2019-11-18T03:06:29.9180280Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9180467Z    |
2019-11-18T03:06:29.9180637Z LL |     let _ = e.to_owned() + &d; //~ ERROR binary operation
2019-11-18T03:06:29.9180984Z 
2019-11-18T03:06:29.9180984Z 
2019-11-18T03:06:29.9181176Z error[E0369]: binary operation `+` cannot be applied to type `&&str`
2019-11-18T03:06:29.9181662Z   --> /checkout/src/test/ui/span/issue-39018.rs:34:16
2019-11-18T03:06:29.9181885Z    |
2019-11-18T03:06:29.9182361Z LL |     let _ = &c + &d; //~ ERROR binary operation
2019-11-18T03:06:29.9182885Z    |             -- ^ -- &&str
2019-11-18T03:06:29.9183965Z    |             &&str
2019-11-18T03:06:29.9184143Z    |
2019-11-18T03:06:29.9184305Z    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2019-11-18T03:06:29.9184443Z 
2019-11-18T03:06:29.9184443Z 
2019-11-18T03:06:29.9184619Z error[E0369]: binary operation `+` cannot be applied to type `&&str`
2019-11-18T03:06:29.9185123Z   --> /checkout/src/test/ui/span/issue-39018.rs:35:16
2019-11-18T03:06:29.9185336Z    |
2019-11-18T03:06:29.9185513Z LL |     let _ = &c + d; //~ ERROR binary operation
2019-11-18T03:06:29.9185901Z    |             -- ^ - &str
2019-11-18T03:06:29.9186495Z    |             &&str
2019-11-18T03:06:29.9186654Z    |
2019-11-18T03:06:29.9186831Z    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2019-11-18T03:06:29.9186970Z 
2019-11-18T03:06:29.9186970Z 
2019-11-18T03:06:29.9187137Z error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-11-18T03:06:29.9187618Z   --> /checkout/src/test/ui/span/issue-39018.rs:36:15
2019-11-18T03:06:29.9187825Z    |
2019-11-18T03:06:29.9187981Z LL |     let _ = c + &d; //~ ERROR binary operation
2019-11-18T03:06:29.9188382Z    |             - ^ -- &&str
2019-11-18T03:06:29.9188589Z    |             | |
2019-11-18T03:06:29.9188753Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-18T03:06:29.9188921Z    |             &str
2019-11-18T03:06:29.9189067Z    |
2019-11-18T03:06:29.9189250Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9189455Z    |
2019-11-18T03:06:29.9189608Z LL |     let _ = c.to_owned() + &d; //~ ERROR binary operation
2019-11-18T03:06:29.9189910Z 
2019-11-18T03:06:29.9189910Z 
2019-11-18T03:06:29.9190183Z error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-11-18T03:06:29.9190681Z   --> /checkout/src/test/ui/span/issue-39018.rs:37:15
2019-11-18T03:06:29.9190910Z    |
2019-11-18T03:06:29.9191065Z LL |     let _ = c + d; //~ ERROR binary operation
2019-11-18T03:06:29.9191467Z    |             - ^ - &str
2019-11-18T03:06:29.9191670Z    |             | |
2019-11-18T03:06:29.9191829Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-11-18T03:06:29.9192376Z    |             &str
2019-11-18T03:06:29.9192618Z    |
2019-11-18T03:06:29.9192802Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-11-18T03:06:29.9193027Z    |
2019-11-18T03:06:29.9193194Z LL |     let _ = c.to_owned() + d; //~ ERROR binary operation
2019-11-18T03:06:29.9193528Z 
2019-11-18T03:06:29.9193690Z error: aborting due to 14 previous errors
2019-11-18T03:06:29.9193831Z 
2019-11-18T03:06:29.9194011Z Some errors have detailed explanations: E0277, E0369.
---
2019-11-18T03:06:29.9197650Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-18T03:06:29.9197893Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-18T03:06:29.9198051Z 
2019-11-18T03:06:29.9198195Z 
2019-11-18T03:06:29.9200263Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-18T03:06:29.9200988Z 
2019-11-18T03:06:29.9201156Z 
2019-11-18T03:06:29.9201324Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-18T03:06:29.9201497Z Build completed unsuccessfully in 1:06:03
2019-11-18T03:06:29.9201497Z Build completed unsuccessfully in 1:06:03
2019-11-18T03:06:29.9224297Z == clock drift check ==
2019-11-18T03:06:29.9244808Z   local time: Mon Nov 18 03:06:29 UTC 2019
2019-11-18T03:06:29.9609789Z   network time: Mon, 18 Nov 2019 03:06:29 GMT
2019-11-18T03:06:29.9609962Z == end clock drift check ==
2019-11-18T03:06:30.7081727Z 
2019-11-18T03:06:30.7195173Z ##[error]Bash exited with code '1'.
2019-11-18T03:06:30.7235916Z ##[section]Starting: Checkout
2019-11-18T03:06:30.7237821Z ==============================================================================
2019-11-18T03:06:30.7237900Z Task         : Get sources
2019-11-18T03:06:30.7237953Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
