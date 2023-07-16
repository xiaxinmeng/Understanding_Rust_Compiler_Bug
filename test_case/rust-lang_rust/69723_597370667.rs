plain
2020-03-10T21:53:06.7669563Z ========================== Starting Command Output ===========================
2020-03-10T21:53:06.7673995Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b133dd8d-f7c0-4e38-bb9b-64adc52a671b.sh
2020-03-10T21:53:06.7674437Z 
2020-03-10T21:53:06.7679415Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T21:53:06.7700355Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69723/merge to s
2020-03-10T21:53:06.7703995Z Task         : Get sources
2020-03-10T21:53:06.7704324Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T21:53:06.7705136Z Version      : 1.0.0
2020-03-10T21:53:06.7705349Z Author       : Microsoft
---
2020-03-10T21:53:07.7561049Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T21:53:07.7568477Z ##[command]git config gc.auto 0
2020-03-10T21:53:07.7574252Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T21:53:07.7579150Z ##[command]git config --get-all http.proxy
2020-03-10T21:53:07.7587800Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69723/merge:refs/remotes/pull/69723/merge
---
2020-03-10T22:48:37.3448405Z .................................................................................................... 1700/9755
2020-03-10T22:48:41.4011043Z .................................................................................................... 1800/9755
2020-03-10T22:48:51.5613185Z .............................................................i...................................... 1900/9755
2020-03-10T22:48:58.6215070Z .................................................................................................... 2000/9755
2020-03-10T22:49:11.9848829Z ...................................................iiiii............................................ 2100/9755
2020-03-10T22:49:21.7431731Z .................................................................................................... 2300/9755
2020-03-10T22:49:23.8182702Z .................................................................................................... 2400/9755
2020-03-10T22:49:26.8814526Z .................................................................................................... 2500/9755
2020-03-10T22:49:47.0699116Z .................................................................................................... 2600/9755
---
2020-03-10T22:52:14.9164952Z .....................i...............i.............................................................. 5000/9755
2020-03-10T22:52:24.6613692Z .................................................................................................... 5100/9755
2020-03-10T22:52:30.0260069Z ................................................................i................................... 5200/9755
2020-03-10T22:52:36.1427636Z .................................................................................................... 5300/9755
2020-03-10T22:52:45.0424041Z .............................................ii.ii........i...i..................................... 5400/9755
2020-03-10T22:52:53.1321537Z .................................................................................................... 5600/9755
2020-03-10T22:53:02.4880382Z .................................................................................................... 5700/9755
2020-03-10T22:53:09.2773046Z ....................................i............................................................... 5800/9755
2020-03-10T22:53:15.0932226Z .................................................................................................... 5900/9755
2020-03-10T22:53:15.0932226Z .................................................................................................... 5900/9755
2020-03-10T22:53:25.4628512Z .................................................................................................... 6000/9755
2020-03-10T22:53:34.3764124Z .............................ii...i..ii...........i................................................. 6100/9755
2020-03-10T22:53:50.2244257Z .................................................................................................... 6300/9755
2020-03-10T22:53:53.6014159Z .................................................................................................... 6400/9755
2020-03-10T22:53:53.6014159Z .................................................................................................... 6400/9755
2020-03-10T22:53:59.7432297Z ............................................................i..ii................................... 6500/9755
2020-03-10T22:54:22.8859492Z .................................................................................................... 6700/9755
2020-03-10T22:54:26.0946218Z ......................................................i............................................. 6800/9755
2020-03-10T22:54:27.8603848Z .................................................................................................... 6900/9755
2020-03-10T22:54:29.7854660Z .....................................................................................i.............. 7000/9755
---
2020-03-10T22:55:58.9901967Z .................................................................................................... 7700/9755
2020-03-10T22:56:02.9732004Z .................................................................................................... 7800/9755
2020-03-10T22:56:08.5997252Z .................................................................................................... 7900/9755
2020-03-10T22:56:15.4613385Z ...................................i................................................................ 8000/9755
2020-03-10T22:56:24.2417521Z ....................................................................................iiiiiiiiii.i.... 8100/9755
2020-03-10T22:56:39.3786715Z ............................i......i................................................................ 8300/9755
2020-03-10T22:56:44.0361075Z .................................................................................................... 8400/9755
2020-03-10T22:56:54.8351382Z .................................................................................................... 8500/9755
2020-03-10T22:57:06.2521797Z .................................................................................................... 8600/9755
---
2020-03-10T22:59:20.8935710Z  finished in 6.803
2020-03-10T22:59:20.9128921Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-10T22:59:21.1019329Z 
2020-03-10T22:59:21.1019716Z running 179 tests
2020-03-10T22:59:23.8978884Z iiii......i...........ii..iiii....i....i...........i............i..i..................i....i........ 100/179
2020-03-10T22:59:26.1032823Z ....i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-03-10T22:59:26.1035508Z 
2020-03-10T22:59:26.1040262Z  finished in 5.191
2020-03-10T22:59:26.1275543Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-10T22:59:26.2842729Z 
---
2020-03-10T22:59:28.1273468Z  finished in 2.002
2020-03-10T22:59:28.1486375Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-10T22:59:28.3057925Z 
2020-03-10T22:59:28.3058886Z running 9 tests
2020-03-10T22:59:28.3060132Z iiiiiiiii
2020-03-10T22:59:28.3061465Z 
2020-03-10T22:59:28.3064961Z  finished in 0.158
2020-03-10T22:59:28.3257870Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-10T22:59:28.4969806Z 
---
2020-03-10T22:59:47.2245961Z  finished in 18.898
2020-03-10T22:59:47.7498470Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-10T22:59:47.9280433Z 
2020-03-10T22:59:47.9280791Z running 115 tests
2020-03-10T23:00:00.5862557Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-10T23:00:02.0919924Z ...iiii.....ii.
2020-03-10T23:00:02.0921482Z 
2020-03-10T23:00:02.0925468Z  finished in 14.343
2020-03-10T23:00:02.0930720Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-10T23:00:02.0931658Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-03-10T23:11:54.2365265Z 
2020-03-10T23:11:54.2366603Z    Doc-tests core
2020-03-10T23:11:58.7779398Z 
2020-03-10T23:11:58.7780060Z running 2480 tests
2020-03-10T23:12:07.4203470Z ......iiiii......................................................................................... 100/2480
2020-03-10T23:12:16.0404425Z ....................................................................................ii.............. 200/2480
2020-03-10T23:12:35.4135234Z ...................i................................................................................ 400/2480
2020-03-10T23:12:35.4135234Z ...................i................................................................................ 400/2480
2020-03-10T23:12:44.4920147Z ........................................................................i..i..................iiii.. 500/2480
2020-03-10T23:12:59.7374429Z .................................................................................................... 700/2480
2020-03-10T23:13:08.0100276Z .................................................................................................... 800/2480
2020-03-10T23:13:15.9144469Z .................................................................................................... 900/2480
2020-03-10T23:13:23.9560351Z .................................................................................................... 1000/2480
---
2020-03-10T23:16:47.6054274Z 
2020-03-10T23:16:47.6054782Z running 1010 tests
2020-03-10T23:17:03.2758785Z i................................................................................................... 100/1010
2020-03-10T23:17:12.5163495Z .................................................................................................... 200/1010
2020-03-10T23:17:19.1795643Z ..................iii......i......i...i......i...................................................... 300/1010
2020-03-10T23:17:23.8678729Z .................................................................................................... 400/1010
2020-03-10T23:17:30.1905753Z ............................................i..i......................................ii............ 500/1010
2020-03-10T23:17:41.8696428Z .................................................................................................... 700/1010
2020-03-10T23:17:41.8696428Z .................................................................................................... 700/1010
2020-03-10T23:17:48.1444567Z ....................................iiii............................................................ 800/1010
2020-03-10T23:18:01.1314640Z .................................................................................................... 900/1010
2020-03-10T23:18:07.3714192Z ..........................................................iiii...................................... 1000/1010
2020-03-10T23:18:07.7842449Z test result: ok. 990 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-10T23:18:07.7842697Z 
2020-03-10T23:18:07.7953746Z  finished in 153.689
2020-03-10T23:18:07.7968370Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-10T23:33:00.2123652Z     Checking backtrace v0.3.44
2020-03-10T23:33:02.3600627Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-03-10T23:33:02.3601783Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-10T23:33:03.1543969Z  Documenting std v0.0.0 (/checkout/src/libstd)
2020-03-10T23:33:07.4962253Z error: `[Reference]` cannot be resolved, ignoring it.
2020-03-10T23:33:07.4964714Z    --> src/libstd/keyword_docs.rs:932:64
2020-03-10T23:33:07.4966073Z     |
2020-03-10T23:33:07.4967596Z 932 | /// For more information on the `pub` keyword, please see the [Reference] section
2020-03-10T23:33:07.4970954Z     |
2020-03-10T23:33:07.4972069Z note: the lint level is defined here
2020-03-10T23:33:07.4974797Z    --> src/libstd/lib.rs:211:9
2020-03-10T23:33:07.4977204Z     |
2020-03-10T23:33:07.4977204Z     |
2020-03-10T23:33:07.4980696Z 211 | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2020-03-10T23:33:07.4984962Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-03-10T23:33:07.4985578Z 
2020-03-10T23:33:07.4985578Z 
2020-03-10T23:33:07.4986341Z error: `[Reference]` cannot be resolved, ignoring it.
2020-03-10T23:33:07.4987326Z    --> src/libstd/keyword_docs.rs:934:6
2020-03-10T23:33:07.4988106Z     |
2020-03-10T23:33:07.4989348Z 934 | /// [Reference]: ../reference/visibility-and-privacy.html?highlight=pub#visibility-and-privacy
2020-03-10T23:33:07.4991757Z     |
2020-03-10T23:33:07.4992862Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-03-10T23:33:07.4993439Z 
2020-03-10T23:33:07.4993439Z 
2020-03-10T23:33:07.5003241Z error: `[Reference]` cannot be resolved, ignoring it.
2020-03-10T23:33:07.5004266Z    --> src/libstd/keyword_docs.rs:932:64
2020-03-10T23:33:07.5005037Z     |
2020-03-10T23:33:07.5006162Z 932 | /// For more information on the `pub` keyword, please see the [Reference] section
2020-03-10T23:33:07.5008836Z     |
2020-03-10T23:33:07.5009941Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-03-10T23:33:07.5010716Z 
2020-03-10T23:33:07.7794872Z error: aborting due to 3 previous errors
2020-03-10T23:33:07.7794872Z error: aborting due to 3 previous errors
2020-03-10T23:33:07.7795231Z 
2020-03-10T23:33:07.8022972Z error: Could not document `std`.
2020-03-10T23:33:07.8023256Z 
2020-03-10T23:33:07.8023405Z Caused by:
2020-03-10T23:33:07.8030181Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std src/libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.43.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liballoc-8176951eda628efd.rmeta --extern backtrace_rs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-6c3cde2f3fa05ae1.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-3d1332e0a777003d.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-b38d6b4fdd1ab358.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-488a6c8f96495370.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-b52ddaece6955c96.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liblibc-95628d6792815695.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-e4d1532af96285a7.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-e200a5c99f1e2488.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libunwind-972b1d207d4e00af.rmeta` (exit code: 1)
2020-03-10T23:33:07.8041344Z 
2020-03-10T23:33:07.8041344Z 
2020-03-10T23:33:07.8055949Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.43.0" "--index-page" "/checkout/src/doc/index.md"
2020-03-10T23:33:07.8057340Z 
2020-03-10T23:33:07.8057442Z 
2020-03-10T23:33:07.8092058Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-10T23:33:07.8092450Z Build completed unsuccessfully in 1:34:10
2020-03-10T23:33:07.8092450Z Build completed unsuccessfully in 1:34:10
2020-03-10T23:33:07.8127030Z == clock drift check ==
2020-03-10T23:33:07.8150826Z   local time: Tue Mar 10 23:33:07 UTC 2020
2020-03-10T23:33:08.1096806Z   network time: Tue, 10 Mar 2020 23:33:08 GMT
2020-03-10T23:33:08.1097316Z == end clock drift check ==
2020-03-10T23:33:11.1450852Z 
2020-03-10T23:33:11.1533526Z ##[error]Bash exited with code '1'.
2020-03-10T23:33:11.1548038Z ##[section]Finishing: Run build
2020-03-10T23:33:11.1598040Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69723/merge to s
2020-03-10T23:33:11.1602999Z Task         : Get sources
2020-03-10T23:33:11.1603365Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T23:33:11.1603685Z Version      : 1.0.0
2020-03-10T23:33:11.1603915Z Author       : Microsoft
2020-03-10T23:33:11.1603915Z Author       : Microsoft
2020-03-10T23:33:11.1604293Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T23:33:11.1604704Z ==============================================================================
2020-03-10T23:33:11.4995968Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T23:33:11.5045577Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69723/merge to s
2020-03-10T23:33:11.5134789Z Cleaning up task key
2020-03-10T23:33:11.5136087Z Start cleaning up orphan processes.
2020-03-10T23:33:11.5292892Z Terminate orphan process: pid (3793) (python)
2020-03-10T23:33:11.5516995Z ##[section]Finishing: Finalize Job
