plain
2020-04-23T21:26:16.3995213Z ========================== Starting Command Output ===========================
2020-04-23T21:26:16.4000128Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cb5eb4bd-01b0-4f97-aa41-bdc92a52121e.sh
2020-04-23T21:26:16.4000653Z 
2020-04-23T21:26:16.4004610Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T21:26:16.4022916Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71481/merge to s
2020-04-23T21:26:16.4025569Z Task         : Get sources
2020-04-23T21:26:16.4025769Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T21:26:16.4025963Z Version      : 1.0.0
2020-04-23T21:26:16.4026138Z Author       : Microsoft
---
2020-04-23T21:26:17.4010222Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T21:26:17.4014716Z ##[command]git config gc.auto 0
2020-04-23T21:26:17.4017964Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T21:26:17.4021299Z ##[command]git config --get-all http.proxy
2020-04-23T21:26:17.4027946Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71481/merge:refs/remotes/pull/71481/merge
---
2020-04-23T21:28:39.4467407Z  ---> 318032b5f0e2
2020-04-23T21:28:39.4468093Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T21:28:39.4468825Z  ---> Using cache
2020-04-23T21:28:39.4471573Z  ---> d44a858fd1ce
2020-04-23T21:28:39.4472380Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T21:28:39.4475528Z  ---> 58b910f50f5a
2020-04-23T21:28:39.4475707Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T21:28:39.4475993Z  ---> Using cache
2020-04-23T21:28:39.4478020Z  ---> ee7702aadba1
---
2020-04-23T21:28:39.4851649Z Looks like docker image is the same as before, not uploading
2020-04-23T21:28:46.5952832Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T21:28:46.6224872Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T21:28:46.6254358Z == clock drift check ==
2020-04-23T21:28:46.6262761Z   local time: Thu Apr 23 21:28:46 UTC 2020
2020-04-23T21:28:46.9183073Z   network time: Thu, 23 Apr 2020 21:28:46 GMT
2020-04-23T21:28:46.9207841Z Starting sccache server...
2020-04-23T21:28:46.9936069Z configure: processing command line
2020-04-23T21:28:46.9936716Z configure: 
2020-04-23T21:28:46.9937741Z configure: rust.dist-src        := False
---
2020-04-23T21:33:51.4535419Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T21:33:52.9513927Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T21:33:54.5259849Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T21:33:56.6310200Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T21:34:04.3365749Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T21:34:07.8499027Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T21:34:12.0261999Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T21:34:16.0610679Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T21:34:24.1665471Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T21:56:48.2445140Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T21:56:49.9399277Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T21:56:51.8318583Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T21:56:52.9279006Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T21:57:02.7244121Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T21:57:06.1060663Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T21:57:10.9556010Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T21:57:15.4081757Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T21:57:24.9134148Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T22:19:30.2998036Z .................................................................................................... 1700/9917
2020-04-23T22:19:34.2571243Z .................................................................................................... 1800/9917
2020-04-23T22:19:41.9465230Z .................................................................................................... 1900/9917
2020-04-23T22:19:49.4298983Z .....i.............................................................................................. 2000/9917
2020-04-23T22:19:55.3521345Z ...............................................................................................iiiii 2100/9917
2020-04-23T22:20:14.0336603Z .................................................................................................... 2300/9917
2020-04-23T22:20:16.0578341Z .................................................................................................... 2400/9917
2020-04-23T22:20:18.1956327Z .................................................................................................... 2500/9917
2020-04-23T22:20:23.4564113Z .................................................................................................... 2600/9917
---
2020-04-23T22:23:07.0844238Z .................................................................................................... 5100/9917
2020-04-23T22:23:13.5885989Z .................................................................................................... 5200/9917
2020-04-23T22:23:17.6277290Z ..................i................................................................................. 5300/9917
2020-04-23T22:23:26.1513107Z ........i........................................................................................... 5400/9917
2020-04-23T22:23:30.8629759Z ........ii.ii........i...i.......................................................................... 5500/9917
2020-04-23T22:23:37.4108174Z .......................................................i............................................ 5700/9917
2020-04-23T22:23:45.1380427Z ........................................................................................ii.......... 5800/9917
2020-04-23T22:23:51.0655900Z ...........................i........................................................................ 5900/9917
2020-04-23T22:23:55.8578333Z .................................................................................................... 6000/9917
2020-04-23T22:23:55.8578333Z .................................................................................................... 6000/9917
2020-04-23T22:24:04.8915007Z .................................................................................................... 6100/9917
2020-04-23T22:24:13.3672276Z .....................ii...i..ii...........i......................................................... 6200/9917
2020-04-23T22:24:27.3431314Z .................................................................................................... 6400/9917
2020-04-23T22:24:33.1667866Z .................................................................................................... 6500/9917
2020-04-23T22:24:33.1667866Z .................................................................................................... 6500/9917
2020-04-23T22:24:40.8870682Z ...................................................i..ii............................................ 6600/9917
2020-04-23T22:25:01.6771015Z .................................................................................................... 6800/9917
2020-04-23T22:25:03.6744874Z ....................................................i............................................... 6900/9917
2020-04-23T22:25:05.3180080Z .................................................................................................... 7000/9917
2020-04-23T22:25:07.0145939Z ............................................................................................i....... 7100/9917
---
2020-04-23T22:26:32.1625208Z .................................................................................................... 7900/9917
2020-04-23T22:26:36.9681430Z .................................................................................................... 8000/9917
2020-04-23T22:26:42.5585446Z ...........................................................i........................................ 8100/9917
2020-04-23T22:26:51.2001607Z .................................................................................................... 8200/9917
2020-04-23T22:26:56.3228411Z ........iiiiii.iiiii.i.............................................................................. 8300/9917
2020-04-23T22:27:08.8931569Z .................................................................................................... 8500/9917
2020-04-23T22:27:15.8599577Z .............................................................F...................................... 8600/9917
2020-04-23T22:27:28.8357976Z .................................................................................................... 8700/9917
2020-04-23T22:27:35.1481371Z .................................................................................................... 8800/9917
---
2020-04-23T22:29:16.1013071Z 
2020-04-23T22:29:16.1013686Z ---- [ui] ui/stability-attribute/stability-attribute-sanity.rs stdout ----
2020-04-23T22:29:16.1013892Z diff of stderr:
2020-04-23T22:29:16.1013988Z 
2020-04-23T22:29:16.1014152Z 94 LL | #[rustc_const_unstable(feature = "d", issue = "none")]
2020-04-23T22:29:16.1014585Z 96 
2020-04-23T22:29:16.1014915Z - error: Invalid stability or deprecation version found
2020-04-23T22:29:16.1015155Z + error: invalid stability or deprecation version found
2020-04-23T22:29:16.1015719Z 98   --> $DIR/stability-attribute-sanity.rs:65:1
2020-04-23T22:29:16.1015719Z 98   --> $DIR/stability-attribute-sanity.rs:65:1
2020-04-23T22:29:16.1015895Z 99    |
2020-04-23T22:29:16.1016065Z 100 LL | pub const fn multiple4() { }
2020-04-23T22:29:16.1016186Z 
2020-04-23T22:29:16.1016261Z 
2020-04-23T22:29:16.1016419Z The actual stderr differed from the expected stderr.
2020-04-23T22:29:16.1017011Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity/stability-attribute-sanity.stderr
2020-04-23T22:29:16.1017533Z To update references, rerun the tests and pass the `--bless` flag
2020-04-23T22:29:16.1018033Z To only update this specific test, also pass `--test-args stability-attribute/stability-attribute-sanity.rs`
2020-04-23T22:29:16.1018408Z error: 1 errors occurred comparing output.
2020-04-23T22:29:16.1018753Z status: exit code: 1
2020-04-23T22:29:16.1018753Z status: exit code: 1
2020-04-23T22:29:16.1020873Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity/auxiliary"
2020-04-23T22:29:16.1022412Z ------------------------------------------
2020-04-23T22:29:16.1022721Z 
2020-04-23T22:29:16.1023098Z ------------------------------------------
2020-04-23T22:29:16.1023283Z stderr:
2020-04-23T22:29:16.1023283Z stderr:
2020-04-23T22:29:16.1023599Z ------------------------------------------
2020-04-23T22:29:16.1023940Z error[E0541]: unknown meta item 'reason'
2020-04-23T22:29:16.1024756Z    |
2020-04-23T22:29:16.1024756Z    |
2020-04-23T22:29:16.1025140Z LL |     #[stable(feature = "a", since = "b", reason)] //~ ERROR unknown meta item 'reason' [E0541]
2020-04-23T22:29:16.1025463Z    |                                          ^^^^^^ expected one of `since`, `note`
2020-04-23T22:29:16.1025760Z error[E0539]: incorrect meta item
2020-04-23T22:29:16.1026189Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:11:29
2020-04-23T22:29:16.1026571Z    |
2020-04-23T22:29:16.1026571Z    |
2020-04-23T22:29:16.1026768Z LL |     #[stable(feature = "a", since)] //~ ERROR incorrect meta item [E0539]
2020-04-23T22:29:16.1027141Z 
2020-04-23T22:29:16.1027278Z error[E0539]: incorrect meta item
2020-04-23T22:29:16.1027707Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:14:14
2020-04-23T22:29:16.1027936Z    |
2020-04-23T22:29:16.1027936Z    |
2020-04-23T22:29:16.1028131Z LL |     #[stable(feature, since = "a")] //~ ERROR incorrect meta item [E0539]
2020-04-23T22:29:16.1028458Z 
2020-04-23T22:29:16.1028612Z error[E0539]: incorrect meta item
2020-04-23T22:29:16.1029037Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:17:29
2020-04-23T22:29:16.1029251Z    |
2020-04-23T22:29:16.1029251Z    |
2020-04-23T22:29:16.1029467Z LL |     #[stable(feature = "a", since(b))] //~ ERROR incorrect meta item [E0539]
2020-04-23T22:29:16.1029992Z 
2020-04-23T22:29:16.1031371Z error[E0539]: incorrect meta item
2020-04-23T22:29:16.1032557Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:20:14
2020-04-23T22:29:16.1032773Z    |
2020-04-23T22:29:16.1032773Z    |
2020-04-23T22:29:16.1032982Z LL |     #[stable(feature(b), since = "a")] //~ ERROR incorrect meta item [E0539]
2020-04-23T22:29:16.1033299Z 
2020-04-23T22:29:16.1033611Z error[E0546]: missing 'feature'
2020-04-23T22:29:16.1034028Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:25:5
2020-04-23T22:29:16.1034236Z    |
2020-04-23T22:29:16.1034236Z    |
2020-04-23T22:29:16.1034572Z LL |     #[unstable(issue = "none")] //~ ERROR missing 'feature' [E0546]
2020-04-23T22:29:16.1034918Z 
2020-04-23T22:29:16.1035170Z error[E0547]: missing 'issue'
2020-04-23T22:29:16.1035565Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:28:5
2020-04-23T22:29:16.1035783Z    |
2020-04-23T22:29:16.1035783Z    |
2020-04-23T22:29:16.1036102Z LL |     #[unstable(feature = "b")] //~ ERROR missing 'issue' [E0547]
2020-04-23T22:29:16.1036448Z 
2020-04-23T22:29:16.1036703Z error[E0546]: missing 'feature'
2020-04-23T22:29:16.1037100Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:31:5
2020-04-23T22:29:16.1037315Z    |
2020-04-23T22:29:16.1037315Z    |
2020-04-23T22:29:16.1037826Z LL |     #[stable(since = "a")] //~ ERROR missing 'feature' [E0546]
2020-04-23T22:29:16.1038271Z 
2020-04-23T22:29:16.1038585Z error[E0542]: missing 'since'
2020-04-23T22:29:16.1039002Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:36:5
2020-04-23T22:29:16.1039215Z    |
2020-04-23T22:29:16.1039215Z    |
2020-04-23T22:29:16.1039567Z LL |     #[stable(feature = "a")] //~ ERROR missing 'since' [E0542]
2020-04-23T22:29:16.1042022Z 
2020-04-23T22:29:16.1042590Z error[E0542]: missing 'since'
2020-04-23T22:29:16.1043218Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:40:5
2020-04-23T22:29:16.1043424Z    |
2020-04-23T22:29:16.1043424Z    |
2020-04-23T22:29:16.1043904Z LL |     #[rustc_deprecated(reason = "a")] //~ ERROR missing 'since' [E0542]
2020-04-23T22:29:16.1044271Z 
2020-04-23T22:29:16.1044555Z error[E0543]: missing 'reason'
2020-04-23T22:29:16.1044972Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:44:5
2020-04-23T22:29:16.1045174Z    |
2020-04-23T22:29:16.1045174Z    |
2020-04-23T22:29:16.1045517Z LL |     #[rustc_deprecated(since = "a")] //~ ERROR missing 'reason' [E0543]
2020-04-23T22:29:16.1045877Z 
2020-04-23T22:29:16.1046015Z error[E0544]: multiple stability levels
2020-04-23T22:29:16.1046420Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:49:1
2020-04-23T22:29:16.1046637Z    |
2020-04-23T22:29:16.1046637Z    |
2020-04-23T22:29:16.1046831Z LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]
2020-04-23T22:29:16.1047205Z 
2020-04-23T22:29:16.1047343Z error[E0544]: multiple stability levels
2020-04-23T22:29:16.1047951Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:53:1
2020-04-23T22:29:16.1048173Z    |
2020-04-23T22:29:16.1048173Z    |
2020-04-23T22:29:16.1048547Z LL | #[unstable(feature = "b", issue = "none")] //~ ERROR multiple stability levels [E0544]
2020-04-23T22:29:16.1048949Z 
2020-04-23T22:29:16.1049283Z error[E0544]: multiple stability levels
2020-04-23T22:29:16.1050500Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:57:1
2020-04-23T22:29:16.1050784Z    |
2020-04-23T22:29:16.1050784Z    |
2020-04-23T22:29:16.1051069Z LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]
2020-04-23T22:29:16.1051556Z 
2020-04-23T22:29:16.1051778Z error[E0540]: multiple rustc_deprecated attributes
2020-04-23T22:29:16.1052386Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:65:1
2020-04-23T22:29:16.1052663Z    |
2020-04-23T22:29:16.1052663Z    |
2020-04-23T22:29:16.1053463Z LL | pub const fn multiple4() { } //~ ERROR multiple rustc_deprecated attributes [E0540]
2020-04-23T22:29:16.1053939Z 
2020-04-23T22:29:16.1054083Z error[E0544]: multiple stability levels
2020-04-23T22:29:16.1054595Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:64:1
2020-04-23T22:29:16.1054808Z    |
2020-04-23T22:29:16.1054808Z    |
2020-04-23T22:29:16.1055195Z LL | #[rustc_const_unstable(feature = "d", issue = "none")] //~ ERROR multiple stability levels
2020-04-23T22:29:16.1055617Z 
2020-04-23T22:29:16.1055768Z error: invalid stability or deprecation version found
2020-04-23T22:29:16.1056208Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:65:1
2020-04-23T22:29:16.1056409Z    |
2020-04-23T22:29:16.1056409Z    |
2020-04-23T22:29:16.1056608Z LL | pub const fn multiple4() { } //~ ERROR multiple rustc_deprecated attributes [E0540]
2020-04-23T22:29:16.1056960Z 
2020-04-23T22:29:16.1057162Z error[E0549]: rustc_deprecated attribute must be paired with either stable or unstable attribute
2020-04-23T22:29:16.1057635Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:69:1
2020-04-23T22:29:16.1057976Z    |
---
2020-04-23T22:29:16.1062458Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-23T22:29:16.1063319Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-23T22:29:16.1063490Z 
2020-04-23T22:29:16.1063565Z 
2020-04-23T22:29:16.1066503Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-23T22:29:16.1069181Z 
2020-04-23T22:29:16.1069271Z 
2020-04-23T22:29:16.1070048Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-23T22:29:16.1070673Z Build completed unsuccessfully in 0:58:58
2020-04-23T22:29:16.1070673Z Build completed unsuccessfully in 0:58:58
2020-04-23T22:29:16.1106843Z == clock drift check ==
2020-04-23T22:29:16.1125401Z   local time: Thu Apr 23 22:29:16 UTC 2020
2020-04-23T22:29:16.4166854Z   network time: Thu, 23 Apr 2020 22:29:16 GMT
2020-04-23T22:29:16.9228732Z 
2020-04-23T22:29:16.9228732Z 
2020-04-23T22:29:16.9295199Z ##[error]Bash exited with code '1'.
2020-04-23T22:29:16.9308609Z ##[section]Finishing: Run build
2020-04-23T22:29:16.9361159Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71481/merge to s
2020-04-23T22:29:16.9367088Z Task         : Get sources
2020-04-23T22:29:16.9367336Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T22:29:16.9367557Z Version      : 1.0.0
2020-04-23T22:29:16.9367733Z Author       : Microsoft
2020-04-23T22:29:16.9367733Z Author       : Microsoft
2020-04-23T22:29:16.9367982Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T22:29:16.9368265Z ==============================================================================
2020-04-23T22:29:17.2485134Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T22:29:17.2530274Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71481/merge to s
2020-04-23T22:29:17.2614868Z Cleaning up task key
2020-04-23T22:29:17.2616372Z Start cleaning up orphan processes.
2020-04-23T22:29:17.2904741Z Terminate orphan process: pid (3508) (python)
2020-04-23T22:29:17.2951534Z ##[section]Finishing: Finalize Job
