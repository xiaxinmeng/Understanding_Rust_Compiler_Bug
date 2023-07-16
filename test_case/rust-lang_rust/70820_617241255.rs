plain
2020-04-21T14:07:37.6014776Z ========================== Starting Command Output ===========================
2020-04-21T14:07:37.6017610Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/577a269c-619e-42b2-86b0-512c3d1db95e.sh
2020-04-21T14:07:37.6017848Z 
2020-04-21T14:07:37.6021534Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-21T14:07:37.6037985Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-21T14:07:37.6040837Z Task         : Get sources
2020-04-21T14:07:37.6041111Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T14:07:37.6041357Z Version      : 1.0.0
2020-04-21T14:07:37.6041526Z Author       : Microsoft
---
2020-04-21T14:07:38.5993172Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-21T14:07:38.5998463Z ##[command]git config gc.auto 0
2020-04-21T14:07:38.6003374Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-21T14:07:38.6007632Z ##[command]git config --get-all http.proxy
2020-04-21T14:07:38.6014793Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-21T14:10:48.3129072Z  ---> 318032b5f0e2
2020-04-21T14:10:48.3129826Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-21T14:10:48.3130388Z  ---> Using cache
2020-04-21T14:10:48.3130702Z  ---> d44a858fd1ce
2020-04-21T14:10:48.3131611Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-21T14:10:48.3132577Z  ---> 58b910f50f5a
2020-04-21T14:10:48.3132793Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-21T14:10:48.3144524Z  ---> Using cache
2020-04-21T14:10:48.3148673Z  ---> ee7702aadba1
---
2020-04-21T14:10:48.5591142Z Looks like docker image is the same as before, not uploading
2020-04-21T14:10:56.3425397Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-21T14:10:56.3741354Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-21T14:10:56.3770336Z == clock drift check ==
2020-04-21T14:10:56.3785984Z   local time: Tue Apr 21 14:10:56 UTC 2020
2020-04-21T14:10:56.5805099Z   network time: Tue, 21 Apr 2020 14:10:56 GMT
2020-04-21T14:10:56.5834806Z Starting sccache server...
2020-04-21T14:10:56.6669173Z configure: processing command line
2020-04-21T14:10:56.6669998Z configure: 
2020-04-21T14:10:56.6671176Z configure: rust.dist-src        := False
---
2020-04-21T14:15:58.4344954Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T14:15:59.7852267Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T14:16:01.3074843Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T14:16:02.5205114Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T14:16:10.8165502Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T14:16:12.9462426Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T14:16:17.1217337Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T14:16:21.0946786Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T14:16:30.0079969Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T14:36:56.6740262Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T14:36:58.1414148Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T14:36:59.7106692Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T14:36:59.8267556Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T14:37:09.3811668Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T14:37:11.5666640Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T14:37:15.9994555Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T14:37:20.1597411Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T14:37:29.9825717Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T14:59:04.3959976Z .............................................i...................................................... 1200/9912
2020-04-21T14:59:11.3678161Z .................................................................................................... 1300/9912
2020-04-21T14:59:15.3125551Z .................................................................................................... 1400/9912
2020-04-21T14:59:21.5953357Z ................................................................................F........F.......... 1500/9912
2020-04-21T14:59:26.5278840Z ....F.......................FF....F...............FF................................................ 1600/9912
2020-04-21T14:59:36.8320909Z .................................................................................................... 1800/9912
2020-04-21T14:59:44.9908251Z ..FF..........F......................................................F.............................. 1900/9912
2020-04-21T14:59:52.5848588Z ..i................................................................................................. 2000/9912
2020-04-21T14:59:52.5848588Z ..i................................................................................................. 2000/9912
2020-04-21T14:59:58.6163442Z ............................................................................................iiiii... 2100/9912
2020-04-21T15:00:17.4960322Z .................................................................................................... 2300/9912
2020-04-21T15:00:19.6721667Z .................................................................................................... 2400/9912
2020-04-21T15:00:21.8962538Z .................................................................................................... 2500/9912
2020-04-21T15:00:27.5547076Z .................................................................................................... 2600/9912
---
2020-04-21T15:03:17.3629708Z .................................................................................................... 5100/9912
2020-04-21T15:03:24.3955906Z .................................................................................................... 5200/9912
2020-04-21T15:03:29.2594537Z ...............i.................................................................................... 5300/9912
2020-04-21T15:03:38.5418900Z .....i.............................................................................................. 5400/9912
2020-04-21T15:03:43.4262540Z .....ii.ii........i...i............................................................................. 5500/9912
2020-04-21T15:03:50.8640021Z ....................................................i............................................... 5700/9912
2020-04-21T15:03:59.2878370Z ....................................................................................ii.............. 5800/9912
2020-04-21T15:04:05.8073226Z .......................i............................................................................ 5900/9912
2020-04-21T15:04:10.9874491Z .................................................................................................... 6000/9912
2020-04-21T15:04:10.9874491Z .................................................................................................... 6000/9912
2020-04-21T15:04:21.1108565Z .................................................................................................... 6100/9912
2020-04-21T15:04:30.8858949Z .................ii...i..ii...........i............................................................. 6200/9912
2020-04-21T15:04:45.4143513Z .................................................................................................... 6400/9912
2020-04-21T15:04:48.7398421Z .................................................................................................... 6500/9912
2020-04-21T15:04:48.7398421Z .................................................................................................... 6500/9912
2020-04-21T15:04:57.9524187Z ...............................................i..ii................................................ 6600/9912
2020-04-21T15:05:19.8825888Z .................................................................................................... 6800/9912
2020-04-21T15:05:22.2614876Z ................................................i................................................... 6900/9912
2020-04-21T15:05:24.3970949Z .................................................................................................... 7000/9912
2020-04-21T15:05:26.4743422Z ........................................................................................i........... 7100/9912
---
2020-04-21T15:06:59.9067168Z .................................................................................................... 7900/9912
2020-04-21T15:07:06.2504808Z .................................................................................................... 8000/9912
2020-04-21T15:07:12.2039587Z ......................................................i............................................. 8100/9912
2020-04-21T15:07:21.5517999Z .................................................................................................... 8200/9912
2020-04-21T15:07:26.8087673Z ...iiiiii.iiiii.i................................................................................... 8300/9912
2020-04-21T15:07:39.8897631Z .................................................................................................... 8500/9912
2020-04-21T15:07:47.4475900Z .................................................................................................... 8600/9912
2020-04-21T15:08:00.5221315Z .................................................................................................... 8700/9912
2020-04-21T15:08:06.9713343Z .................................................................................................... 8800/9912
---
2020-04-21T15:09:50.5959826Z - error: aborting due to 3 previous errors
2020-04-21T15:09:50.5960194Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.5960775Z +   --> $DIR/defaults-not-assumed-fail.rs:33:16
2020-04-21T15:09:50.5961424Z +    |
2020-04-21T15:09:50.5961806Z + LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
2020-04-21T15:09:50.5962636Z +    |                ^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.5968193Z + error: aborting due to 4 previous errors
2020-04-21T15:09:50.5968715Z 30 
2020-04-21T15:09:50.5969550Z 31 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.5969970Z 32 
2020-04-21T15:09:50.5969970Z 32 
2020-04-21T15:09:50.5970178Z 
2020-04-21T15:09:50.5970379Z 
2020-04-21T15:09:50.5970696Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.5971509Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-not-assumed-fail/defaults-not-assumed-fail.stderr
2020-04-21T15:09:50.5972257Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.5973005Z To only update this specific test, also pass `--test-args associated-const/defaults-not-assumed-fail.rs`
2020-04-21T15:09:50.5973666Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.5974015Z status: exit code: 1
2020-04-21T15:09:50.5974015Z status: exit code: 1
2020-04-21T15:09:50.5975972Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/defaults-not-assumed-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-not-assumed-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-not-assumed-fail/auxiliary"
2020-04-21T15:09:50.5981840Z ------------------------------------------
2020-04-21T15:09:50.5982027Z 
2020-04-21T15:09:50.5982375Z ------------------------------------------
2020-04-21T15:09:50.5982561Z stderr:
2020-04-21T15:09:50.5982561Z stderr:
2020-04-21T15:09:50.5982923Z ------------------------------------------
2020-04-21T15:09:50.5983462Z error: any use of this value will cause an error
2020-04-21T15:09:50.5983959Z   --> /checkout/src/test/ui/associated-const/defaults-not-assumed-fail.rs:8:19
2020-04-21T15:09:50.5984201Z    |
2020-04-21T15:09:50.5984373Z LL |     const B: u8 = Self::A + 1;
2020-04-21T15:09:50.5984934Z    |                   |
2020-04-21T15:09:50.5985314Z    |                   attempt to add with overflow
2020-04-21T15:09:50.5985501Z    |
2020-04-21T15:09:50.5985705Z    = note: `#[deny(const_err)]` on by default
2020-04-21T15:09:50.5985705Z    = note: `#[deny(const_err)]` on by default
2020-04-21T15:09:50.5985867Z 
2020-04-21T15:09:50.5986063Z error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.5986590Z   --> /checkout/src/test/ui/associated-const/defaults-not-assumed-fail.rs:33:5
2020-04-21T15:09:50.5986857Z    |
2020-04-21T15:09:50.5987092Z LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
2020-04-21T15:09:50.5987748Z    |                |
2020-04-21T15:09:50.5987961Z    |                referenced constant has errors
2020-04-21T15:09:50.5988147Z    |
2020-04-21T15:09:50.5988663Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-21T15:09:50.5988663Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-21T15:09:50.5988916Z 
2020-04-21T15:09:50.5989075Z error: erroneous constant used
2020-04-21T15:09:50.5989579Z   --> /checkout/src/test/ui/associated-const/defaults-not-assumed-fail.rs:33:5
2020-04-21T15:09:50.5989824Z    |
2020-04-21T15:09:50.5990053Z LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
2020-04-21T15:09:50.5990390Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.5991242Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-21T15:09:50.5991563Z 
2020-04-21T15:09:50.5991750Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.5992263Z   --> /checkout/src/test/ui/associated-const/defaults-not-assumed-fail.rs:33:16
2020-04-21T15:09:50.5992263Z   --> /checkout/src/test/ui/associated-const/defaults-not-assumed-fail.rs:33:16
2020-04-21T15:09:50.5992509Z    |
2020-04-21T15:09:50.5992753Z LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
2020-04-21T15:09:50.5993068Z    |                ^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.5993437Z error: aborting due to 4 previous errors
2020-04-21T15:09:50.5993588Z 
2020-04-21T15:09:50.5993985Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.5994173Z 
2020-04-21T15:09:50.5994173Z 
2020-04-21T15:09:50.5994521Z ------------------------------------------
2020-04-21T15:09:50.5994675Z 
2020-04-21T15:09:50.5994763Z 
2020-04-21T15:09:50.5995104Z ---- [ui] ui/consts/const-err.rs stdout ----
2020-04-21T15:09:50.5995326Z diff of stderr:
2020-04-21T15:09:50.5995447Z 
2020-04-21T15:09:50.5995619Z 24 LL |     black_box((FOO, FOO));
2020-04-21T15:09:50.5995873Z 25    |                     ^^^ referenced constant has errors
2020-04-21T15:09:50.5996091Z 26 
2020-04-21T15:09:50.5996484Z - error: aborting due to 2 previous errors; 1 warning emitted
2020-04-21T15:09:50.5996752Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.5997330Z +    |
2020-04-21T15:09:50.5997330Z +    |
2020-04-21T15:09:50.5997498Z + LL |     black_box((FOO, FOO));
2020-04-21T15:09:50.5997757Z +    |                ^^^ referenced constant has errors
2020-04-21T15:09:50.5998237Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.5998594Z +   --> $DIR/const-err.rs:15:21
2020-04-21T15:09:50.5998772Z +    |
2020-04-21T15:09:50.5998772Z +    |
2020-04-21T15:09:50.5998926Z + LL |     black_box((FOO, FOO));
2020-04-21T15:09:50.5999157Z +    |                     ^^^ referenced constant has errors
2020-04-21T15:09:50.5999356Z + 
2020-04-21T15:09:50.5999545Z + error: aborting due to 4 previous errors; 1 warning emitted
2020-04-21T15:09:50.6000138Z 29 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6000338Z 30 
2020-04-21T15:09:50.6000424Z 
2020-04-21T15:09:50.6000507Z 
2020-04-21T15:09:50.6000507Z 
2020-04-21T15:09:50.6000695Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6001223Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err/const-err.stderr
2020-04-21T15:09:50.6001728Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6002220Z To only update this specific test, also pass `--test-args consts/const-err.rs`
2020-04-21T15:09:50.6002572Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6002785Z status: exit code: 1
2020-04-21T15:09:50.6002785Z status: exit code: 1
2020-04-21T15:09:50.6004359Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zforce-overflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err/auxiliary"
2020-04-21T15:09:50.6005849Z ------------------------------------------
2020-04-21T15:09:50.6006006Z 
2020-04-21T15:09:50.6006358Z ------------------------------------------
2020-04-21T15:09:50.6006545Z stderr:
2020-04-21T15:09:50.6006545Z stderr:
2020-04-21T15:09:50.6006890Z ------------------------------------------
2020-04-21T15:09:50.6007153Z warning: any use of this value will cause an error
2020-04-21T15:09:50.6007601Z   --> /checkout/src/test/ui/consts/const-err.rs:11:17
2020-04-21T15:09:50.6007801Z    |
2020-04-21T15:09:50.6007968Z LL | const FOO: u8 = [5u8][1];
2020-04-21T15:09:50.6008665Z    |                 |
2020-04-21T15:09:50.6008978Z    |                 index out of bounds: the len is 1 but the index is 1
2020-04-21T15:09:50.6009232Z    |
2020-04-21T15:09:50.6009404Z note: the lint level is defined here
---
2020-04-21T15:09:50.6010524Z 
2020-04-21T15:09:50.6010697Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6011129Z   --> /checkout/src/test/ui/consts/const-err.rs:15:16
2020-04-21T15:09:50.6011447Z    |
2020-04-21T15:09:50.6011612Z LL |     black_box((FOO, FOO));
2020-04-21T15:09:50.6011828Z    |                ^^^ referenced constant has errors
2020-04-21T15:09:50.6012158Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6012568Z   --> /checkout/src/test/ui/consts/const-err.rs:15:21
2020-04-21T15:09:50.6012755Z    |
2020-04-21T15:09:50.6012755Z    |
2020-04-21T15:09:50.6012923Z LL |     black_box((FOO, FOO));
2020-04-21T15:09:50.6013148Z    |                     ^^^ referenced constant has errors
2020-04-21T15:09:50.6013465Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6013880Z   --> /checkout/src/test/ui/consts/const-err.rs:15:16
2020-04-21T15:09:50.6014065Z    |
2020-04-21T15:09:50.6014065Z    |
2020-04-21T15:09:50.6014212Z LL |     black_box((FOO, FOO));
2020-04-21T15:09:50.6014446Z    |                ^^^ referenced constant has errors
2020-04-21T15:09:50.6014760Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6015157Z   --> /checkout/src/test/ui/consts/const-err.rs:15:21
2020-04-21T15:09:50.6015357Z    |
2020-04-21T15:09:50.6015357Z    |
2020-04-21T15:09:50.6015505Z LL |     black_box((FOO, FOO));
2020-04-21T15:09:50.6015727Z    |                     ^^^ referenced constant has errors
2020-04-21T15:09:50.6015905Z 
2020-04-21T15:09:50.6016096Z error: aborting due to 4 previous errors; 1 warning emitted
2020-04-21T15:09:50.6019072Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6019295Z 
2020-04-21T15:09:50.6019614Z ------------------------------------------
2020-04-21T15:09:50.6019758Z 
---
2020-04-21T15:09:50.6022370Z +    |
2020-04-21T15:09:50.6022539Z + LL |     println!("{}", FOO);
2020-04-21T15:09:50.6022782Z +    |                    ^^^ referenced constant has errors
2020-04-21T15:09:50.6022964Z + 
2020-04-21T15:09:50.6023172Z + error: aborting due to 2 previous errors; 2 warnings emitted
2020-04-21T15:09:50.6024117Z 29 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6024333Z 30 
2020-04-21T15:09:50.6024421Z 
2020-04-21T15:09:50.6024504Z 
2020-04-21T15:09:50.6024504Z 
2020-04-21T15:09:50.6024676Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6025324Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/conditional_array_execution.stderr
2020-04-21T15:09:50.6026100Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6026993Z To only update this specific test, also pass `--test-args consts/const-eval/conditional_array_execution.rs`
2020-04-21T15:09:50.6027434Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6027840Z status: exit code: 1
2020-04-21T15:09:50.6027840Z status: exit code: 1
2020-04-21T15:09:50.6029707Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/auxiliary"
2020-04-21T15:09:50.6031157Z ------------------------------------------
2020-04-21T15:09:50.6031317Z 
2020-04-21T15:09:50.6031651Z ------------------------------------------
2020-04-21T15:09:50.6031863Z stderr:
2020-04-21T15:09:50.6031863Z stderr:
2020-04-21T15:09:50.6032372Z ------------------------------------------
2020-04-21T15:09:50.6032630Z warning: any use of this value will cause an error
2020-04-21T15:09:50.6033174Z   --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:7:19
2020-04-21T15:09:50.6033425Z    |
2020-04-21T15:09:50.6033814Z LL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];
2020-04-21T15:09:50.6034512Z    |                   |
2020-04-21T15:09:50.6037462Z    |                   attempt to subtract with overflow
2020-04-21T15:09:50.6037840Z    |
2020-04-21T15:09:50.6038015Z note: the lint level is defined here
---
2020-04-21T15:09:50.6051681Z    |
2020-04-21T15:09:50.6051838Z LL |     println!("{}", FOO);
2020-04-21T15:09:50.6052091Z    |                    ^^^ referenced constant has errors
2020-04-21T15:09:50.6052281Z 
2020-04-21T15:09:50.6052482Z error: aborting due to 2 previous errors; 2 warnings emitted
2020-04-21T15:09:50.6053064Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6053273Z 
2020-04-21T15:09:50.6053607Z ------------------------------------------
2020-04-21T15:09:50.6053761Z 
2020-04-21T15:09:50.6053761Z 
2020-04-21T15:09:50.6053850Z 
2020-04-21T15:09:50.6054253Z ---- [ui] ui/consts/const-eval/const_fn_ptr_fail2.rs stdout ----
2020-04-21T15:09:50.6054489Z diff of stderr:
2020-04-21T15:09:50.6054609Z 
2020-04-21T15:09:50.6054726Z 24    |
2020-04-21T15:09:50.6055261Z 25    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-21T15:09:50.6055547Z 26 
2020-04-21T15:09:50.6055959Z - error: aborting due to 2 previous errors; 1 warning emitted
2020-04-21T15:09:50.6056230Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6057079Z +    |
2020-04-21T15:09:50.6057079Z +    |
2020-04-21T15:09:50.6057253Z + LL |     assert_eq!(Y, 4);
2020-04-21T15:09:50.6057480Z +    |                ^ referenced constant has errors
2020-04-21T15:09:50.6057862Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6058280Z +   --> $DIR/const_fn_ptr_fail2.rs:22:16
2020-04-21T15:09:50.6058469Z +    |
2020-04-21T15:09:50.6058469Z +    |
2020-04-21T15:09:50.6058643Z + LL |     assert_eq!(Z, 4);
2020-04-21T15:09:50.6058872Z +    |                ^ referenced constant has errors
2020-04-21T15:09:50.6059058Z + 
2020-04-21T15:09:50.6059282Z + error: aborting due to 4 previous errors; 1 warning emitted
2020-04-21T15:09:50.6059894Z 29 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6060128Z 30 
2020-04-21T15:09:50.6060221Z 
2020-04-21T15:09:50.6060556Z 
2020-04-21T15:09:50.6060556Z 
2020-04-21T15:09:50.6060747Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6061429Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2/const_fn_ptr_fail2.stderr
2020-04-21T15:09:50.6062021Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6062583Z To only update this specific test, also pass `--test-args consts/const-eval/const_fn_ptr_fail2.rs`
2020-04-21T15:09:50.6063009Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6063221Z status: exit code: 1
2020-04-21T15:09:50.6063221Z status: exit code: 1
2020-04-21T15:09:50.6066547Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2/auxiliary"
2020-04-21T15:09:50.6068028Z ------------------------------------------
2020-04-21T15:09:50.6068187Z 
2020-04-21T15:09:50.6068521Z ------------------------------------------
2020-04-21T15:09:50.6068725Z stderr:
2020-04-21T15:09:50.6068725Z stderr:
2020-04-21T15:09:50.6069072Z ------------------------------------------
2020-04-21T15:09:50.6069291Z warning: skipping const checks
2020-04-21T15:09:50.6069778Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:13:5
2020-04-21T15:09:50.6070015Z    |
2020-04-21T15:09:50.6070200Z LL |     x(y) //~ WARN skipping const checks
2020-04-21T15:09:50.6070518Z 
2020-04-21T15:09:50.6070712Z error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.6071223Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:20:5
2020-04-21T15:09:50.6071477Z    |
---
2020-04-21T15:09:50.6073328Z 
2020-04-21T15:09:50.6073525Z error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.6074046Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:22:5
2020-04-21T15:09:50.6074280Z    |
2020-04-21T15:09:50.6074431Z LL |     assert_eq!(Z, 4);
2020-04-21T15:09:50.6074780Z    |     ^^^^^^^^^^^-^^^^^
2020-04-21T15:09:50.6075172Z    |                referenced constant has errors
2020-04-21T15:09:50.6075483Z    |
2020-04-21T15:09:50.6076059Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-21T15:09:50.6076316Z 
---
2020-04-21T15:09:50.6077791Z 
2020-04-21T15:09:50.6077959Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6078440Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:22:16
2020-04-21T15:09:50.6078688Z    |
2020-04-21T15:09:50.6078840Z LL |     assert_eq!(Z, 4);
2020-04-21T15:09:50.6079066Z    |                ^ referenced constant has errors
2020-04-21T15:09:50.6079248Z 
2020-04-21T15:09:50.6079445Z error: aborting due to 4 previous errors; 1 warning emitted
2020-04-21T15:09:50.6080026Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6080235Z 
2020-04-21T15:09:50.6080568Z ------------------------------------------
2020-04-21T15:09:50.6080721Z 
2020-04-21T15:09:50.6080721Z 
2020-04-21T15:09:50.6080809Z 
2020-04-21T15:09:50.6081193Z ---- [ui] ui/consts/const-eval/issue-43197.rs stdout ----
2020-04-21T15:09:50.6081418Z diff of stderr:
2020-04-21T15:09:50.6081536Z 
2020-04-21T15:09:50.6081697Z 44 LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6081969Z 45    |                          ^ referenced constant has errors
2020-04-21T15:09:50.6082173Z 46 
2020-04-21T15:09:50.6082566Z - error: aborting due to 2 previous errors; 4 warnings emitted
2020-04-21T15:09:50.6082853Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6083424Z +    |
2020-04-21T15:09:50.6083424Z +    |
2020-04-21T15:09:50.6083610Z + LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6083856Z +    |                       ^ referenced constant has errors
2020-04-21T15:09:50.6084254Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6084649Z +   --> $DIR/issue-43197.rs:14:26
2020-04-21T15:09:50.6084826Z +    |
2020-04-21T15:09:50.6084826Z +    |
2020-04-21T15:09:50.6084995Z + LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6085263Z +    |                          ^ referenced constant has errors
2020-04-21T15:09:50.6085465Z + 
2020-04-21T15:09:50.6085673Z + error: aborting due to 4 previous errors; 4 warnings emitted
2020-04-21T15:09:50.6086306Z 49 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6086520Z 50 
2020-04-21T15:09:50.6086630Z 
2020-04-21T15:09:50.6086722Z 
2020-04-21T15:09:50.6086722Z 
2020-04-21T15:09:50.6086909Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6087517Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/issue-43197.stderr
2020-04-21T15:09:50.6088105Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6088655Z To only update this specific test, also pass `--test-args consts/const-eval/issue-43197.rs`
2020-04-21T15:09:50.6089070Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6089283Z status: exit code: 1
2020-04-21T15:09:50.6089283Z status: exit code: 1
2020-04-21T15:09:50.6090999Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-43197.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/auxiliary"
2020-04-21T15:09:50.6092482Z ------------------------------------------
2020-04-21T15:09:50.6092662Z 
2020-04-21T15:09:50.6093007Z ------------------------------------------
2020-04-21T15:09:50.6093192Z stderr:
2020-04-21T15:09:50.6093192Z stderr:
2020-04-21T15:09:50.6093553Z ------------------------------------------
2020-04-21T15:09:50.6093802Z warning: any use of this value will cause an error
2020-04-21T15:09:50.6094280Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:10:20
2020-04-21T15:09:50.6094522Z    |
2020-04-21T15:09:50.6094850Z LL |     const X: u32 = 0 - 1;
2020-04-21T15:09:50.6095421Z    |                    |
2020-04-21T15:09:50.6095651Z    |                    attempt to subtract with overflow
2020-04-21T15:09:50.6095845Z    |
2020-04-21T15:09:50.6096013Z note: the lint level is defined here
---
2020-04-21T15:09:50.6097176Z 
2020-04-21T15:09:50.6097361Z warning: any use of this value will cause an error
2020-04-21T15:09:50.6097858Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:12:24
2020-04-21T15:09:50.6098077Z    |
2020-04-21T15:09:50.6098413Z LL |     const Y: u32 = foo(0 - 1);
2020-04-21T15:09:50.6099012Z    |                        |
2020-04-21T15:09:50.6099250Z    |                        attempt to subtract with overflow
2020-04-21T15:09:50.6099427Z 
2020-04-21T15:09:50.6099635Z error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.6099635Z error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.6100124Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:23
2020-04-21T15:09:50.6100342Z    |
2020-04-21T15:09:50.6100521Z LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6100764Z    |                       ^ referenced constant has errors
2020-04-21T15:09:50.6101100Z warning: erroneous constant used
2020-04-21T15:09:50.6101579Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:23
2020-04-21T15:09:50.6101799Z    |
2020-04-21T15:09:50.6101799Z    |
2020-04-21T15:09:50.6101962Z LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6102220Z    |                       ^ referenced constant has errors
2020-04-21T15:09:50.6102588Z error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.6103089Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:26
2020-04-21T15:09:50.6103870Z    |
2020-04-21T15:09:50.6103870Z    |
2020-04-21T15:09:50.6104033Z LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6104280Z    |                          ^ referenced constant has errors
2020-04-21T15:09:50.6104632Z warning: erroneous constant used
2020-04-21T15:09:50.6105097Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:26
2020-04-21T15:09:50.6105334Z    |
2020-04-21T15:09:50.6105334Z    |
2020-04-21T15:09:50.6105498Z LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6105753Z    |                          ^ referenced constant has errors
2020-04-21T15:09:50.6106118Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6106581Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:23
2020-04-21T15:09:50.6106800Z    |
2020-04-21T15:09:50.6106800Z    |
2020-04-21T15:09:50.6106979Z LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6107221Z    |                       ^ referenced constant has errors
2020-04-21T15:09:50.6107581Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6108043Z   --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:14:26
2020-04-21T15:09:50.6108263Z    |
2020-04-21T15:09:50.6108263Z    |
2020-04-21T15:09:50.6108424Z LL |     println!("{} {}", X, Y);
2020-04-21T15:09:50.6108688Z    |                          ^ referenced constant has errors
2020-04-21T15:09:50.6108865Z 
2020-04-21T15:09:50.6109063Z error: aborting due to 4 previous errors; 4 warnings emitted
2020-04-21T15:09:50.6109719Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6109957Z 
2020-04-21T15:09:50.6110298Z ------------------------------------------
2020-04-21T15:09:50.6110468Z 
2020-04-21T15:09:50.6110468Z 
2020-04-21T15:09:50.6110558Z 
2020-04-21T15:09:50.6110927Z ---- [ui] ui/consts/const-eval/issue-44578.rs stdout ----
2020-04-21T15:09:50.6111150Z diff of stderr:
2020-04-21T15:09:50.6111288Z 
2020-04-21T15:09:50.6111504Z 4 LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
2020-04-21T15:09:50.6111837Z 5    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6112421Z - error: aborting due to previous error
2020-04-21T15:09:50.6112657Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6113051Z +   --> $DIR/issue-44578.rs:27:20
2020-04-21T15:09:50.6113247Z +    |
2020-04-21T15:09:50.6113247Z +    |
2020-04-21T15:09:50.6113466Z + LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
2020-04-21T15:09:50.6113798Z +    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6114228Z + error: aborting due to 2 previous errors
2020-04-21T15:09:50.6114402Z 8 
2020-04-21T15:09:50.6114824Z 9 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6115038Z 10 
2020-04-21T15:09:50.6115038Z 10 
2020-04-21T15:09:50.6115132Z 
2020-04-21T15:09:50.6115221Z 
2020-04-21T15:09:50.6115423Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6116032Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578/issue-44578.stderr
2020-04-21T15:09:50.6116602Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6117166Z To only update this specific test, also pass `--test-args consts/const-eval/issue-44578.rs`
2020-04-21T15:09:50.6117564Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6117793Z status: exit code: 1
2020-04-21T15:09:50.6117793Z status: exit code: 1
2020-04-21T15:09:50.6119519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-44578.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578/auxiliary"
2020-04-21T15:09:50.6120897Z ------------------------------------------
2020-04-21T15:09:50.6121052Z 
2020-04-21T15:09:50.6121404Z ------------------------------------------
2020-04-21T15:09:50.6121589Z stderr:
2020-04-21T15:09:50.6121589Z stderr:
2020-04-21T15:09:50.6121934Z ------------------------------------------
2020-04-21T15:09:50.6122208Z error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.6122698Z   --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:27:20
2020-04-21T15:09:50.6122928Z    |
2020-04-21T15:09:50.6123162Z LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
2020-04-21T15:09:50.6123488Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6123865Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6124345Z   --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:27:20
2020-04-21T15:09:50.6124567Z    |
2020-04-21T15:09:50.6124567Z    |
2020-04-21T15:09:50.6124781Z LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
2020-04-21T15:09:50.6125124Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6125499Z error: aborting due to 2 previous errors
2020-04-21T15:09:50.6125651Z 
2020-04-21T15:09:50.6126062Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6126250Z 
---
2020-04-21T15:09:50.6128182Z 10 
2020-04-21T15:09:50.6128380Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6128770Z +   --> $DIR/issue-50814.rs:20:6
2020-04-21T15:09:50.6128945Z +    |
2020-04-21T15:09:50.6129140Z + LL |     &Sum::<U8,U8>::MAX
2020-04-21T15:09:50.6129393Z +    |      ^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6129810Z 11 error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.6130232Z 12   --> $DIR/issue-50814.rs:20:5
2020-04-21T15:09:50.6130409Z 13    |
2020-04-21T15:09:50.6130509Z 
---
2020-04-21T15:09:50.6132539Z 
2020-04-21T15:09:50.6132630Z 
2020-04-21T15:09:50.6132833Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6133443Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/issue-50814.stderr
2020-04-21T15:09:50.6134009Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6134571Z To only update this specific test, also pass `--test-args consts/const-eval/issue-50814.rs`
2020-04-21T15:09:50.6134967Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6135194Z status: exit code: 1
2020-04-21T15:09:50.6135194Z status: exit code: 1
2020-04-21T15:09:50.6136919Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-50814.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/auxiliary"
2020-04-21T15:09:50.6138295Z ------------------------------------------
2020-04-21T15:09:50.6138449Z 
2020-04-21T15:09:50.6138802Z ------------------------------------------
2020-04-21T15:09:50.6138985Z stderr:
2020-04-21T15:09:50.6138985Z stderr:
2020-04-21T15:09:50.6139330Z ------------------------------------------
2020-04-21T15:09:50.6139576Z error: any use of this value will cause an error
2020-04-21T15:09:50.6140080Z   --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:15:21
2020-04-21T15:09:50.6140304Z    |
2020-04-21T15:09:50.6140522Z LL |     const MAX: u8 = A::MAX + B::MAX;
2020-04-21T15:09:50.6141143Z    |                     |
2020-04-21T15:09:50.6141367Z    |                     attempt to add with overflow
2020-04-21T15:09:50.6141574Z    |
2020-04-21T15:09:50.6141762Z    = note: `#[deny(const_err)]` on by default
2020-04-21T15:09:50.6141762Z    = note: `#[deny(const_err)]` on by default
2020-04-21T15:09:50.6141922Z 
2020-04-21T15:09:50.6142110Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6142573Z   --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:20:6
2020-04-21T15:09:50.6142793Z    |
2020-04-21T15:09:50.6142978Z LL |     &Sum::<U8,U8>::MAX
2020-04-21T15:09:50.6143224Z    |      ^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6143778Z error[E0080]: evaluation of constant expression failed
2020-04-21T15:09:50.6144500Z   --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:20:5
2020-04-21T15:09:50.6144725Z    |
2020-04-21T15:09:50.6144725Z    |
2020-04-21T15:09:50.6144892Z LL |     &Sum::<U8,U8>::MAX
2020-04-21T15:09:50.6145438Z    |      |
2020-04-21T15:09:50.6145623Z    |      referenced constant has errors
2020-04-21T15:09:50.6145771Z 
2020-04-21T15:09:50.6145958Z error: aborting due to 3 previous errors
---
2020-04-21T15:09:50.6147279Z 
2020-04-21T15:09:50.6147668Z ---- [ui] ui/consts/const-eval/panic-assoc-never-type.rs stdout ----
2020-04-21T15:09:50.6147927Z diff of stderr:
2020-04-21T15:09:50.6148047Z 
2020-04-21T15:09:50.6148223Z 19 LL |     let _ = PrintName::VOID;
2020-04-21T15:09:50.6148517Z 20    |             ^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6149112Z - error: aborting due to previous error; 1 warning emitted
2020-04-21T15:09:50.6149393Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6149817Z +   --> $DIR/panic-assoc-never-type.rs:16:13
2020-04-21T15:09:50.6150015Z +    |
2020-04-21T15:09:50.6150015Z +    |
2020-04-21T15:09:50.6150200Z + LL |     let _ = PrintName::VOID;
2020-04-21T15:09:50.6150485Z +    |             ^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6150688Z + 
2020-04-21T15:09:50.6150893Z + error: aborting due to 2 previous errors; 1 warning emitted
2020-04-21T15:09:50.6151603Z 24 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6151801Z 25 
2020-04-21T15:09:50.6151905Z 
2020-04-21T15:09:50.6151989Z 
2020-04-21T15:09:50.6151989Z 
2020-04-21T15:09:50.6152161Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6152775Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-assoc-never-type/panic-assoc-never-type.stderr
2020-04-21T15:09:50.6153352Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6153877Z To only update this specific test, also pass `--test-args consts/const-eval/panic-assoc-never-type.rs`
2020-04-21T15:09:50.6154278Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6154475Z status: exit code: 1
2020-04-21T15:09:50.6154475Z status: exit code: 1
2020-04-21T15:09:50.6156129Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-assoc-never-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-assoc-never-type/auxiliary"
2020-04-21T15:09:50.6157801Z ------------------------------------------
2020-04-21T15:09:50.6157945Z 
2020-04-21T15:09:50.6158256Z ------------------------------------------
2020-04-21T15:09:50.6158427Z stderr:
2020-04-21T15:09:50.6158427Z stderr:
2020-04-21T15:09:50.6158762Z ------------------------------------------
2020-04-21T15:09:50.6158991Z warning: any use of this value will cause an error
2020-04-21T15:09:50.6159462Z   --> /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:11:21
2020-04-21T15:09:50.6159703Z    |
2020-04-21T15:09:50.6159873Z LL |     const VOID: ! = panic!();
2020-04-21T15:09:50.6160424Z    |                     |
2020-04-21T15:09:50.6161014Z    |                     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:11:21
2020-04-21T15:09:50.6161386Z    |
2020-04-21T15:09:50.6161597Z note: the lint level is defined here
---
2020-04-21T15:09:50.6163343Z 
2020-04-21T15:09:50.6163502Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6163958Z   --> /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:16:13
2020-04-21T15:09:50.6164198Z    |
2020-04-21T15:09:50.6164362Z LL |     let _ = PrintName::VOID;
2020-04-21T15:09:50.6164608Z    |             ^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6164947Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6165406Z   --> /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:16:13
2020-04-21T15:09:50.6165633Z    |
2020-04-21T15:09:50.6165633Z    |
2020-04-21T15:09:50.6165815Z LL |     let _ = PrintName::VOID;
2020-04-21T15:09:50.6166055Z    |             ^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6166221Z 
2020-04-21T15:09:50.6166406Z error: aborting due to 2 previous errors; 1 warning emitted
2020-04-21T15:09:50.6166953Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6167126Z 
2020-04-21T15:09:50.6167449Z ------------------------------------------
2020-04-21T15:09:50.6167591Z 
---
2020-04-21T15:09:50.6170079Z +    |
2020-04-21T15:09:50.6170219Z + LL |     let _ = VOID;
2020-04-21T15:09:50.6170445Z +    |             ^^^^ referenced constant has errors
2020-04-21T15:09:50.6170620Z + 
2020-04-21T15:09:50.6170808Z + error: aborting due to 2 previous errors; 1 warning emitted
2020-04-21T15:09:50.6171387Z 24 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6171588Z 25 
2020-04-21T15:09:50.6171675Z 
2020-04-21T15:09:50.6171759Z 
2020-04-21T15:09:50.6171759Z 
2020-04-21T15:09:50.6171949Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6172531Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-never-type/panic-never-type.stderr
2020-04-21T15:09:50.6173066Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6173605Z To only update this specific test, also pass `--test-args consts/const-eval/panic-never-type.rs`
2020-04-21T15:09:50.6173980Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6174191Z status: exit code: 1
2020-04-21T15:09:50.6174191Z status: exit code: 1
2020-04-21T15:09:50.6175806Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/panic-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-never-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-never-type/auxiliary"
2020-04-21T15:09:50.6177396Z ------------------------------------------
2020-04-21T15:09:50.6177554Z 
2020-04-21T15:09:50.6177916Z ------------------------------------------
2020-04-21T15:09:50.6178100Z stderr:
2020-04-21T15:09:50.6178100Z stderr:
2020-04-21T15:09:50.6178445Z ------------------------------------------
2020-04-21T15:09:50.6178711Z warning: any use of this value will cause an error
2020-04-21T15:09:50.6179206Z   --> /checkout/src/test/ui/consts/const-eval/panic-never-type.rs:8:17
2020-04-21T15:09:50.6179436Z    |
2020-04-21T15:09:50.6179623Z LL | const VOID: ! = panic!();
2020-04-21T15:09:50.6180169Z    |                 |
2020-04-21T15:09:50.6180788Z    |                 the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/panic-never-type.rs:8:17
2020-04-21T15:09:50.6181115Z    |
2020-04-21T15:09:50.6181289Z note: the lint level is defined here
---
2020-04-21T15:09:50.6190508Z    |
2020-04-21T15:09:50.6190653Z LL |     let _ = VOID;
2020-04-21T15:09:50.6190886Z    |             ^^^^ referenced constant has errors
2020-04-21T15:09:50.6191066Z 
2020-04-21T15:09:50.6191270Z error: aborting due to 2 previous errors; 1 warning emitted
2020-04-21T15:09:50.6191864Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6192053Z 
2020-04-21T15:09:50.6192390Z ------------------------------------------
2020-04-21T15:09:50.6192544Z 
2020-04-21T15:09:50.6192544Z 
2020-04-21T15:09:50.6192635Z 
2020-04-21T15:09:50.6193031Z ---- [ui] ui/consts/miri_unleashed/assoc_const.rs stdout ----
2020-04-21T15:09:50.6193261Z diff of stderr:
2020-04-21T15:09:50.6193382Z 
2020-04-21T15:09:50.6193622Z 10 LL |     let y = <String as Bar<Vec<u32>, String>>::F;
2020-04-21T15:09:50.6193967Z 11    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6194602Z - error: aborting due to previous error; 1 warning emitted
2020-04-21T15:09:50.6194868Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6195261Z +   --> $DIR/assoc_const.rs:31:13
2020-04-21T15:09:50.6195443Z +    |
2020-04-21T15:09:50.6195443Z +    |
2020-04-21T15:09:50.6195691Z + LL |     let y = <String as Bar<Vec<u32>, String>>::F;
2020-04-21T15:09:50.6196034Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6196266Z + 
2020-04-21T15:09:50.6196486Z + error: aborting due to 2 previous errors; 1 warning emitted
2020-04-21T15:09:50.6197097Z 15 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6197327Z 16 
2020-04-21T15:09:50.6197421Z 
2020-04-21T15:09:50.6197510Z 
2020-04-21T15:09:50.6197510Z 
2020-04-21T15:09:50.6197697Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6198327Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/assoc_const.stderr
2020-04-21T15:09:50.6198898Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6199449Z To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const.rs`
2020-04-21T15:09:50.6200079Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6200292Z status: exit code: 1
2020-04-21T15:09:50.6200292Z status: exit code: 1
2020-04-21T15:09:50.6202147Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/auxiliary"
2020-04-21T15:09:50.6203604Z ------------------------------------------
2020-04-21T15:09:50.6203759Z 
2020-04-21T15:09:50.6204105Z ------------------------------------------
2020-04-21T15:09:50.6204310Z stderr:
2020-04-21T15:09:50.6204310Z stderr:
2020-04-21T15:09:50.6204659Z ------------------------------------------
2020-04-21T15:09:50.6204879Z warning: skipping const checks
2020-04-21T15:09:50.6205797Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:14:20
2020-04-21T15:09:50.6206032Z    |
2020-04-21T15:09:50.6206268Z LL |     const F: u32 = (U::X, 42).1; //~ WARN skipping const checks
2020-04-21T15:09:50.6206698Z 
2020-04-21T15:09:50.6206868Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6207363Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:31:13
2020-04-21T15:09:50.6207605Z    |
2020-04-21T15:09:50.6207605Z    |
2020-04-21T15:09:50.6207866Z LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ ERROR erroneous constant
2020-04-21T15:09:50.6208244Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6208649Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6209120Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:31:13
2020-04-21T15:09:50.6209362Z    |
2020-04-21T15:09:50.6209362Z    |
2020-04-21T15:09:50.6209625Z LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ ERROR erroneous constant
2020-04-21T15:09:50.6209999Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6210225Z 
2020-04-21T15:09:50.6210424Z error: aborting due to 2 previous errors; 1 warning emitted
2020-04-21T15:09:50.6210998Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6211207Z 
2020-04-21T15:09:50.6211542Z ------------------------------------------
2020-04-21T15:09:50.6211695Z 
2020-04-21T15:09:50.6211695Z 
2020-04-21T15:09:50.6211784Z 
2020-04-21T15:09:50.6212187Z ---- [ui] ui/consts/miri_unleashed/assoc_const_2.rs stdout ----
2020-04-21T15:09:50.6212419Z diff of stderr:
2020-04-21T15:09:50.6212539Z 
2020-04-21T15:09:50.6212736Z 4 LL |     let y = <String as Bar<String>>::F;
2020-04-21T15:09:50.6213070Z 5    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6213630Z - error: aborting due to previous error
2020-04-21T15:09:50.6213886Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6214285Z +   --> $DIR/assoc_const_2.rs:29:13
2020-04-21T15:09:50.6214466Z +    |
2020-04-21T15:09:50.6214466Z +    |
2020-04-21T15:09:50.6214692Z + LL |     let y = <String as Bar<String>>::F;
2020-04-21T15:09:50.6215000Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6215415Z + error: aborting due to 2 previous errors
2020-04-21T15:09:50.6215590Z 8 
2020-04-21T15:09:50.6215991Z 9 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6216206Z 10 
2020-04-21T15:09:50.6216206Z 10 
2020-04-21T15:09:50.6216317Z 
2020-04-21T15:09:50.6216406Z 
2020-04-21T15:09:50.6216591Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6217341Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2/assoc_const_2.stderr
2020-04-21T15:09:50.6217952Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6218511Z To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const_2.rs`
2020-04-21T15:09:50.6218934Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6219145Z status: exit code: 1
2020-04-21T15:09:50.6219145Z status: exit code: 1
2020-04-21T15:09:50.6221436Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2/auxiliary"
2020-04-21T15:09:50.6223719Z ------------------------------------------
2020-04-21T15:09:50.6223882Z 
2020-04-21T15:09:50.6224525Z ------------------------------------------
2020-04-21T15:09:50.6224716Z stderr:
2020-04-21T15:09:50.6224716Z stderr:
2020-04-21T15:09:50.6225081Z ------------------------------------------
2020-04-21T15:09:50.6225441Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6225932Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs:29:13
2020-04-21T15:09:50.6226183Z    |
2020-04-21T15:09:50.6226431Z LL |     let y = <String as Bar<String>>::F; //~ ERROR erroneous constant
2020-04-21T15:09:50.6226792Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6227159Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6227647Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs:29:13
2020-04-21T15:09:50.6227896Z    |
2020-04-21T15:09:50.6227896Z    |
2020-04-21T15:09:50.6228138Z LL |     let y = <String as Bar<String>>::F; //~ ERROR erroneous constant
2020-04-21T15:09:50.6228479Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6228864Z error: aborting due to 2 previous errors
2020-04-21T15:09:50.6229017Z 
2020-04-21T15:09:50.6229415Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6229621Z 
---
2020-04-21T15:09:50.6232840Z +    |
2020-04-21T15:09:50.6233033Z + LL |     println!("{:?}", C);
2020-04-21T15:09:50.6233287Z +    |                      ^ referenced constant has errors
2020-04-21T15:09:50.6233482Z + 
2020-04-21T15:09:50.6233688Z + error: aborting due to 2 previous errors; 3 warnings emitted
2020-04-21T15:09:50.6234318Z 35 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6234533Z 36 
2020-04-21T15:09:50.6234644Z 
2020-04-21T15:09:50.6234734Z 
2020-04-21T15:09:50.6234734Z 
2020-04-21T15:09:50.6234918Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6235537Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn/non_const_fn.stderr
2020-04-21T15:09:50.6236317Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6236888Z To only update this specific test, also pass `--test-args consts/miri_unleashed/non_const_fn.rs`
2020-04-21T15:09:50.6237306Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6237520Z status: exit code: 1
2020-04-21T15:09:50.6237520Z status: exit code: 1
2020-04-21T15:09:50.6239367Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn/auxiliary"
2020-04-21T15:09:50.6240830Z ------------------------------------------
2020-04-21T15:09:50.6240989Z 
2020-04-21T15:09:50.6241324Z ------------------------------------------
2020-04-21T15:09:50.6241510Z stderr:
2020-04-21T15:09:50.6241510Z stderr:
2020-04-21T15:09:50.6241872Z ------------------------------------------
2020-04-21T15:09:50.6242093Z warning: skipping const checks
2020-04-21T15:09:50.6242553Z   --> /checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs:10:15
2020-04-21T15:09:50.6242804Z    |
2020-04-21T15:09:50.6243037Z LL | const C: () = foo(); //~ WARN: skipping const checks
2020-04-21T15:09:50.6243435Z 
2020-04-21T15:09:50.6243621Z warning: any use of this value will cause an error
2020-04-21T15:09:50.6244116Z   --> /checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs:10:15
2020-04-21T15:09:50.6244360Z    |
2020-04-21T15:09:50.6244360Z    |
2020-04-21T15:09:50.6244588Z LL | const C: () = foo(); //~ WARN: skipping const checks
2020-04-21T15:09:50.6245240Z    |               |
2020-04-21T15:09:50.6245629Z    |               calling non-const function `foo`
2020-04-21T15:09:50.6245820Z    |
2020-04-21T15:09:50.6245990Z note: the lint level is defined here
---
2020-04-21T15:09:50.6251052Z    |
2020-04-21T15:09:50.6251234Z LL |     println!("{:?}", C);
2020-04-21T15:09:50.6251479Z    |                      ^ referenced constant has errors
2020-04-21T15:09:50.6251651Z 
2020-04-21T15:09:50.6251868Z error: aborting due to 2 previous errors; 3 warnings emitted
2020-04-21T15:09:50.6252441Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6252630Z 
2020-04-21T15:09:50.6252978Z ------------------------------------------
2020-04-21T15:09:50.6253190Z 
---
2020-04-21T15:09:50.6255111Z - error: aborting due to 3 previous errors
2020-04-21T15:09:50.6255350Z + error[E0080]: erroneous constant used
2020-04-21T15:09:50.6255811Z +   --> $DIR/uninhabited-const-issue-61744.rs:18:10
2020-04-21T15:09:50.6256023Z +    |
2020-04-21T15:09:50.6256198Z + LL |     dbg!(i32::CONSTANT);
2020-04-21T15:09:50.6256465Z +    |          ^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6256841Z + error: aborting due to 4 previous errors
2020-04-21T15:09:50.6271844Z 159 
2020-04-21T15:09:50.6272566Z 160 For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6272808Z 161 
2020-04-21T15:09:50.6272808Z 161 
2020-04-21T15:09:50.6272928Z 
2020-04-21T15:09:50.6273020Z 
2020-04-21T15:09:50.6273211Z The actual stderr differed from the expected stderr.
2020-04-21T15:09:50.6273908Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/uninhabited-const-issue-61744.stderr
2020-04-21T15:09:50.6274677Z To update references, rerun the tests and pass the `--bless` flag
2020-04-21T15:09:50.6275374Z To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`
2020-04-21T15:09:50.6275799Z error: 1 errors occurred comparing output.
2020-04-21T15:09:50.6276014Z status: exit code: 1
2020-04-21T15:09:50.6276014Z status: exit code: 1
2020-04-21T15:09:50.6277912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary"
2020-04-21T15:09:50.6279522Z ------------------------------------------
2020-04-21T15:09:50.6279680Z 
2020-04-21T15:09:50.6280024Z ------------------------------------------
2020-04-21T15:09:50.6280211Z stderr:
2020-04-21T15:09:50.6280211Z stderr:
2020-04-21T15:09:50.6280575Z ------------------------------------------
2020-04-21T15:09:50.6280826Z error[E0080]: evaluation of constant value failed
2020-04-21T15:09:50.6281335Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2020-04-21T15:09:50.6281590Z    |
2020-04-21T15:09:50.6281749Z LL |     hint_unreachable()
2020-04-21T15:09:50.6282092Z    |     ------------------
2020-04-21T15:09:50.6282283Z    |     |
2020-04-21T15:09:50.6282812Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6283495Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6284197Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6284878Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6285575Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6286256Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6286933Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6287767Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6288516Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6289199Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6289897Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6290575Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6291353Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6292000Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6292628Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6293283Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6293913Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6294541Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6295187Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6295819Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6296449Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6297094Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6297721Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6298361Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6299007Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6299636Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6300280Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6300914Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6301543Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6302189Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6302817Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6303598Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6304257Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6304886Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6305529Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6306159Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6306787Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6307433Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6308120Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6308806Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6309449Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6310077Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6310704Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6311348Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6311977Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6312619Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6313251Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6313883Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6314524Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6315157Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6315783Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6316426Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6317056Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6317685Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6318331Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6318964Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6319608Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6320241Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6320870Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6321515Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6322146Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6322776Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6323428Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6324055Z    |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6324695Z    |     inside `fake_type::<i32>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2020-04-21T15:09:50.6324981Z ...
2020-04-21T15:09:50.6325177Z LL |     fake_type() //~ ERROR evaluation of constant value failed
2020-04-21T15:09:50.6325550Z    |     |
2020-04-21T15:09:50.6325750Z    |     reached the configured maximum number of stack frames
2020-04-21T15:09:50.6326297Z    |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2020-04-21T15:09:50.6326932Z    |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
---
2020-04-21T15:09:50.6369477Z 
2020-04-21T15:09:50.6369706Z error: any use of this value will cause an error
2020-04-21T15:09:50.6370232Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:12:36
2020-04-21T15:09:50.6370487Z    |
2020-04-21T15:09:50.6370769Z LL |     const CONSTANT: i32 = unsafe { fake_type() }; //~ ERROR any use of this value will cause an err
2020-04-21T15:09:50.6371559Z    |                                    |
2020-04-21T15:09:50.6371830Z    |                                    referenced constant has errors
2020-04-21T15:09:50.6372043Z    |
2020-04-21T15:09:50.6372243Z    = note: `#[deny(const_err)]` on by default
2020-04-21T15:09:50.6372243Z    = note: `#[deny(const_err)]` on by default
2020-04-21T15:09:50.6372403Z 
2020-04-21T15:09:50.6372573Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6373067Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:18:10
2020-04-21T15:09:50.6373323Z    |
2020-04-21T15:09:50.6373543Z LL |     dbg!(i32::CONSTANT); //~ ERROR erroneous constant used
2020-04-21T15:09:50.6373850Z    |          ^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6374206Z error[E0080]: erroneous constant used
2020-04-21T15:09:50.6374702Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:18:10
2020-04-21T15:09:50.6374956Z    |
2020-04-21T15:09:50.6374956Z    |
2020-04-21T15:09:50.6375175Z LL |     dbg!(i32::CONSTANT); //~ ERROR erroneous constant used
2020-04-21T15:09:50.6375472Z    |          ^^^^^^^^^^^^^ referenced constant has errors
2020-04-21T15:09:50.6375830Z error: aborting due to 4 previous errors
2020-04-21T15:09:50.6375981Z 
2020-04-21T15:09:50.6376918Z For more information about this error, try `rustc --explain E0080`.
2020-04-21T15:09:50.6377112Z 
---
2020-04-21T15:09:50.6384708Z test result: FAILED. 9838 passed; 13 failed; 61 ignored; 0 measured; 0 filtered out
2020-04-21T15:09:50.6384957Z 
2020-04-21T15:09:50.6385047Z 
2020-04-21T15:09:50.6385135Z 
2020-04-21T15:09:50.6388558Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-21T15:09:50.6390872Z 
2020-04-21T15:09:50.6390966Z 
2020-04-21T15:09:50.6391447Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-21T15:09:50.6391792Z Build completed unsuccessfully in 0:57:16
2020-04-21T15:09:50.6391792Z Build completed unsuccessfully in 0:57:16
2020-04-21T15:09:50.6392327Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-21T15:09:50.6392701Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-21T15:09:50.6397002Z == clock drift check ==
2020-04-21T15:09:50.6397720Z   local time: Tue Apr 21 15:09:50 UTC 2020
2020-04-21T15:09:50.7526894Z   network time: Tue, 21 Apr 2020 15:09:50 GMT
2020-04-21T15:09:51.4820841Z 
2020-04-21T15:09:51.4820841Z 
2020-04-21T15:09:51.4884443Z ##[error]Bash exited with code '1'.
2020-04-21T15:09:51.4902025Z ##[section]Finishing: Run build
2020-04-21T15:09:51.4959822Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-21T15:09:51.4964732Z Task         : Get sources
2020-04-21T15:09:51.4965044Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T15:09:51.4965326Z Version      : 1.0.0
2020-04-21T15:09:51.4965546Z Author       : Microsoft
2020-04-21T15:09:51.4965546Z Author       : Microsoft
2020-04-21T15:09:51.4965866Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T15:09:51.4966233Z ==============================================================================
2020-04-21T15:09:51.8258491Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T15:09:51.8304910Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-21T15:09:51.8393087Z Cleaning up task key
2020-04-21T15:09:51.8394331Z Start cleaning up orphan processes.
2020-04-21T15:09:51.8690214Z Terminate orphan process: pid (3668) (python)
2020-04-21T15:09:51.8733464Z ##[section]Finishing: Finalize Job
