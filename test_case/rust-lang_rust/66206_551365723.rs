plain
2019-11-08T02:00:15.2622942Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T02:00:15.2796144Z ##[command]git config gc.auto 0
2019-11-08T02:00:15.2872201Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T02:00:15.2920937Z ##[command]git config --get-all http.proxy
2019-11-08T02:00:15.3062123Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66206/merge:refs/remotes/pull/66206/merge
---
2019-11-08T02:59:30.3845396Z .................................................................................................... 1600/9288
2019-11-08T02:59:35.9650068Z .................................................................................................... 1700/9288
2019-11-08T02:59:48.0491632Z ................................................................i................................... 1800/9288
2019-11-08T02:59:55.7535598Z .................................................................................................... 1900/9288
2019-11-08T03:00:09.9936605Z ................................................iiiii............................................... 2000/9288
2019-11-08T03:00:20.3689024Z .................................................................................................... 2200/9288
2019-11-08T03:00:22.8808352Z .................................................................................................... 2300/9288
2019-11-08T03:00:26.4180701Z .................................................................................................... 2400/9288
2019-11-08T03:00:49.3169740Z .................................................................................................... 2500/9288
---
2019-11-08T03:03:26.3997102Z ............................................i...............i....................................... 4800/9288
2019-11-08T03:03:35.2966146Z .................................................................................................... 4900/9288
2019-11-08T03:03:42.3672967Z .................................................................................................... 5000/9288
2019-11-08T03:03:48.4638853Z .................................................................................................... 5100/9288
2019-11-08T03:03:58.1007231Z ..............................................ii.ii...........i..................................... 5200/9288
2019-11-08T03:04:07.5306236Z .................................................................................................... 5400/9288
2019-11-08T03:04:17.5446279Z .................................................................................................... 5500/9288
2019-11-08T03:04:24.7135454Z ............................i....................................................................... 5600/9288
2019-11-08T03:04:31.3550558Z .................................................................................................... 5700/9288
2019-11-08T03:04:31.3550558Z .................................................................................................... 5700/9288
2019-11-08T03:04:42.4686887Z .................................................................................................... 5800/9288
2019-11-08T03:04:53.5908505Z .............ii...i..ii............i................................................................ 5900/9288
2019-11-08T03:05:11.3579710Z .................................................................................................... 6100/9288
2019-11-08T03:05:18.6967254Z .................................................................................................... 6200/9288
2019-11-08T03:05:18.6967254Z .................................................................................................... 6200/9288
2019-11-08T03:05:32.1493069Z ................................i..ii............................................................... 6300/9288
2019-11-08T03:05:52.4137499Z .................................................................................................... 6500/9288
2019-11-08T03:05:54.6003442Z i................................................................................................... 6600/9288
2019-11-08T03:05:56.8481680Z ....................................................................................i............... 6700/9288
2019-11-08T03:05:59.4323003Z .................................................................................................... 6800/9288
---
2019-11-08T03:10:44.8681719Z 
2019-11-08T03:10:44.8682002Z 2   --> $DIR/issue-43988.rs:24:5
2019-11-08T03:10:44.8682056Z 3    |
2019-11-08T03:10:44.8682123Z 4 LL |     #[repr]
2019-11-08T03:10:44.8682447Z -    |     ^^^^^^^ help: must be of the form: `#[repr(C, packed, ...)]`
2019-11-08T03:10:44.8682512Z +    |     ^^^^^^^ help: must be of the form: `#[repr(C)]`
2019-11-08T03:10:44.8682632Z 7 error: malformed `repr` attribute input
2019-11-08T03:10:44.8682887Z 8   --> $DIR/issue-43988.rs:35:14
2019-11-08T03:10:44.8682924Z 
2019-11-08T03:10:44.8683218Z 9    |
2019-11-08T03:10:44.8683218Z 9    |
2019-11-08T03:10:44.8683512Z 10 LL |     let _z = #[repr] 1;
2019-11-08T03:10:44.8684033Z -    |              ^^^^^^^ help: must be of the form: `#[repr(C, packed, ...)]`
2019-11-08T03:10:44.8684292Z +    |              ^^^^^^^ help: must be of the form: `#[repr(C)]`
2019-11-08T03:10:44.8684640Z 13 error[E0518]: attribute should be applied to function or closure
2019-11-08T03:10:44.8685061Z 14   --> $DIR/issue-43988.rs:5:5
2019-11-08T03:10:44.8685248Z 
2019-11-08T03:10:44.8685394Z 
2019-11-08T03:10:44.8685394Z 
2019-11-08T03:10:44.8685580Z The actual stderr differed from the expected stderr.
2019-11-08T03:10:44.8686356Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43988/issue-43988.stderr
2019-11-08T03:10:44.8686871Z To update references, rerun the tests and pass the `--bless` flag
2019-11-08T03:10:44.8687848Z To only update this specific test, also pass `--test-args issues/issue-43988.rs`
2019-11-08T03:10:44.8688281Z error: 1 errors occurred comparing output.
2019-11-08T03:10:44.8688447Z status: exit code: 1
2019-11-08T03:10:44.8688447Z status: exit code: 1
2019-11-08T03:10:44.8689468Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43988.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43988" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43988/auxiliary" "-A" "unused"
2019-11-08T03:10:44.8690189Z ------------------------------------------
2019-11-08T03:10:44.8690484Z 
2019-11-08T03:10:44.8690898Z ------------------------------------------
2019-11-08T03:10:44.8691102Z stderr:
2019-11-08T03:10:44.8691102Z stderr:
2019-11-08T03:10:44.8691533Z ------------------------------------------
2019-11-08T03:10:44.8691754Z error: malformed `repr` attribute input
2019-11-08T03:10:44.8692185Z   --> /checkout/src/test/ui/issues/issue-43988.rs:24:5
2019-11-08T03:10:44.8692418Z    |
2019-11-08T03:10:44.8692578Z LL |     #[repr]
2019-11-08T03:10:44.8692757Z    |     ^^^^^^^ help: must be of the form: `#[repr(C)]`
2019-11-08T03:10:44.8693090Z error: malformed `repr` attribute input
2019-11-08T03:10:44.8693545Z   --> /checkout/src/test/ui/issues/issue-43988.rs:35:14
2019-11-08T03:10:44.8693756Z    |
2019-11-08T03:10:44.8693756Z    |
2019-11-08T03:10:44.8693916Z LL |     let _z = #[repr] 1;
2019-11-08T03:10:44.8694097Z    |              ^^^^^^^ help: must be of the form: `#[repr(C)]`
2019-11-08T03:10:44.8694418Z error[E0518]: attribute should be applied to function or closure
2019-11-08T03:10:44.8694872Z   --> /checkout/src/test/ui/issues/issue-43988.rs:5:5
2019-11-08T03:10:44.8695081Z    |
2019-11-08T03:10:44.8695241Z LL |     #[inline]
2019-11-08T03:10:44.8695241Z LL |     #[inline]
2019-11-08T03:10:44.8695415Z    |     ^^^^^^^^^
2019-11-08T03:10:44.8695721Z LL |     let _a = 4;
2019-11-08T03:10:44.8696229Z    |     ----------- not a function or closure
2019-11-08T03:10:44.8696446Z 
2019-11-08T03:10:44.8696621Z error[E0518]: attribute should be applied to function or closure
2019-11-08T03:10:44.8697491Z   --> /checkout/src/test/ui/issues/issue-43988.rs:10:5
2019-11-08T03:10:44.8697811Z    |
2019-11-08T03:10:44.8697993Z LL |     #[inline(XYZ)]
2019-11-08T03:10:44.8698164Z    |     ^^^^^^^^^^^^^^
2019-11-08T03:10:44.8698348Z LL |     let _b = 4;
2019-11-08T03:10:44.8698864Z    |     ----------- not a function or closure
2019-11-08T03:10:44.8699286Z error[E0517]: attribute should not be applied to a statement
2019-11-08T03:10:44.8699820Z   --> /checkout/src/test/ui/issues/issue-43988.rs:14:5
2019-11-08T03:10:44.8700903Z    |
2019-11-08T03:10:44.8701044Z LL |     #[repr(nothing)]
2019-11-08T03:10:44.8701044Z LL |     #[repr(nothing)]
2019-11-08T03:10:44.8701096Z    |     ^^^^^^^^^^^^^^^^
2019-11-08T03:10:44.8701146Z LL |     let _x = 0;
2019-11-08T03:10:44.8701567Z    |     ----------- not a struct, enum, or union
2019-11-08T03:10:44.8701667Z error[E0517]: attribute should not be applied to an expression
2019-11-08T03:10:44.8701973Z   --> /checkout/src/test/ui/issues/issue-43988.rs:18:5
2019-11-08T03:10:44.8702046Z    |
2019-11-08T03:10:44.8702046Z    |
2019-11-08T03:10:44.8702098Z LL |       #[repr(something_not_real)]
2019-11-08T03:10:44.8702220Z LL | /     loop {
2019-11-08T03:10:44.8702271Z LL | |         ()
2019-11-08T03:10:44.8702320Z LL | |     };
2019-11-08T03:10:44.8702320Z LL | |     };
2019-11-08T03:10:44.8702622Z    | |_____- not defining a struct, enum, or union
2019-11-08T03:10:44.8702870Z error[E0517]: attribute should not be applied to a statement
2019-11-08T03:10:44.8703206Z   --> /checkout/src/test/ui/issues/issue-43988.rs:24:5
2019-11-08T03:10:44.8703280Z    |
2019-11-08T03:10:44.8703331Z LL |     #[repr]
2019-11-08T03:10:44.8703331Z LL |     #[repr]
2019-11-08T03:10:44.8703380Z    |     ^^^^^^^
2019-11-08T03:10:44.8703448Z LL |     let _y = "123";
2019-11-08T03:10:44.8703749Z    |     --------------- not a struct, enum, or union
2019-11-08T03:10:44.8703844Z error[E0518]: attribute should be applied to function or closure
2019-11-08T03:10:44.8704156Z   --> /checkout/src/test/ui/issues/issue-43988.rs:31:5
2019-11-08T03:10:44.8704211Z    |
2019-11-08T03:10:44.8704211Z    |
2019-11-08T03:10:44.8704261Z LL |     #[inline(ABC)]
2019-11-08T03:10:44.8704376Z LL |     foo();
2019-11-08T03:10:44.8704648Z    |     ----- not a function or closure
2019-11-08T03:10:44.8704687Z 
2019-11-08T03:10:44.8704758Z error[E0517]: attribute should not be applied to an expression
2019-11-08T03:10:44.8704758Z error[E0517]: attribute should not be applied to an expression
2019-11-08T03:10:44.8705068Z   --> /checkout/src/test/ui/issues/issue-43988.rs:35:14
2019-11-08T03:10:44.8705123Z    |
2019-11-08T03:10:44.8705191Z LL |     let _z = #[repr] 1;
2019-11-08T03:10:44.8705493Z    |              ^^^^^^^ - not defining a struct, enum, or union
2019-11-08T03:10:44.8705593Z error: aborting due to 9 previous errors
2019-11-08T03:10:44.8705646Z 
2019-11-08T03:10:44.8705699Z Some errors have detailed explanations: E0517, E0518.
2019-11-08T03:10:44.8706004Z For more information about an error, try `rustc --explain E0517`.
---
2019-11-08T03:10:44.8726156Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-08T03:10:44.8726250Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-08T03:10:44.8743947Z 
2019-11-08T03:10:44.8744052Z 
2019-11-08T03:10:44.8746476Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-08T03:10:44.8746805Z 
2019-11-08T03:10:44.8747432Z 
2019-11-08T03:10:44.8752566Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-08T03:10:44.8752665Z Build completed unsuccessfully in 1:03:51
2019-11-08T03:10:44.8752665Z Build completed unsuccessfully in 1:03:51
2019-11-08T03:10:44.8809454Z == clock drift check ==
2019-11-08T03:10:44.8823821Z   local time: Fri Nov  8 03:10:44 UTC 2019
2019-11-08T03:10:45.1630324Z   network time: Fri, 08 Nov 2019 03:10:45 GMT
2019-11-08T03:10:45.1634592Z == end clock drift check ==
2019-11-08T03:10:46.0253750Z 
2019-11-08T03:10:46.0354057Z ##[error]Bash exited with code '1'.
2019-11-08T03:10:46.0384331Z ##[section]Starting: Checkout
2019-11-08T03:10:46.0386059Z ==============================================================================
2019-11-08T03:10:46.0386129Z Task         : Get sources
2019-11-08T03:10:46.0386171Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
