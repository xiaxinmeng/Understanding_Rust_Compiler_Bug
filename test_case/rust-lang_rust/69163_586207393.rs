plain
2020-02-14T08:54:41.0084374Z ========================== Starting Command Output ===========================
2020-02-14T08:54:41.0086340Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3f9a4201-5e92-47f0-9663-df4650a9fa6a.sh
2020-02-14T08:54:41.0086386Z 
2020-02-14T08:54:41.0089562Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-14T08:54:41.0096119Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-14T08:54:41.0097787Z Task         : Get sources
2020-02-14T08:54:41.0097823Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-14T08:54:41.0097859Z Version      : 1.0.0
2020-02-14T08:54:41.0097947Z Author       : Microsoft
---
2020-02-14T08:54:41.8443512Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-14T08:54:41.8538480Z ##[command]git config gc.auto 0
2020-02-14T08:54:41.8621041Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-14T08:54:41.8686468Z ##[command]git config --get-all http.proxy
2020-02-14T08:54:41.8835496Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69163/merge:refs/remotes/pull/69163/merge
---
2020-02-14T09:55:33.1315387Z .................................................................................................... 1700/9637
2020-02-14T09:55:38.2364965Z .................................................................................................... 1800/9637
2020-02-14T09:55:50.5917966Z ..............................i..................................................................... 1900/9637
2020-02-14T09:55:58.4249164Z .................................................................................................... 2000/9637
2020-02-14T09:56:13.0629028Z ....................iiiii........................................................................... 2100/9637
2020-02-14T09:56:22.9457576Z .................................................................................................... 2300/9637
2020-02-14T09:56:25.4331592Z .................................................................................................... 2400/9637
2020-02-14T09:56:30.5920655Z .................................................................................................... 2500/9637
2020-02-14T09:56:52.0873246Z .................................................................................................... 2600/9637
---
2020-02-14T09:59:39.3647141Z ........................................................................i...............i........... 4900/9637
2020-02-14T09:59:47.4413921Z .................................................................................................... 5000/9637
2020-02-14T09:59:55.6963212Z .................................................................................................... 5100/9637
2020-02-14T10:00:00.7610152Z ..............i..................................................................................... 5200/9637
2020-02-14T10:00:12.0232899Z ........................................................................................ii.ii....... 5300/9637
2020-02-14T10:00:16.1680905Z .i...i.............................................................................................. 5400/9637
2020-02-14T10:00:26.7976955Z .................................................................................................... 5600/9637
2020-02-14T10:00:37.5018345Z .............................................................................i...................... 5700/9637
2020-02-14T10:00:45.7717090Z .................................................................................................... 5800/9637
2020-02-14T10:00:52.5789121Z ...........................................................................i........................ 5900/9637
2020-02-14T10:00:52.5789121Z ...........................................................................i........................ 5900/9637
2020-02-14T10:01:03.5701600Z .....................................................................ii...i..ii...........i......... 6000/9637
2020-02-14T10:01:26.1654929Z .................................................................................................... 6200/9637
2020-02-14T10:01:34.3871300Z .................................................................................................... 6300/9637
2020-02-14T10:01:43.0053411Z .................................................................................................i.. 6400/9637
2020-02-14T10:02:00.7238671Z ii.................................................................................................. 6500/9637
---
2020-02-14T10:04:14.0755338Z .................................................................................................... 7600/9637
2020-02-14T10:04:18.7729445Z .................................................................................................... 7700/9637
2020-02-14T10:04:25.3264139Z .................................................................................................... 7800/9637
2020-02-14T10:04:33.7803528Z .................................................................................................... 7900/9637
2020-02-14T10:04:43.0840064Z ............................................................................iiiiiii.i............... 8000/9637
2020-02-14T10:05:00.0259143Z ................i......i............................................................................ 8200/9637
2020-02-14T10:05:05.8546496Z .................................................................................................... 8300/9637
2020-02-14T10:05:19.5240915Z .................................................................................................... 8400/9637
2020-02-14T10:05:30.0240986Z .................................................................................................... 8500/9637
---
2020-02-14T10:08:06.9968326Z  finished in 7.759
2020-02-14T10:08:07.0152356Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T10:08:07.2134934Z 
2020-02-14T10:08:07.2135862Z running 178 tests
2020-02-14T10:08:10.3365932Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-14T10:08:12.8042709Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-14T10:08:12.8053585Z 
2020-02-14T10:08:12.8058111Z  finished in 5.790
2020-02-14T10:08:12.8283710Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T10:08:13.0120397Z 
---
2020-02-14T10:08:15.0651288Z  finished in 2.237
2020-02-14T10:08:15.0822344Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T10:08:15.2304506Z 
2020-02-14T10:08:15.2305674Z running 9 tests
2020-02-14T10:08:15.2306669Z iiiiiiiii
2020-02-14T10:08:15.2307484Z 
2020-02-14T10:08:15.2307729Z  finished in 0.148
2020-02-14T10:08:15.2502283Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T10:08:15.4959453Z 
---
2020-02-14T10:08:36.3458412Z  finished in 21.095
2020-02-14T10:08:36.3724230Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T10:08:36.5600563Z 
2020-02-14T10:08:36.5640514Z running 116 tests
2020-02-14T10:08:50.9300334Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-14T10:08:53.4768740Z ....iiii.....ii.
2020-02-14T10:08:53.4770062Z 
2020-02-14T10:08:53.4770641Z  finished in 16.476
2020-02-14T10:08:53.4776163Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T10:08:53.4786320Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-14T10:22:46.7961945Z 
2020-02-14T10:22:46.7976123Z    Doc-tests core
2020-02-14T10:22:51.7880334Z 
2020-02-14T10:22:51.7881305Z running 2471 tests
2020-02-14T10:23:01.0426851Z ......iiiii......................................................................................... 100/2471
2020-02-14T10:23:10.3894068Z ..................................................................................ii................ 200/2471
2020-02-14T10:23:32.0087776Z .................i.................................................................................. 400/2471
2020-02-14T10:23:32.0087776Z .................i.................................................................................. 400/2471
2020-02-14T10:23:41.8510105Z ......................................................................i..i..................iiii.... 500/2471
2020-02-14T10:23:58.9923794Z .................................................................................................... 700/2471
2020-02-14T10:24:07.8628629Z .................................................................................................... 800/2471
2020-02-14T10:24:16.6719133Z .................................................................................................... 900/2471
2020-02-14T10:24:25.4786859Z .................................................................................................... 1000/2471
---
2020-02-14T10:28:08.7354735Z 
2020-02-14T10:28:08.7355026Z running 1009 tests
2020-02-14T10:28:27.4182552Z i................................................................................................... 100/1009
2020-02-14T10:28:36.7308525Z .................................................................................................... 200/1009
2020-02-14T10:28:43.9411891Z ..................iii......i......i...i......i...................................................... 300/1009
2020-02-14T10:28:48.9511851Z .................................................................................................... 400/1009
2020-02-14T10:28:55.6617997Z ............................................i..i.....................................ii............. 500/1009
2020-02-14T10:29:08.1072869Z .................................................................................................... 700/1009
2020-02-14T10:29:08.1072869Z .................................................................................................... 700/1009
2020-02-14T10:29:14.7281697Z ...................................iiii............................................................. 800/1009
2020-02-14T10:29:29.1034033Z .................................................................................................... 900/1009
2020-02-14T10:29:36.0097548Z .........................................................iiii....................................... 1000/1009
2020-02-14T10:29:36.3793261Z test result: ok. 989 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-02-14T10:29:36.3793425Z 
2020-02-14T10:29:36.3933287Z  finished in 170.045
2020-02-14T10:29:36.3947081Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-02-14T10:48:22.7146180Z     Checking backtrace v0.3.44
2020-02-14T10:48:24.9340846Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-02-14T10:48:24.9371161Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-02-14T10:48:25.7822572Z  Documenting std v0.0.0 (/checkout/src/libstd)
2020-02-14T10:48:30.9887362Z error: `[source]` cannot be resolved, ignoring it.
2020-02-14T10:48:30.9891536Z    --> src/libstd/error.rs:219:30
2020-02-14T10:48:30.9892792Z 219 |     /// recursively calling [`source`].
2020-02-14T10:48:30.9893483Z     |                              ^^^^^^^^ cannot be resolved, ignoring
2020-02-14T10:48:30.9894295Z     |
2020-02-14T10:48:30.9894821Z note: the lint level is defined here
2020-02-14T10:48:30.9894821Z note: the lint level is defined here
2020-02-14T10:48:30.9895365Z    --> src/libstd/lib.rs:211:9
2020-02-14T10:48:30.9895855Z     |
2020-02-14T10:48:30.9897074Z 211 | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2020-02-14T10:48:30.9898257Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-14T10:48:30.9898432Z 
2020-02-14T10:48:30.9898432Z 
2020-02-14T10:48:30.9905662Z error: `[source]` cannot be resolved, ignoring it.
2020-02-14T10:48:30.9906207Z    --> src/libstd/error.rs:219:30
2020-02-14T10:48:30.9907056Z 219 |     /// recursively calling [`source`].
2020-02-14T10:48:30.9907556Z     |                              ^^^^^^^^ cannot be resolved, ignoring
2020-02-14T10:48:30.9907933Z     |
2020-02-14T10:48:30.9908442Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-14T10:48:30.9908442Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-14T10:48:30.9908609Z 
2020-02-14T10:48:31.0404890Z error: `[source]` cannot be resolved, ignoring it.
2020-02-14T10:48:31.0406046Z    --> src/libstd/error.rs:229:30
2020-02-14T10:48:31.0407183Z 229 |     /// recursively calling [`source`].
2020-02-14T10:48:31.0407669Z     |                              ^^^^^^^^ cannot be resolved, ignoring
2020-02-14T10:48:31.0408032Z     |
2020-02-14T10:48:31.0408498Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-14T10:48:31.0408498Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-14T10:48:31.0408654Z 
2020-02-14T10:48:31.3094005Z error: aborting due to 3 previous errors
2020-02-14T10:48:31.3094718Z 
2020-02-14T10:48:31.3307461Z error: Could not document `std`.
2020-02-14T10:48:31.3308011Z 
2020-02-14T10:48:31.3308244Z Caused by:
2020-02-14T10:48:31.3311777Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std src/libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.43.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liballoc-f2c2ee531bd8be59.rmeta --extern backtrace_rs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-512bbd4bd20314cc.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-647ef3970fb37342.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-ae5970a4fd29f03e.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-44b33b5768502e3e.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-2b45fd7c7cead493.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liblibc-430f3e5776f8af9c.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-cf44ab04c1a76b90.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-d69ab35c94b5f557.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libunwind-881400b15301aa72.rmeta` (exit code: 1)
2020-02-14T10:48:31.3334214Z 
2020-02-14T10:48:31.3334214Z 
2020-02-14T10:48:31.3337589Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.43.0" "--index-page" "/checkout/src/doc/index.md"
2020-02-14T10:48:31.3338384Z 
2020-02-14T10:48:31.3338538Z 
2020-02-14T10:48:31.3338781Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-14T10:48:31.3338952Z Build completed unsuccessfully in 1:47:43
2020-02-14T10:48:31.3338952Z Build completed unsuccessfully in 1:47:43
2020-02-14T10:48:31.3384010Z == clock drift check ==
2020-02-14T10:48:31.3406963Z   local time: Fri Feb 14 10:48:31 UTC 2020
2020-02-14T10:48:31.4962417Z   network time: Fri, 14 Feb 2020 10:48:31 GMT
2020-02-14T10:48:31.4968794Z == end clock drift check ==
2020-02-14T10:48:34.2584289Z 
2020-02-14T10:48:34.2707423Z ##[error]Bash exited with code '1'.
2020-02-14T10:48:34.2721797Z ##[section]Finishing: Run build
2020-02-14T10:48:34.2744634Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-14T10:48:34.2746974Z Task         : Get sources
2020-02-14T10:48:34.2747049Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-14T10:48:34.2747124Z Version      : 1.0.0
2020-02-14T10:48:34.2747172Z Author       : Microsoft
2020-02-14T10:48:34.2747172Z Author       : Microsoft
2020-02-14T10:48:34.2747246Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-14T10:48:34.2747303Z ==============================================================================
2020-02-14T10:48:34.7484353Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-14T10:48:34.7523233Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-14T10:48:34.7629892Z Cleaning up task key
2020-02-14T10:48:34.7630646Z Start cleaning up orphan processes.
2020-02-14T10:48:34.7731699Z Terminate orphan process: pid (3889) (python)
2020-02-14T10:48:34.7995710Z ##[section]Finishing: Finalize Job
