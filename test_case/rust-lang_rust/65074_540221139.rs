plain
2019-10-09T20:18:06.2422846Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T20:18:06.2548822Z ##[command]git config gc.auto 0
2019-10-09T20:18:06.2701332Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T20:18:06.2785315Z ##[command]git config --get-all http.proxy
2019-10-09T20:18:06.3010569Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65074/merge:refs/remotes/pull/65074/merge
---
2019-10-09T21:44:54.5979097Z .................................................................................................... 1600/9145
2019-10-09T21:45:05.2272455Z .................................................................................................... 1700/9145
2019-10-09T21:45:21.6175946Z .................i...............i.................................................................. 1800/9145
2019-10-09T21:45:31.8149401Z .................................................................................................... 1900/9145
2019-10-09T21:45:55.6271541Z ........iiiii....................................................................................... 2000/9145
2019-10-09T21:46:09.3300314Z .................................................................................................... 2200/9145
2019-10-09T21:46:13.0111731Z .................................................................................................... 2300/9145
2019-10-09T21:46:21.1415607Z .................................................................................................... 2400/9145
2019-10-09T21:46:29.8895790Z .................................................................................................... 2500/9145
---
2019-10-09T21:50:37.7791013Z .................................................................................................... 4700/9145
2019-10-09T21:50:48.3127758Z .i...............i.................................................................................. 4800/9145
2019-10-09T21:51:04.8155303Z .................................................................................................... 4900/9145
2019-10-09T21:51:12.8142096Z .................................................................................................... 5000/9145
2019-10-09T21:51:29.0697056Z ...............................................................................................ii.ii 5100/9145
2019-10-09T21:51:44.6195748Z .................................................................................................... 5300/9145
2019-10-09T21:51:57.7562436Z .................................................................................................... 5400/9145
2019-10-09T21:52:08.2644616Z ................................................................i................................... 5500/9145
2019-10-09T21:52:18.8734408Z .................................................................................................... 5600/9145
2019-10-09T21:52:18.8734408Z .................................................................................................... 5600/9145
2019-10-09T21:52:29.4047987Z .................................................................................................... 5700/9145
2019-10-09T21:52:46.4762599Z .............................................................ii...i..ii...........i................. 5800/9145
2019-10-09T21:53:22.8767945Z .................................................................................................... 6000/9145
2019-10-09T21:53:36.5106548Z .................................................................................................... 6100/9145
2019-10-09T21:53:36.5106548Z .................................................................................................... 6100/9145
2019-10-09T21:53:48.2783253Z ...................................................................i..ii............test [ui] ui/mpsc_stress.rs has been running for over 60 seconds
2019-10-09T21:54:16.1543160Z .................................................................................................... 6300/9145
2019-10-09T21:54:39.2443439Z .................................................................................................... 6400/9145
2019-10-09T21:54:41.7701043Z ...........................i........................................................................ 6500/9145
2019-10-09T21:54:44.9903974Z .................................................................................................... 6600/9145
---
2019-10-09T22:00:35.6994644Z .....................................................i.............................................. 9100/9145
2019-10-09T22:00:50.9660232Z .............................................
2019-10-09T22:00:50.9660749Z failures:
2019-10-09T22:00:50.9715698Z 
2019-10-09T22:00:50.9716461Z ---- [ui] ui/json_bom_plus_crlf_multifile_aux.rs stdout ----
2019-10-09T22:00:50.9716561Z normalized stderr:
2019-10-09T22:00:50.9716618Z error[E0601]: `main` function not found in crate `json_bom_plus_crlf_multifile_aux`
2019-10-09T22:00:50.9718105Z   --> $DIR/json_bom_plus_crlf_multifile_aux.rs:13:1
2019-10-09T22:00:50.9718177Z    |
2019-10-09T22:00:50.9718221Z LL | / pub fn test() {
2019-10-09T22:00:50.9718262Z LL | |
2019-10-09T22:00:50.9718331Z LL | |     let s : String = 1;  // Error in the middle of line.
2019-10-09T22:00:50.9720276Z ...  |
2019-10-09T22:00:50.9720344Z LL | |     );  // Error spanning the newline.
2019-10-09T22:00:50.9720389Z LL | | }
2019-10-09T22:00:50.9720389Z LL | | }
2019-10-09T22:00:50.9720440Z    | |_^ consider adding a `main` function to `$DIR/json_bom_plus_crlf_multifile_aux.rs`
2019-10-09T22:00:50.9720537Z error[E0308]: mismatched types
2019-10-09T22:00:50.9720537Z error[E0308]: mismatched types
2019-10-09T22:00:50.9721019Z   --> $DIR/json_bom_plus_crlf_multifile_aux.rs:15:22
2019-10-09T22:00:50.9721069Z    |
2019-10-09T22:00:50.9721138Z LL |     let s : String = 1;  // Error in the middle of line.
2019-10-09T22:00:50.9721238Z    |                      |
2019-10-09T22:00:50.9721308Z    |                      expected struct `std::string::String`, found integer
2019-10-09T22:00:50.9721362Z    |                      help: try using a conversion method: `1.to_string()`
2019-10-09T22:00:50.9721407Z    |
2019-10-09T22:00:50.9721407Z    |
2019-10-09T22:00:50.9721471Z    = note: expected type `std::string::String`
2019-10-09T22:00:50.9721778Z               found type `{integer}`
2019-10-09T22:00:50.9721867Z error[E0308]: mismatched types
2019-10-09T22:00:50.9721867Z error[E0308]: mismatched types
2019-10-09T22:00:50.9722162Z   --> $DIR/json_bom_plus_crlf_multifile_aux.rs:17:22
2019-10-09T22:00:50.9722252Z LL |     let s : String = 1
2019-10-09T22:00:50.9722315Z    |                      ^
2019-10-09T22:00:50.9722359Z    |                      |
2019-10-09T22:00:50.9722408Z    |                      expected struct `std::string::String`, found integer
2019-10-09T22:00:50.9722408Z    |                      expected struct `std::string::String`, found integer
2019-10-09T22:00:50.9722461Z    |                      help: try using a conversion method: `1.to_string()`
2019-10-09T22:00:50.9722537Z    |
2019-10-09T22:00:50.9722581Z    = note: expected type `std::string::String`
2019-10-09T22:00:50.9722628Z               found type `{integer}`
2019-10-09T22:00:50.9722718Z error[E0308]: mismatched types
2019-10-09T22:00:50.9722718Z error[E0308]: mismatched types
2019-10-09T22:00:50.9722949Z   --> $DIR/json_bom_plus_crlf_multifile_aux.rs:21:1
2019-10-09T22:00:50.9722996Z    |
2019-10-09T22:00:50.9723067Z LL | 1;  // Error after the newline.
2019-10-09T22:00:50.9723150Z    | |
2019-10-09T22:00:50.9723216Z    | expected struct `std::string::String`, found integer
2019-10-09T22:00:50.9723266Z    | help: try using a conversion method: `1.to_string()`
2019-10-09T22:00:50.9723310Z    |
2019-10-09T22:00:50.9723310Z    |
2019-10-09T22:00:50.9723445Z    = note: expected type `std::string::String`
2019-10-09T22:00:50.9723493Z               found type `{integer}`
2019-10-09T22:00:50.9723563Z error[E0308]: mismatched types
2019-10-09T22:00:50.9723563Z error[E0308]: mismatched types
2019-10-09T22:00:50.9723814Z   --> $DIR/json_bom_plus_crlf_multifile_aux.rs:23:22
2019-10-09T22:00:50.9724253Z LL |       let s : String = (
2019-10-09T22:00:50.9724832Z    |  ______________________^
2019-10-09T22:00:50.9724889Z LL | |     );  // Error spanning the newline.
2019-10-09T22:00:50.9724938Z    | |_____^ expected struct `std::string::String`, found ()
---
2019-10-09T22:00:50.9726308Z 
2019-10-09T22:00:50.9726335Z 
2019-10-09T22:00:50.9726360Z 
2019-10-09T22:00:50.9726408Z The actual stderr differed from the expected stderr.
2019-10-09T22:00:50.9726968Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json_bom_plus_crlf_multifile_aux/json_bom_plus_crlf_multifile_aux.stderr
2019-10-09T22:00:50.9727221Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T22:00:50.9727493Z To only update this specific test, also pass `--test-args json_bom_plus_crlf_multifile_aux.rs`
2019-10-09T22:00:50.9727604Z error: 1 errors occurred comparing output.
2019-10-09T22:00:50.9727649Z status: exit code: 1
2019-10-09T22:00:50.9727649Z status: exit code: 1
2019-10-09T22:00:50.9728427Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/json_bom_plus_crlf_multifile_aux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json_bom_plus_crlf_multifile_aux" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json_bom_plus_crlf_multifile_aux/auxiliary" "-A" "unused"
2019-10-09T22:00:50.9728763Z ------------------------------------------
2019-10-09T22:00:50.9728797Z 
2019-10-09T22:00:50.9730573Z ------------------------------------------
2019-10-09T22:00:50.9730631Z stderr:
2019-10-09T22:00:50.9730631Z stderr:
2019-10-09T22:00:50.9731052Z ------------------------------------------
2019-10-09T22:00:50.9731122Z error[E0601]: `main` function not found in crate `json_bom_plus_crlf_multifile_aux`
2019-10-09T22:00:50.9731408Z   --> /checkout/src/test/ui/json_bom_plus_crlf_multifile_aux.rs:13:1
2019-10-09T22:00:50.9731491Z    |
2019-10-09T22:00:50.9731533Z LL | / pub fn test() {
2019-10-09T22:00:50.9731575Z LL | |
2019-10-09T22:00:50.9731640Z LL | |     let s : String = 1;  // Error in the middle of line.
2019-10-09T22:00:50.9731724Z ...  |
2019-10-09T22:00:50.9731788Z LL | |     );  // Error spanning the newline.
2019-10-09T22:00:50.9731843Z LL | | }
2019-10-09T22:00:50.9731843Z LL | | }
2019-10-09T22:00:50.9731894Z    | |_^ consider adding a `main` function to `/checkout/src/test/ui/json_bom_plus_crlf_multifile_aux.rs`
2019-10-09T22:00:50.9731991Z error[E0308]: mismatched types
2019-10-09T22:00:50.9731991Z error[E0308]: mismatched types
2019-10-09T22:00:50.9732244Z   --> /checkout/src/test/ui/json_bom_plus_crlf_multifile_aux.rs:15:22
2019-10-09T22:00:50.9732778Z    |
2019-10-09T22:00:50.9732864Z LL |     let s : String = 1;  // Error in the middle of line.
2019-10-09T22:00:50.9732956Z    |                      |
2019-10-09T22:00:50.9733026Z    |                      expected struct `std::string::String`, found integer
2019-10-09T22:00:50.9733080Z    |                      help: try using a conversion method: `1.to_string()`
2019-10-09T22:00:50.9733124Z    |
2019-10-09T22:00:50.9733124Z    |
2019-10-09T22:00:50.9733168Z    = note: expected type `std::string::String`
2019-10-09T22:00:50.9733233Z               found type `{integer}`
2019-10-09T22:00:50.9733313Z error[E0308]: mismatched types
2019-10-09T22:00:50.9733313Z error[E0308]: mismatched types
2019-10-09T22:00:50.9733928Z   --> /checkout/src/test/ui/json_bom_plus_crlf_multifile_aux.rs:17:22
2019-10-09T22:00:50.9734026Z LL |     let s : String = 1
2019-10-09T22:00:50.9734069Z    |                      ^
2019-10-09T22:00:50.9734132Z    |                      |
2019-10-09T22:00:50.9734192Z    |                      expected struct `std::string::String`, found integer
2019-10-09T22:00:50.9734192Z    |                      expected struct `std::string::String`, found integer
2019-10-09T22:00:50.9734245Z    |                      help: try using a conversion method: `1.to_string()`
2019-10-09T22:00:50.9734312Z    |
2019-10-09T22:00:50.9734356Z    = note: expected type `std::string::String`
2019-10-09T22:00:50.9734402Z               found type `{integer}`
2019-10-09T22:00:50.9734640Z error[E0308]: mismatched types
2019-10-09T22:00:50.9734640Z error[E0308]: mismatched types
2019-10-09T22:00:50.9734921Z   --> /checkout/src/test/ui/json_bom_plus_crlf_multifile_aux.rs:21:1
2019-10-09T22:00:50.9734970Z    |
2019-10-09T22:00:50.9735034Z LL | 1;  // Error after the newline.
2019-10-09T22:00:50.9735307Z    | |
2019-10-09T22:00:50.9735373Z    | expected struct `std::string::String`, found integer
2019-10-09T22:00:50.9735422Z    | help: try using a conversion method: `1.to_string()`
2019-10-09T22:00:50.9735466Z    |
2019-10-09T22:00:50.9735466Z    |
2019-10-09T22:00:50.9735529Z    = note: expected type `std::string::String`
2019-10-09T22:00:50.9735584Z               found type `{integer}`
2019-10-09T22:00:50.9735654Z error[E0308]: mismatched types
2019-10-09T22:00:50.9735654Z error[E0308]: mismatched types
2019-10-09T22:00:50.9735955Z   --> /checkout/src/test/ui/json_bom_plus_crlf_multifile_aux.rs:23:22
2019-10-09T22:00:50.9736045Z LL |       let s : String = (
2019-10-09T22:00:50.9736108Z    |  ______________________^
2019-10-09T22:00:50.9736154Z LL | |     );  // Error spanning the newline.
2019-10-09T22:00:50.9736203Z    | |_____^ expected struct `std::string::String`, found ()
---
2019-10-09T22:00:50.9737175Z 
2019-10-09T22:00:50.9737202Z 
2019-10-09T22:00:50.9737227Z 
2019-10-09T22:00:50.9737320Z failures:
2019-10-09T22:00:50.9737366Z     [ui] ui/json_bom_plus_crlf_multifile_aux.rs
2019-10-09T22:00:50.9737999Z test result: FAILED. 9104 passed; 1 failed; 40 ignored; 0 measured; 0 filtered out
2019-10-09T22:00:50.9738256Z 
2019-10-09T22:00:50.9770767Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-09T22:00:50.9770876Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T22:00:50.9770876Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T22:00:50.9831759Z 
2019-10-09T22:00:50.9834287Z 
2019-10-09T22:00:50.9836102Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-09T22:00:50.9836396Z 
2019-10-09T22:00:50.9836426Z 
2019-10-09T22:00:50.9841177Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-09T22:00:50.9841271Z Build completed unsuccessfully in 1:34:54
2019-10-09T22:00:50.9841271Z Build completed unsuccessfully in 1:34:54
2019-10-09T22:00:50.9918401Z == clock drift check ==
2019-10-09T22:00:50.9938600Z   local time: Wed Oct  9 22:00:50 UTC 2019
2019-10-09T22:00:51.1948158Z   network time: Wed, 09 Oct 2019 22:00:51 GMT
2019-10-09T22:00:51.1952240Z == end clock drift check ==
2019-10-09T22:00:51.8296738Z ##[error]Bash exited with code '1'.
2019-10-09T22:00:51.8346871Z ##[section]Starting: Checkout
2019-10-09T22:00:51.8349139Z ==============================================================================
2019-10-09T22:00:51.8349210Z Task         : Get sources
2019-10-09T22:00:51.8349257Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
