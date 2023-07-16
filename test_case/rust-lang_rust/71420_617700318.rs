plain
2020-04-22T10:15:25.4531470Z ========================== Starting Command Output ===========================
2020-04-22T10:15:25.4534279Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/570d6fc4-6216-40ca-bee1-dcd6c4bc39ad.sh
2020-04-22T10:15:25.4534524Z 
2020-04-22T10:15:25.4538648Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T10:15:25.4557508Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T10:15:25.4560748Z Task         : Get sources
2020-04-22T10:15:25.4561024Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T10:15:25.4561297Z Version      : 1.0.0
2020-04-22T10:15:25.4561500Z Author       : Microsoft
---
2020-04-22T10:15:26.7025144Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T10:15:26.7034817Z ##[command]git config gc.auto 0
2020-04-22T10:15:26.7040774Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T10:15:26.7046065Z ##[command]git config --get-all http.proxy
2020-04-22T10:15:26.7056191Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71420/merge:refs/remotes/pull/71420/merge
---
2020-04-22T10:17:44.9944957Z  ---> 318032b5f0e2
2020-04-22T10:17:44.9945721Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T10:17:44.9950406Z  ---> Using cache
2020-04-22T10:17:44.9950797Z  ---> d44a858fd1ce
2020-04-22T10:17:44.9951702Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T10:17:44.9957126Z  ---> 58b910f50f5a
2020-04-22T10:17:44.9957329Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T10:17:44.9961794Z  ---> Using cache
2020-04-22T10:17:44.9962177Z  ---> ee7702aadba1
---
2020-04-22T10:17:45.0403916Z Looks like docker image is the same as before, not uploading
2020-04-22T10:17:52.9168020Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T10:17:52.9378759Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T10:17:52.9407070Z == clock drift check ==
2020-04-22T10:17:52.9422066Z   local time: Wed Apr 22 10:17:52 UTC 2020
2020-04-22T10:17:53.2397166Z   network time: Wed, 22 Apr 2020 10:17:53 GMT
2020-04-22T10:17:53.2422497Z Starting sccache server...
2020-04-22T10:17:53.3287464Z configure: processing command line
2020-04-22T10:17:53.3288268Z configure: 
2020-04-22T10:17:53.3289541Z configure: rust.dist-src        := False
---
2020-04-22T10:23:14.2806383Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T10:23:15.7403862Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T10:23:17.3778220Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T10:23:18.2962509Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T10:23:27.3610299Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T10:23:29.4377241Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T10:23:33.7914814Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T10:23:37.9142948Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T10:23:47.5081847Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T10:43:31.2073294Z    |
2020-04-22T10:43:31.2074249Z 13 | #![feature(specialization)]
2020-04-22T10:43:31.2075304Z    |            ^^^^^^^^^^^^^^
2020-04-22T10:43:31.2076135Z    |
2020-04-22T10:43:31.2077168Z    = note: `-D incomplete-features` implied by `-D warnings`
2020-04-22T10:43:31.2078825Z    = note: see issue #31844 <***/issues/31844> for more information
2020-04-22T10:43:32.1924256Z error: aborting due to previous error
2020-04-22T10:43:32.1924523Z 
2020-04-22T10:43:32.1974710Z error: could not compile `serialize`.
2020-04-22T10:43:32.1974977Z 
---
2020-04-22T10:43:32.3250570Z expected success, got: exit code: 101
2020-04-22T10:43:32.3264540Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T10:43:32.3264972Z Build completed unsuccessfully in 0:23:55
2020-04-22T10:43:32.3324345Z == clock drift check ==
2020-04-22T10:43:32.3343873Z   local time: Wed Apr 22 10:43:32 UTC 2020
2020-04-22T10:43:32.5017354Z   network time: Wed, 22 Apr 2020 10:43:32 GMT
2020-04-22T10:43:33.3395838Z 
2020-04-22T10:43:33.3395838Z 
2020-04-22T10:43:33.3473696Z ##[error]Bash exited with code '1'.
2020-04-22T10:43:33.3488497Z ##[section]Finishing: Run build
2020-04-22T10:43:33.3538769Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T10:43:33.3544934Z Task         : Get sources
2020-04-22T10:43:33.3545268Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T10:43:33.3545553Z Version      : 1.0.0
2020-04-22T10:43:33.3545776Z Author       : Microsoft
2020-04-22T10:43:33.3545776Z Author       : Microsoft
2020-04-22T10:43:33.3546110Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T10:43:33.3546474Z ==============================================================================
2020-04-22T10:43:33.6905494Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T10:43:33.6949920Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T10:43:33.7044182Z Cleaning up task key
2020-04-22T10:43:33.7046243Z Start cleaning up orphan processes.
2020-04-22T10:43:33.7236508Z Terminate orphan process: pid (4007) (python)
2020-04-22T10:43:33.7399875Z ##[section]Finishing: Finalize Job
