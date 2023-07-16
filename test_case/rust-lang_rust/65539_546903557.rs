plain
2019-10-28T10:23:10.7717607Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T10:23:10.7998836Z ##[command]git config gc.auto 0
2019-10-28T10:23:10.8069081Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T10:23:10.8118987Z ##[command]git config --get-all http.proxy
2019-10-28T10:23:10.8255833Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65539/merge:refs/remotes/pull/65539/merge
---
2019-10-28T11:14:56.4894084Z .................................................................................................... 1600/9259
2019-10-28T11:15:01.5640924Z .................................................................................................... 1700/9259
2019-10-28T11:15:12.2159670Z ..........................................................i...............i......................... 1800/9259
2019-10-28T11:15:18.8078605Z .................................................................................................... 1900/9259
2019-10-28T11:15:31.4098895Z ................................................iiiii............................................... 2000/9259
2019-10-28T11:15:40.7555804Z .................................................................................................... 2200/9259
2019-10-28T11:15:43.0228886Z .................................................................................................... 2300/9259
2019-10-28T11:15:46.2654448Z .................................................................................................... 2400/9259
2019-10-28T11:16:06.2008236Z .................................................................................................... 2500/9259
---
2019-10-28T11:18:37.2056074Z ..................................................i..............i.................................. 4800/9259
2019-10-28T11:18:44.9109902Z .................................................................................................... 4900/9259
2019-10-28T11:18:52.3853613Z .................................................................................................... 5000/9259
2019-10-28T11:18:57.8428591Z .................................................................................................... 5100/9259
2019-10-28T11:19:06.9094098Z ..................................................ii.ii...........i................................. 5200/9259
2019-10-28T11:19:15.3925602Z .................................................................................................... 5400/9259
2019-10-28T11:19:23.6137609Z .................................................................................................... 5500/9259
2019-10-28T11:19:30.4632772Z ....................i............................................................................... 5600/9259
2019-10-28T11:19:35.6711830Z .................................................................................................... 5700/9259
2019-10-28T11:19:35.6711830Z .................................................................................................... 5700/9259
2019-10-28T11:19:45.7942600Z .................................................................................................... 5800/9259
2019-10-28T11:19:56.5495021Z .....ii...i..ii...........i......................................................................... 5900/9259
2019-10-28T11:20:15.3431185Z .................................................................................................... 6100/9259
2019-10-28T11:20:19.0686682Z .................................................................................................... 6200/9259
2019-10-28T11:20:19.0686682Z .................................................................................................... 6200/9259
2019-10-28T11:20:31.0826401Z ........................i..ii....................................................................... 6300/9259
2019-10-28T11:20:48.5546285Z ..........................................................................................i......... 6500/9259
2019-10-28T11:20:50.5234327Z .................................................................................................... 6600/9259
2019-10-28T11:20:52.5664168Z .................................................................i.................................. 6700/9259
2019-10-28T11:20:55.2526578Z .................................................................................................... 6800/9259
---
2019-10-28T11:24:28.7981967Z 4 LL |         pub use super::foo;
2019-10-28T11:24:28.7982203Z 5    |                 ^^^^^^^^^^
2019-10-28T11:24:28.7982405Z 
2019-10-28T11:24:28.7982658Z 6    |
2019-10-28T11:24:28.7982889Z 7 note: consider marking `foo` as `pub` in the imported module
2019-10-28T11:24:28.7983515Z -   --> $DIR/reexports.rs:10:17
2019-10-28T11:24:28.7984143Z +   --> $DIR/reexports.rs:8:17
2019-10-28T11:24:28.7984726Z 10 LL |         pub use super::foo;
2019-10-28T11:24:28.7985009Z 11    |                 ^^^^^^^^^^
2019-10-28T11:24:28.7985224Z 
2019-10-28T11:24:28.7985462Z 12 
2019-10-28T11:24:28.7985462Z 12 
2019-10-28T11:24:28.7985718Z 13 error[E0603]: module `foo` is private
2019-10-28T11:24:28.7986214Z -   --> $DIR/reexports.rs:35:15
2019-10-28T11:24:28.7986775Z +   --> $DIR/reexports.rs:33:15
2019-10-28T11:24:28.7987115Z 15    |
2019-10-28T11:24:28.7987359Z 16 LL |     use b::a::foo::S;
2019-10-28T11:24:28.7987843Z 
2019-10-28T11:24:28.7988082Z 18 
2019-10-28T11:24:28.7988313Z 19 error[E0603]: module `foo` is private
2019-10-28T11:24:28.7988820Z -   --> $DIR/reexports.rs:36:15
2019-10-28T11:24:28.7988820Z -   --> $DIR/reexports.rs:36:15
2019-10-28T11:24:28.7989365Z +   --> $DIR/reexports.rs:34:15
2019-10-28T11:24:28.7989678Z 21    |
2019-10-28T11:24:28.7989953Z 22 LL |     use b::b::foo::S as T;
2019-10-28T11:24:28.7990400Z 
2019-10-28T11:24:28.7990659Z 24 
2019-10-28T11:24:28.7990659Z 24 
2019-10-28T11:24:28.7991178Z - warning: this glob doesn't reexport anything because no canditate is public enough
2019-10-28T11:24:28.7991731Z -   --> $DIR/reexports.rs:11:17
2019-10-28T11:24:28.7992356Z + warning: glob import doesn't reexport anything because no candidate is public enough
2019-10-28T11:24:28.7995421Z +   --> $DIR/reexports.rs:9:17
2019-10-28T11:24:28.7995636Z 28 LL |         pub use super::*;
2019-10-28T11:24:28.7995679Z 29    |                 ^^^^^^^^
2019-10-28T11:24:28.7995740Z 
2019-10-28T11:24:28.7995766Z 
2019-10-28T11:24:28.7995766Z 
2019-10-28T11:24:28.7995809Z The actual stderr differed from the expected stderr.
2019-10-28T11:24:28.7996199Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/reexports/reexports.stderr
2019-10-28T11:24:28.7996478Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:24:28.7996753Z To only update this specific test, also pass `--test-args imports/reexports.rs`
2019-10-28T11:24:28.7996852Z error: 1 errors occurred comparing output.
2019-10-28T11:24:28.7996895Z status: exit code: 1
2019-10-28T11:24:28.7996895Z status: exit code: 1
2019-10-28T11:24:28.7997543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/reexports.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/reexports" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/reexports/auxiliary" "-A" "unused"
2019-10-28T11:24:28.7998059Z ------------------------------------------
2019-10-28T11:24:28.7998117Z 
2019-10-28T11:24:28.7998354Z ------------------------------------------
2019-10-28T11:24:28.7998401Z stderr:
2019-10-28T11:24:28.7998401Z stderr:
2019-10-28T11:24:28.7998625Z ------------------------------------------
2019-10-28T11:24:28.7998887Z error[E0364]: `foo` is private, and cannot be re-exported
2019-10-28T11:24:28.7999126Z   --> /checkout/src/test/ui/imports/reexports.rs:8:17
2019-10-28T11:24:28.7999177Z    |
2019-10-28T11:24:28.7999439Z LL |         pub use super::foo; //~ ERROR cannot be re-exported
2019-10-28T11:24:28.7999488Z    |                 ^^^^^^^^^^
2019-10-28T11:24:28.7999528Z    |
2019-10-28T11:24:28.7999590Z note: consider marking `foo` as `pub` in the imported module
2019-10-28T11:24:28.7999888Z    |
2019-10-28T11:24:28.8000140Z LL |         pub use super::foo; //~ ERROR cannot be re-exported
2019-10-28T11:24:28.8000191Z    |                 ^^^^^^^^^^
2019-10-28T11:24:28.8000220Z 
2019-10-28T11:24:28.8000220Z 
2019-10-28T11:24:28.8000260Z error[E0603]: module `foo` is private
2019-10-28T11:24:28.8000596Z   --> /checkout/src/test/ui/imports/reexports.rs:33:15
2019-10-28T11:24:28.8000655Z    |
2019-10-28T11:24:28.8000698Z LL |     use b::a::foo::S; //~ ERROR `foo`
2019-10-28T11:24:28.8000789Z 
2019-10-28T11:24:28.8000829Z error[E0603]: module `foo` is private
2019-10-28T11:24:28.8001098Z   --> /checkout/src/test/ui/imports/reexports.rs:34:15
2019-10-28T11:24:28.8001163Z    |
2019-10-28T11:24:28.8001163Z    |
2019-10-28T11:24:28.8001205Z LL |     use b::b::foo::S as T; //~ ERROR `foo`
2019-10-28T11:24:28.8001274Z 
2019-10-28T11:24:28.8001274Z 
2019-10-28T11:24:28.8001548Z warning: glob import doesn't reexport anything because no candidate is public enough
2019-10-28T11:24:28.8001855Z    |
2019-10-28T11:24:28.8001917Z LL |         pub use super::*;
2019-10-28T11:24:28.8001959Z    |                 ^^^^^^^^
2019-10-28T11:24:28.8001998Z    |
---
2019-10-28T11:24:28.8007429Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-28T11:24:28.8007491Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-28T11:24:28.8022344Z 
2019-10-28T11:24:28.8022450Z 
2019-10-28T11:24:28.8023937Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-28T11:24:28.8024355Z 
2019-10-28T11:24:28.8024382Z 
2019-10-28T11:24:28.8030376Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-28T11:24:28.8030481Z Build completed unsuccessfully in 0:55:25
2019-10-28T11:24:28.8030481Z Build completed unsuccessfully in 0:55:25
2019-10-28T11:24:28.8078929Z == clock drift check ==
2019-10-28T11:24:28.8094543Z   local time: Mon Oct 28 11:24:28 UTC 2019
2019-10-28T11:24:29.0752668Z   network time: Mon, 28 Oct 2019 11:24:29 GMT
2019-10-28T11:24:29.0754735Z == end clock drift check ==
2019-10-28T11:24:30.3291334Z 
2019-10-28T11:24:30.3415267Z ##[error]Bash exited with code '1'.
2019-10-28T11:24:30.3450824Z ##[section]Starting: Checkout
2019-10-28T11:24:30.3453386Z ==============================================================================
2019-10-28T11:24:30.3453445Z Task         : Get sources
2019-10-28T11:24:30.3453492Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
