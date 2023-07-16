plain
2020-04-14T15:20:54.0028074Z ========================== Starting Command Output ===========================
2020-04-14T15:20:54.0030685Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ad9c2e5e-abdf-4399-be4c-f7aa00b3a52c.sh
2020-04-14T15:20:54.0031058Z 
2020-04-14T15:20:54.0034576Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T15:20:54.0050833Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-14T15:20:54.0053919Z Task         : Get sources
2020-04-14T15:20:54.0054176Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T15:20:54.0054446Z Version      : 1.0.0
2020-04-14T15:20:54.0054616Z Author       : Microsoft
---
2020-04-14T15:20:54.9953488Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T15:20:54.9959063Z ##[command]git config gc.auto 0
2020-04-14T15:20:54.9962568Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T15:20:54.9965779Z ##[command]git config --get-all http.proxy
2020-04-14T15:20:54.9971476Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71091/merge:refs/remotes/pull/71091/merge
---
2020-04-14T15:23:25.8772010Z  ---> f58a2bb1e753
2020-04-14T15:23:25.8774105Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-14T15:23:25.8776055Z  ---> Using cache
2020-04-14T15:23:25.8776485Z  ---> d079cc6b6db8
2020-04-14T15:23:25.8777411Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-14T15:23:25.8778602Z  ---> 4183ca46ee56
2020-04-14T15:23:25.8778915Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-14T15:23:25.8779347Z  ---> Using cache
2020-04-14T15:23:25.8779779Z  ---> 69e7f8a2a2fb
---
2020-04-14T15:23:25.9125430Z Looks like docker image is the same as before, not uploading
2020-04-14T15:23:32.2034624Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T15:23:32.2281063Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T15:23:32.2308983Z == clock drift check ==
2020-04-14T15:23:32.2320448Z   local time: Tue Apr 14 15:23:32 UTC 2020
2020-04-14T15:23:32.3741633Z   network time: Tue, 14 Apr 2020 15:23:32 GMT
2020-04-14T15:23:32.3776071Z Starting sccache server...
2020-04-14T15:23:32.4505625Z configure: processing command line
2020-04-14T15:23:32.4506206Z configure: 
2020-04-14T15:23:32.4507407Z configure: rust.dist-src        := False
---
2020-04-14T15:28:01.6547946Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T15:28:02.9058755Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T15:28:04.2482400Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T15:28:04.8070017Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T15:28:12.9729555Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T15:28:14.5736080Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T15:28:18.3703267Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T15:28:21.9064102Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T15:28:30.7622515Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T15:44:49.9110306Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-04-14T15:44:51.1213287Z error: cannot specialize on trait `io::BufRead`
2020-04-14T15:44:51.1214726Z   --> src/libstd/io/util.rs:40:5
2020-04-14T15:44:51.1215437Z    |
2020-04-14T15:44:51.1216415Z 40 | /     impl<T: BufRead + ?Sized> Copyable for T {
2020-04-14T15:44:51.1217777Z 41 | |         fn copy_to<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<u64> {
2020-04-14T15:44:51.1219028Z 42 | |             let mut written = 0;
2020-04-14T15:44:51.1221051Z ...  |
2020-04-14T15:44:51.1221927Z 62 | |         }
2020-04-14T15:44:51.1222892Z 63 | |     }
2020-04-14T15:44:51.1223711Z    | |_____^
---
2020-04-14T15:44:51.1501978Z expected success, got: exit code: 101
2020-04-14T15:44:51.1514402Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-14T15:44:51.1514773Z Build completed unsuccessfully in 0:19:57
2020-04-14T15:44:51.1569519Z == clock drift check ==
2020-04-14T15:44:51.1585847Z   local time: Tue Apr 14 15:44:51 UTC 2020
2020-04-14T15:44:51.3490809Z   network time: Tue, 14 Apr 2020 15:44:51 GMT
2020-04-14T15:44:53.9111973Z 
2020-04-14T15:44:53.9111973Z 
2020-04-14T15:44:53.9170701Z ##[error]Bash exited with code '1'.
2020-04-14T15:44:53.9190292Z ##[section]Finishing: Run build
2020-04-14T15:44:53.9233807Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-14T15:44:53.9238672Z Task         : Get sources
2020-04-14T15:44:53.9238977Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T15:44:53.9239273Z Version      : 1.0.0
2020-04-14T15:44:53.9239476Z Author       : Microsoft
2020-04-14T15:44:53.9239476Z Author       : Microsoft
2020-04-14T15:44:53.9239797Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T15:44:53.9240171Z ==============================================================================
2020-04-14T15:44:54.2308949Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T15:44:54.2361322Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-14T15:44:54.2442652Z Cleaning up task key
2020-04-14T15:44:54.2443812Z Start cleaning up orphan processes.
2020-04-14T15:44:54.2605155Z Terminate orphan process: pid (3435) (python)
2020-04-14T15:44:54.2744513Z ##[section]Finishing: Finalize Job
