plain
2020-03-28T15:45:54.2958349Z ========================== Starting Command Output ===========================
2020-03-28T15:45:54.2961704Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a5906e84-27bd-4590-a331-76e566dc2a15.sh
2020-03-28T15:45:54.2961987Z 
2020-03-28T15:45:54.2967006Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-28T15:45:54.2987837Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70501/merge to s
2020-03-28T15:45:54.2991784Z Task         : Get sources
2020-03-28T15:45:54.2992097Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-28T15:45:54.2992415Z Version      : 1.0.0
2020-03-28T15:45:54.2992620Z Author       : Microsoft
---
2020-03-28T15:45:55.2853711Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-28T15:45:55.2860808Z ##[command]git config gc.auto 0
2020-03-28T15:45:55.2864537Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-28T15:45:55.2867855Z ##[command]git config --get-all http.proxy
2020-03-28T15:45:55.2874172Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70501/merge:refs/remotes/pull/70501/merge
---
2020-03-28T15:54:12.6240823Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-28T15:54:14.1436583Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-28T15:54:15.7563061Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-28T15:54:16.4733438Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-28T15:54:25.5478090Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-28T15:54:27.6822426Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-28T15:54:32.0757383Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-28T15:54:36.2274027Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-28T15:55:06.7529806Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-28T16:16:31.6034001Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-28T16:16:33.4137083Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-28T16:16:35.5554110Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-28T16:16:35.9944976Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-28T16:16:47.5274322Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-28T16:16:49.6932393Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-28T16:16:55.3985553Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-28T16:17:00.1514475Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-28T16:17:39.3845305Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-28T16:42:31.9357659Z .................................................................................................... 1700/9855
2020-03-28T16:42:35.8908658Z .................................................................................................... 1800/9855
2020-03-28T16:42:45.8591028Z .........................................................................................i....F..... 1900/9855
2020-03-28T16:42:52.8933519Z .................................................................................................... 2000/9855
2020-03-28T16:42:59.6018492Z ...............................................................................iiiii................ 2100/9855
2020-03-28T16:43:20.9931277Z .................................................................................................... 2300/9855
2020-03-28T16:43:23.3637045Z .................................................................................................... 2400/9855
2020-03-28T16:43:25.9306332Z .................................................................................................... 2500/9855
2020-03-28T16:43:35.4996495Z .................................................................................................... 2600/9855
---
2020-03-28T16:46:27.6133771Z .....................................................i...............i.............................. 5000/9855
2020-03-28T16:46:35.4817892Z .................................................................................................... 5100/9855
2020-03-28T16:46:42.5057255Z ..................................................................................................i. 5200/9855
2020-03-28T16:46:47.3615587Z .................................................................................................... 5300/9855
2020-03-28T16:46:58.1171950Z ....................................................................................ii.ii........i.. 5400/9855
2020-03-28T16:47:02.0445500Z .i.................................................................................................. 5500/9855
2020-03-28T16:47:11.7863022Z .............................i...................................................................... 5700/9855
2020-03-28T16:47:21.8718386Z ...............................................ii....................................i.............. 5800/9855
2020-03-28T16:47:29.8416347Z .................................................................................................... 5900/9855
2020-03-28T16:47:35.1918714Z .................................................................................................... 6000/9855
2020-03-28T16:47:35.1918714Z .................................................................................................... 6000/9855
2020-03-28T16:47:44.8431183Z ...............................................................................ii...i..ii........... 6100/9855
2020-03-28T16:47:57.3128531Z i................................................................................................... 6200/9855
2020-03-28T16:48:13.1027839Z .................................................................................................... 6400/9855
2020-03-28T16:48:19.0973502Z .................................................................................................... 6500/9855
2020-03-28T16:48:19.0973502Z .................................................................................................... 6500/9855
2020-03-28T16:48:31.5954491Z .........i..ii...................................................................................... 6600/9855
2020-03-28T16:48:51.9550943Z .................................................................................................... 6800/9855
2020-03-28T16:48:54.1408885Z .........i.......................................................................................... 6900/9855
2020-03-28T16:48:56.2891454Z .................................................................................................... 7000/9855
2020-03-28T16:48:58.5546533Z ..............................................i..................................................... 7100/9855
---
2020-03-28T16:50:37.9032898Z .................................................................................................... 7800/9855
2020-03-28T16:50:43.0423942Z .................................................................................................... 7900/9855
2020-03-28T16:50:49.3898787Z .................................................................................................... 8000/9855
2020-03-28T16:50:57.6127238Z ......i............................................................................................. 8100/9855
2020-03-28T16:51:06.1127253Z .......................................................iiiiiiiiii.i................................. 8200/9855
2020-03-28T16:51:14.5800276Z ...................................................................................................i 8300/9855
2020-03-28T16:51:25.6363516Z .................................................................................................... 8500/9855
2020-03-28T16:51:38.5515182Z .................................................................................................... 8600/9855
2020-03-28T16:51:48.3101583Z .................................................................................................... 8700/9855
2020-03-28T16:51:53.7071368Z .................................................................................................... 8800/9855
---
2020-03-28T16:53:43.3641856Z .......................................................
2020-03-28T16:53:43.3644370Z failures:
2020-03-28T16:53:43.3644828Z 
2020-03-28T16:53:43.3645777Z ---- [ui] ui/crt-static-on-works.rs stdout ----
2020-03-28T16:53:43.3647177Z thread '[ui] ui/crt-static-on-works.rs' panicked at 'failed to exec `"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crt-static-on-works/a"`: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/runtest.rs:1824:25
2020-03-28T16:53:43.3648662Z 
2020-03-28T16:53:43.3648789Z 
2020-03-28T16:53:43.3648963Z failures:
2020-03-28T16:53:43.3649494Z     [ui] ui/crt-static-on-works.rs
2020-03-28T16:53:43.3649494Z     [ui] ui/crt-static-on-works.rs
2020-03-28T16:53:43.3650985Z 
2020-03-28T16:53:43.3652108Z test result: FAILED. 9794 passed; 1 failed; 60 ignored; 0 measured; 0 filtered out
2020-03-28T16:53:43.3652603Z 
2020-03-28T16:53:43.3709348Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-28T16:53:43.3729660Z 
2020-03-28T16:53:43.3729832Z 
2020-03-28T16:53:43.3733559Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-28T16:53:43.3736009Z 
2020-03-28T16:53:43.3736117Z 
2020-03-28T16:53:43.3740338Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-28T16:53:43.3740916Z Build completed unsuccessfully in 1:02:59
2020-03-28T16:53:43.3740916Z Build completed unsuccessfully in 1:02:59
2020-03-28T16:53:43.3785158Z == clock drift check ==
2020-03-28T16:53:43.3807637Z   local time: Sat Mar 28 16:53:43 UTC 2020
2020-03-28T16:53:43.5425761Z   network time: Sat, 28 Mar 2020 16:53:43 GMT
2020-03-28T16:53:43.5430201Z == end clock drift check ==
2020-03-28T16:53:43.9767712Z 
2020-03-28T16:53:43.9852413Z ##[error]Bash exited with code '1'.
2020-03-28T16:53:43.9867129Z ##[section]Finishing: Run build
2020-03-28T16:53:43.9932448Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70501/merge to s
2020-03-28T16:53:43.9937699Z Task         : Get sources
2020-03-28T16:53:43.9938046Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-28T16:53:43.9938384Z Version      : 1.0.0
2020-03-28T16:53:43.9938614Z Author       : Microsoft
2020-03-28T16:53:43.9938614Z Author       : Microsoft
2020-03-28T16:53:43.9938966Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-28T16:53:43.9939387Z ==============================================================================
2020-03-28T16:53:44.3205504Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-28T16:53:44.3253741Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70501/merge to s
2020-03-28T16:53:44.3345827Z Cleaning up task key
2020-03-28T16:53:44.3347221Z Start cleaning up orphan processes.
2020-03-28T16:53:44.3524311Z Terminate orphan process: pid (4899) (python)
2020-03-28T16:53:44.3688151Z ##[section]Finishing: Finalize Job
