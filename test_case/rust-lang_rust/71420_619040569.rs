plain
2020-04-24T13:54:01.4873399Z ========================== Starting Command Output ===========================
2020-04-24T13:54:01.4875858Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/952b3854-45fb-4160-b681-cee1c2f707fe.sh
2020-04-24T13:54:01.4876101Z 
2020-04-24T13:54:01.4882373Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T13:54:01.4900554Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-24T13:54:01.4904026Z Task         : Get sources
2020-04-24T13:54:01.4904303Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T13:54:01.4904570Z Version      : 1.0.0
2020-04-24T13:54:01.4904770Z Author       : Microsoft
---
2020-04-24T13:54:02.5429055Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T13:54:02.5482536Z ##[command]git config gc.auto 0
2020-04-24T13:54:02.5518057Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T13:54:02.5545749Z ##[command]git config --get-all http.proxy
2020-04-24T13:54:02.5637361Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71420/merge:refs/remotes/pull/71420/merge
---
2020-04-24T13:57:32.2645180Z  ---> cb2676f08729
2020-04-24T13:57:32.2645892Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-24T13:57:32.2649234Z  ---> Using cache
2020-04-24T13:57:32.2649581Z  ---> df25ce111862
2020-04-24T13:57:32.2650541Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-24T13:57:32.2655702Z  ---> 599b9ac96b27
2020-04-24T13:57:32.2655933Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-24T13:57:32.2658779Z  ---> Using cache
2020-04-24T13:57:32.2659117Z  ---> 091087e35a36
---
2020-04-24T13:57:32.4135620Z Looks like docker image is the same as before, not uploading
2020-04-24T13:57:41.6099780Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T13:57:41.6366595Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T13:57:41.6398552Z == clock drift check ==
2020-04-24T13:57:41.6407967Z   local time: Fri Apr 24 13:57:41 UTC 2020
2020-04-24T13:57:41.9344118Z   network time: Fri, 24 Apr 2020 13:57:41 GMT
2020-04-24T13:57:41.9373871Z Starting sccache server...
2020-04-24T13:57:42.0245952Z configure: processing command line
2020-04-24T13:57:42.0246359Z configure: 
2020-04-24T13:57:42.0247934Z configure: rust.dist-src        := False
---
2020-04-24T14:02:36.0698533Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T14:02:37.4508672Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T14:02:38.9382129Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T14:02:39.9665800Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T14:02:48.1401867Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T14:02:50.3802700Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T14:02:54.4020982Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T14:02:58.2479521Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T14:03:07.0038527Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T14:21:44.6374736Z    |
2020-04-24T14:21:44.6375247Z 13 | #![feature(specialization)]
2020-04-24T14:21:44.6375804Z    |            ^^^^^^^^^^^^^^
2020-04-24T14:21:44.6376407Z    |
2020-04-24T14:21:44.6376977Z    = note: `-D incomplete-features` implied by `-D warnings`
2020-04-24T14:21:44.6377839Z    = note: see issue #31844 <***/issues/31844> for more information
2020-04-24T14:21:45.5233364Z error: aborting due to previous error
2020-04-24T14:21:45.5233641Z 
2020-04-24T14:21:45.5291157Z error: could not compile `serialize`.
2020-04-24T14:21:45.5291916Z 
---
2020-04-24T14:21:45.6869739Z expected success, got: exit code: 101
2020-04-24T14:21:45.6876616Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-24T14:21:45.6876996Z Build completed unsuccessfully in 0:22:32
2020-04-24T14:21:45.6929216Z == clock drift check ==
2020-04-24T14:21:45.6944787Z   local time: Fri Apr 24 14:21:45 UTC 2020
2020-04-24T14:21:45.9901810Z   network time: Fri, 24 Apr 2020 14:21:46 GMT
2020-04-24T14:21:47.1676867Z 
2020-04-24T14:21:47.1676867Z 
2020-04-24T14:21:47.1749843Z ##[error]Bash exited with code '1'.
2020-04-24T14:21:47.1764097Z ##[section]Finishing: Run build
2020-04-24T14:21:47.1812450Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-24T14:21:47.1818481Z Task         : Get sources
2020-04-24T14:21:47.1818796Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T14:21:47.1819098Z Version      : 1.0.0
2020-04-24T14:21:47.1819310Z Author       : Microsoft
2020-04-24T14:21:47.1819310Z Author       : Microsoft
2020-04-24T14:21:47.1819632Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T14:21:47.1820013Z ==============================================================================
2020-04-24T14:21:47.5175513Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T14:21:47.5222456Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-24T14:21:47.5308896Z Cleaning up task key
2020-04-24T14:21:47.5310169Z Start cleaning up orphan processes.
2020-04-24T14:21:47.5489084Z Terminate orphan process: pid (4131) (python)
2020-04-24T14:21:47.5642678Z ##[section]Finishing: Finalize Job
