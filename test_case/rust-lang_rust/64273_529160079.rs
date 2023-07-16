plain
2019-09-08T00:02:35.3611710Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T00:02:35.3820954Z ##[command]git config gc.auto 0
2019-09-08T00:02:35.3909782Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T00:02:36.0035859Z ##[command]git config --get-all http.proxy
2019-09-08T00:02:36.0043542Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64273/merge:refs/remotes/pull/64273/merge
---
2019-09-08T01:09:07.7366820Z .................................................................................................... 1500/9009
2019-09-08T01:09:14.0874375Z .................................................................................................... 1600/9009
2019-09-08T01:09:28.3111027Z ......................................................i...............i............................. 1700/9009
2019-09-08T01:09:36.8526039Z .................................................................................................... 1800/9009
2019-09-08T01:09:52.5366466Z .............................................iiiii.................................................. 1900/9009
2019-09-08T01:10:04.4780681Z .................................................................................................... 2100/9009
2019-09-08T01:10:07.3301462Z .................................................................................................... 2200/9009
2019-09-08T01:10:11.2168267Z .................................................................................................... 2300/9009
2019-09-08T01:10:19.8000138Z .................................................................................................... 2400/9009
---
2019-09-08T01:13:31.6177436Z .................................i...............i.................................................. 4700/9009
2019-09-08T01:13:44.3696159Z .................................................................................................... 4800/9009
2019-09-08T01:13:50.9928092Z .................................................................................................... 4900/9009
2019-09-08T01:14:02.5119979Z .................................................................................................... 5000/9009
2019-09-08T01:14:08.9431992Z ...............ii.ii................................................................................ 5100/9009
2019-09-08T01:14:20.3946145Z .................................................................................................... 5300/9009
2019-09-08T01:14:31.2637166Z ..............................................................................i..................... 5400/9009
2019-09-08T01:14:39.4092847Z .................................................................................................... 5500/9009
2019-09-08T01:14:45.9681552Z .................................................................................................... 5600/9009
2019-09-08T01:14:45.9681552Z .................................................................................................... 5600/9009
2019-09-08T01:14:57.3140184Z ........................................................................ii...i..ii...........i...... 5700/9009
2019-09-08T01:15:24.6483034Z .................................................................................................... 5900/9009
2019-09-08T01:15:35.5439649Z .................................................................................................... 6000/9009
2019-09-08T01:15:35.5439649Z .................................................................................................... 6000/9009
2019-09-08T01:15:46.4070941Z ..........................................................................i..ii..................... 6100/9009
2019-09-08T01:16:18.1159681Z .................................................................................................... 6300/9009
2019-09-08T01:16:20.3684792Z .................................i.................................................................. 6400/9009
2019-09-08T01:16:22.7064928Z .................................................................................................... 6500/9009
2019-09-08T01:16:25.5448714Z .....i.............................................................................................. 6600/9009
---
2019-09-08T01:20:46.1416442Z 
2019-09-08T01:20:46.1417485Z ---- [ui] ui/proc-macro/attributes-on-modules-fail.rs stdout ----
2019-09-08T01:20:46.1417746Z diff of stderr:
2019-09-08T01:20:46.1417889Z 
2019-09-08T01:20:46.1418424Z - error: `derive` may only be applied to structs, enums and unions
2019-09-08T01:20:46.1418900Z -   --> $DIR/attributes-on-modules-fail.rs:16:1
2019-09-08T01:20:46.1419355Z + error: couldn't read $DIR/outer/../module.rs: No such file or directory (os error 2)
2019-09-08T01:20:46.1419796Z +   --> $DIR/attributes-on-modules-fail.rs:25:9
2019-09-08T01:20:46.1420986Z - LL | #[derive(Copy)]
2019-09-08T01:20:46.1421715Z -    | ^^^^^^^^^^^^^^^
2019-09-08T01:20:46.1422099Z - 
2019-09-08T01:20:46.1422411Z - error[E0658]: non-inline modules in proc macro input are unstable
2019-09-08T01:20:46.1422411Z - error[E0658]: non-inline modules in proc macro input are unstable
2019-09-08T01:20:46.1422688Z -   --> $DIR/attributes-on-modules-fail.rs:20:1
2019-09-08T01:20:46.1422902Z -    |
2019-09-08T01:20:46.1423131Z - LL | mod module;
2019-09-08T01:20:46.1423542Z -    |
2019-09-08T01:20:46.1423542Z -    |
2019-09-08T01:20:46.1423972Z -    = note: for more information, see ***/issues/54727
2019-09-08T01:20:46.1424287Z -    = help: add `#![feature(proc_macro_hygiene)]` to the crate attributes to enable
2019-09-08T01:20:46.1424752Z - error[E0658]: non-inline modules in proc macro input are unstable
2019-09-08T01:20:46.1425019Z -   --> $DIR/attributes-on-modules-fail.rs:25:5
2019-09-08T01:20:46.1425219Z -    |
2019-09-08T01:20:46.1425274Z 19 LL |     mod module;
2019-09-08T01:20:46.1425274Z 19 LL |     mod module;
2019-09-08T01:20:46.1425506Z -    |     ^^^^^^^^^^^
2019-09-08T01:20:46.1425703Z -    |
2019-09-08T01:20:46.1426009Z -    = note: for more information, see ***/issues/54727
2019-09-08T01:20:46.1426339Z -    = help: add `#![feature(proc_macro_hygiene)]` to the crate attributes to enable
2019-09-08T01:20:46.1426690Z 24 
2019-09-08T01:20:46.1427785Z - error[E0412]: cannot find type `Y` in this scope
2019-09-08T01:20:46.1428080Z -   --> $DIR/attributes-on-modules-fail.rs:10:14
2019-09-08T01:20:46.1428283Z -    |
2019-09-08T01:20:46.1428283Z -    |
2019-09-08T01:20:46.1428519Z - LL |     type A = Y;
2019-09-08T01:20:46.1428978Z - help: a type alias with a similar name exists
2019-09-08T01:20:46.1429178Z -    |
2019-09-08T01:20:46.1429178Z -    |
2019-09-08T01:20:46.1429409Z - LL |     type A = A;
2019-09-08T01:20:46.1429897Z - help: possible candidate is found in another module, you can import it into scope
2019-09-08T01:20:46.1430628Z -    |
2019-09-08T01:20:46.1430854Z - LL |     use Y;
2019-09-08T01:20:46.1431049Z -    |
2019-09-08T01:20:46.1431049Z -    |
2019-09-08T01:20:46.1431599Z + error: aborting due to previous error
2019-09-08T01:20:46.1431680Z 38 
2019-09-08T01:20:46.1432015Z - error[E0412]: cannot find type `X` in this scope
2019-09-08T01:20:46.1432274Z -   --> $DIR/attributes-on-modules-fail.rs:14:10
2019-09-08T01:20:46.1432499Z -    |
2019-09-08T01:20:46.1432706Z - LL | type A = X;
2019-09-08T01:20:46.1433397Z - help: a type alias with a similar name exists
2019-09-08T01:20:46.1433608Z -    |
2019-09-08T01:20:46.1433608Z -    |
2019-09-08T01:20:46.1433813Z - LL | type A = A;
2019-09-08T01:20:46.1434346Z - help: possible candidate is found in another module, you can import it into scope
2019-09-08T01:20:46.1434547Z -    |
2019-09-08T01:20:46.1434547Z -    |
2019-09-08T01:20:46.1434751Z - LL | use m::X;
2019-09-08T01:20:46.1435154Z - 
2019-09-08T01:20:46.1435388Z - error: aborting due to 5 previous errors
2019-09-08T01:20:46.1435597Z - 
2019-09-08T01:20:46.1435851Z - Some errors have detailed explanations: E0412, E0658.
2019-09-08T01:20:46.1435851Z - Some errors have detailed explanations: E0412, E0658.
2019-09-08T01:20:46.1436400Z - For more information about an error, try `rustc --explain E0412`.
2019-09-08T01:20:46.1436475Z 57 
2019-09-08T01:20:46.1436510Z 
2019-09-08T01:20:46.1436539Z 
2019-09-08T01:20:46.1436599Z The actual stderr differed from the expected stderr.
2019-09-08T01:20:46.1436983Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attributes-on-modules-fail/attributes-on-modules-fail.stderr
2019-09-08T01:20:46.1437262Z To update references, rerun the tests and pass the `--bless` flag
2019-09-08T01:20:46.1437605Z To only update this specific test, also pass `--test-args proc-macro/attributes-on-modules-fail.rs`
2019-09-08T01:20:46.1437724Z error: 1 errors occurred comparing output.
2019-09-08T01:20:46.1437778Z status: exit code: 1
2019-09-08T01:20:46.1437778Z status: exit code: 1
2019-09-08T01:20:46.1438665Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/attributes-on-modules-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attributes-on-modules-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attributes-on-modules-fail/auxiliary" "-A" "unused"
2019-09-08T01:20:46.1439056Z ------------------------------------------
2019-09-08T01:20:46.1439096Z 
2019-09-08T01:20:46.1439354Z ------------------------------------------
2019-09-08T01:20:46.1439409Z stderr:
2019-09-08T01:20:46.1439409Z stderr:
2019-09-08T01:20:46.1439681Z ------------------------------------------
2019-09-08T01:20:46.1440522Z error: couldn't read /checkout/src/test/ui/proc-macro/outer/../module.rs: No such file or directory (os error 2)
2019-09-08T01:20:46.1440982Z    |
2019-09-08T01:20:46.1440982Z    |
2019-09-08T01:20:46.1441281Z LL |     mod module; //~ ERROR non-inline modules in proc macro input are unstable
2019-09-08T01:20:46.1441389Z 
2019-09-08T01:20:46.1441689Z error: aborting due to previous error
2019-09-08T01:20:46.1441736Z 
2019-09-08T01:20:46.1441764Z 
---
2019-09-08T01:20:46.1464984Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-08T01:20:46.1465335Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-08T01:20:46.1491944Z 
2019-09-08T01:20:46.1492036Z 
2019-09-08T01:20:46.1494538Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-08T01:20:46.1495258Z 
2019-09-08T01:20:46.1495289Z 
2019-09-08T01:20:46.1498318Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-08T01:20:46.1498572Z Build completed unsuccessfully in 1:10:50
2019-09-08T01:20:46.1498572Z Build completed unsuccessfully in 1:10:50
2019-09-08T01:20:46.1552454Z == clock drift check ==
2019-09-08T01:20:46.1572084Z   local time: Sun Sep  8 01:20:46 UTC 2019
2019-09-08T01:20:46.3091063Z   network time: Sun, 08 Sep 2019 01:20:46 GMT
2019-09-08T01:20:46.3091499Z == end clock drift check ==
2019-09-08T01:20:47.0791558Z ##[error]Bash exited with code '1'.
2019-09-08T01:20:47.0831656Z ##[section]Starting: Checkout
2019-09-08T01:20:47.0833822Z ==============================================================================
2019-09-08T01:20:47.0833897Z Task         : Get sources
2019-09-08T01:20:47.0833944Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
