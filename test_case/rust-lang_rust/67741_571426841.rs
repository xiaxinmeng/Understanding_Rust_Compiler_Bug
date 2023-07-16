plain
2020-01-07T03:24:10.6599301Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T03:24:10.6609271Z ##[command]git config gc.auto 0
2020-01-07T03:24:10.6614576Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T03:24:10.6617586Z ##[command]git config --get-all http.proxy
2020-01-07T03:24:10.6622927Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67741/merge:refs/remotes/pull/67741/merge
---
2020-01-07T04:10:57.8898993Z .................................................................................................... 1500/9478
2020-01-07T04:11:03.0077827Z .................................................................................................... 1600/9478
2020-01-07T04:11:07.4370961Z .................................................................................................... 1700/9478
2020-01-07T04:11:15.6778683Z .................................................................................................... 1800/9478
2020-01-07T04:11:22.8191739Z .i.................................................................................................. 1900/9478
2020-01-07T04:11:28.6913375Z .........................................................................................iiiii...... 2000/9478
2020-01-07T04:11:48.4943245Z .................................................................................................... 2200/9478
2020-01-07T04:11:49.8587645Z .................................................................................................... 2300/9478
2020-01-07T04:11:52.0435956Z .................................................................................................... 2400/9478
2020-01-07T04:11:57.3141528Z .................................................................................................... 2500/9478
---
2020-01-07T04:14:35.7894008Z .....................i...............i.............................................................. 4900/9478
2020-01-07T04:14:44.8491265Z .................................................................................................... 5000/9478
2020-01-07T04:14:50.1087672Z ..................................................................i................................. 5100/9478
2020-01-07T04:14:57.4721718Z .................................................................................................... 5200/9478
2020-01-07T04:15:04.4858808Z .................................ii.ii...........i.................................................. 5300/9478
2020-01-07T04:15:13.2292373Z .................................................................................................... 5500/9478
2020-01-07T04:15:22.3336529Z .................................................................................................... 5600/9478
2020-01-07T04:15:29.1312356Z .................i.................................................................................. 5700/9478
2020-01-07T04:15:34.7977229Z .................................................................................................... 5800/9478
2020-01-07T04:15:34.7977229Z .................................................................................................... 5800/9478
2020-01-07T04:15:44.8723805Z .................................................................................................... 5900/9478
2020-01-07T04:15:55.1825199Z .......ii...i..ii...........i....................................................................... 6000/9478
2020-01-07T04:16:10.8081820Z .................................................................................................... 6200/9478
2020-01-07T04:16:17.6310030Z .................................................................................................... 6300/9478
2020-01-07T04:16:17.6310030Z .................................................................................................... 6300/9478
2020-01-07T04:16:30.6527648Z ..................................i..ii............................................................. 6400/9478
2020-01-07T04:16:48.5524381Z .................................................................................................... 6600/9478
2020-01-07T04:16:50.3853814Z .........i.......................................................................................... 6700/9478
2020-01-07T04:16:52.4113985Z .................................................................................................... 6800/9478
2020-01-07T04:16:54.6769790Z .........i.......................................................................................... 6900/9478
---
2020-01-07T04:18:22.5463717Z .................................................................................................... 7500/9478
2020-01-07T04:18:26.2600329Z .................................................................................................... 7600/9478
2020-01-07T04:18:31.1576376Z ...............F.................................................................................... 7700/9478
2020-01-07T04:18:40.9304927Z .................................................................................................... 7800/9478
2020-01-07T04:18:48.4264820Z .............................................iiii................................................... 7900/9478
2020-01-07T04:19:02.0638484Z .................................................................................................... 8100/9478
2020-01-07T04:19:08.1468008Z .................................................................................................... 8200/9478
2020-01-07T04:19:21.9270747Z .................................................................................................... 8300/9478
2020-01-07T04:19:28.7038673Z .........................................................................F.......................... 8400/9478
---
2020-01-07T04:21:11.7174283Z diff of stderr:
2020-01-07T04:21:11.7174314Z 
2020-01-07T04:21:11.7178907Z 2   --> $DIR/const.rs:14:9
2020-01-07T04:21:11.7180530Z 3    |
2020-01-07T04:21:11.7180784Z 4 LL | const FOO: Foo = Foo{bar: 5};
2020-01-07T04:21:11.7181517Z -    | ----------------------------- `FOO` defined here
2020-01-07T04:21:11.7181884Z 6 ...
2020-01-07T04:21:11.7182030Z 7 LL |         FOO => {},
2020-01-07T04:21:11.7182075Z 8    |         ^^^
2020-01-07T04:21:11.7182122Z 
2020-01-07T04:21:11.7182122Z 
2020-01-07T04:21:11.7182164Z 9    |         |
2020-01-07T04:21:11.7182212Z 10    |         expected `&Foo`, found struct `Foo`
2020-01-07T04:21:11.7182518Z -    |         help: use different name to introduce a new binding: `other_foo`
2020-01-07T04:21:11.7183005Z -    = note: `FOO` is interpreted as a constant, not a new binding
2020-01-07T04:21:11.7183077Z +    |         `FOO` is interpreted as a constant, not a new binding
2020-01-07T04:21:11.7183129Z +    |         help: introduce a new binding instead: `other_foo`
2020-01-07T04:21:11.7183172Z 14 
2020-01-07T04:21:11.7183172Z 14 
2020-01-07T04:21:11.7183216Z 15 error: aborting due to previous error
2020-01-07T04:21:11.7183274Z 16 
2020-01-07T04:21:11.7183301Z 
2020-01-07T04:21:11.7183326Z 
2020-01-07T04:21:11.7183479Z The actual stderr differed from the expected stderr.
2020-01-07T04:21:11.7183924Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/const/const.stderr
2020-01-07T04:21:11.7184258Z To update references, rerun the tests and pass the `--bless` flag
2020-01-07T04:21:11.7184512Z To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/const.rs`
2020-01-07T04:21:11.7184767Z error: 1 errors occurred comparing output.
2020-01-07T04:21:11.7184818Z status: exit code: 1
2020-01-07T04:21:11.7184818Z status: exit code: 1
2020-01-07T04:21:11.7185633Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/const/auxiliary" "-A" "unused"
2020-01-07T04:21:11.7185926Z ------------------------------------------
2020-01-07T04:21:11.7185956Z 
2020-01-07T04:21:11.7186150Z ------------------------------------------
2020-01-07T04:21:11.7186327Z stderr:
2020-01-07T04:21:11.7186327Z stderr:
2020-01-07T04:21:11.7186543Z ------------------------------------------
2020-01-07T04:21:11.7186586Z error[E0308]: mismatched types
2020-01-07T04:21:11.7186864Z   --> /checkout/src/test/ui/rfc-2005-default-binding-mode/const.rs:14:9
2020-01-07T04:21:11.7186910Z    |
2020-01-07T04:21:11.7186952Z LL | const FOO: Foo = Foo{bar: 5};
2020-01-07T04:21:11.7187215Z ...
2020-01-07T04:21:11.7187254Z LL |         FOO => {}, //~ ERROR mismatched types
2020-01-07T04:21:11.7187293Z    |         ^^^
2020-01-07T04:21:11.7187348Z    |         |
2020-01-07T04:21:11.7187348Z    |         |
2020-01-07T04:21:11.7187387Z    |         expected `&Foo`, found struct `Foo`
2020-01-07T04:21:11.7187430Z    |         `FOO` is interpreted as a constant, not a new binding
2020-01-07T04:21:11.7187491Z    |         help: introduce a new binding instead: `other_foo`
2020-01-07T04:21:11.7187563Z error: aborting due to previous error
2020-01-07T04:21:11.7187594Z 
2020-01-07T04:21:11.7187823Z For more information about this error, try `rustc --explain E0308`.
2020-01-07T04:21:11.7187852Z 
---
2020-01-07T04:21:11.7188645Z 4 LL | struct foo;
2020-01-07T04:21:11.7188846Z -    | ----------- `foo` defined here
2020-01-07T04:21:11.7189040Z +    | ----------- unit struct defined here
2020-01-07T04:21:11.7189081Z 6 ...
2020-01-07T04:21:11.7189118Z 7 LL |     let Thing { foo } = t;
2020-01-07T04:21:11.7189358Z -    |                 ^^^ expected struct `std::string::String`, found struct `foo`
2020-01-07T04:21:11.7189740Z -    = note: `foo` is interpreted as a unit struct, not a new binding
2020-01-07T04:21:11.7189965Z - help: you can bind the struct field to a different name
2020-01-07T04:21:11.7190122Z -    |
2020-01-07T04:21:11.7190122Z -    |
2020-01-07T04:21:11.7190309Z - LL |     let Thing { foo: other_foo } = t;
2020-01-07T04:21:11.7190552Z +    |                 ^^^
2020-01-07T04:21:11.7190588Z +    |                 |
2020-01-07T04:21:11.7190649Z +    |                 expected struct `std::string::String`, found struct `foo`
2020-01-07T04:21:11.7190696Z +    |                 `foo` is interpreted as a unit struct, not a new binding
2020-01-07T04:21:11.7190696Z +    |                 `foo` is interpreted as a unit struct, not a new binding
2020-01-07T04:21:11.7190744Z +    |                 help: bind the struct field to a different name instead: `foo: other_foo`
2020-01-07T04:21:11.7190804Z 15 
2020-01-07T04:21:11.7190843Z 16 error: aborting due to previous error
2020-01-07T04:21:11.7190877Z 17 
2020-01-07T04:21:11.7190900Z 
2020-01-07T04:21:11.7190937Z 
2020-01-07T04:21:11.7191060Z The actual stderr differed from the expected stderr.
2020-01-07T04:21:11.7191375Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-in-struct-pat/const-in-struct-pat.stderr
2020-01-07T04:21:11.7191614Z To update references, rerun the tests and pass the `--bless` flag
2020-01-07T04:21:11.7191852Z To only update this specific test, also pass `--test-args suggestions/const-in-struct-pat.rs`
2020-01-07T04:21:11.7191923Z error: 1 errors occurred comparing output.
2020-01-07T04:21:11.7191977Z status: exit code: 1
2020-01-07T04:21:11.7191977Z status: exit code: 1
2020-01-07T04:21:11.7192722Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/const-in-struct-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-in-struct-pat" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-in-struct-pat/auxiliary" "-A" "unused"
2020-01-07T04:21:11.7193120Z ------------------------------------------
2020-01-07T04:21:11.7193165Z 
2020-01-07T04:21:11.7193355Z ------------------------------------------
2020-01-07T04:21:11.7193395Z stderr:
2020-01-07T04:21:11.7193395Z stderr:
2020-01-07T04:21:11.7193579Z ------------------------------------------
2020-01-07T04:21:11.7193638Z error[E0308]: mismatched types
2020-01-07T04:21:11.7193850Z   --> /checkout/src/test/ui/suggestions/const-in-struct-pat.rs:8:17
2020-01-07T04:21:11.7193894Z    |
2020-01-07T04:21:11.7193947Z LL | struct foo;
2020-01-07T04:21:11.7194133Z    | ----------- unit struct defined here
2020-01-07T04:21:11.7194171Z ...
2020-01-07T04:21:11.7194229Z LL |     let Thing { foo } = t; //~ ERROR mismatched types
2020-01-07T04:21:11.7194328Z    |                 |
2020-01-07T04:21:11.7194388Z    |                 expected struct `std::string::String`, found struct `foo`
2020-01-07T04:21:11.7194436Z    |                 `foo` is interpreted as a unit struct, not a new binding
2020-01-07T04:21:11.7194486Z    |                 help: bind the struct field to a different name instead: `foo: other_foo`
---
2020-01-07T04:21:11.7254397Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-07T04:21:11.7254501Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-07T04:21:11.7276565Z 
2020-01-07T04:21:11.7276639Z 
2020-01-07T04:21:11.7279506Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-07T04:21:11.7279782Z 
2020-01-07T04:21:11.7279828Z 
2020-01-07T04:21:11.7288139Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-07T04:21:11.7288265Z Build completed unsuccessfully in 0:51:45
2020-01-07T04:21:11.7288265Z Build completed unsuccessfully in 0:51:45
2020-01-07T04:21:11.7339372Z == clock drift check ==
2020-01-07T04:21:11.7359385Z   local time: Tue Jan  7 04:21:11 UTC 2020
2020-01-07T04:21:12.0075044Z   network time: Tue, 07 Jan 2020 04:21:12 GMT
2020-01-07T04:21:12.0080763Z == end clock drift check ==
2020-01-07T04:21:12.4134509Z 
2020-01-07T04:21:12.4193781Z ##[error]Bash exited with code '1'.
2020-01-07T04:21:12.4233666Z ##[section]Starting: Checkout
2020-01-07T04:21:12.4235416Z ==============================================================================
2020-01-07T04:21:12.4235492Z Task         : Get sources
2020-01-07T04:21:12.4235536Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
