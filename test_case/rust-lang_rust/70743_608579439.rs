plain
2020-04-03T17:26:40.5612343Z ========================== Starting Command Output ===========================
2020-04-03T17:26:40.5615526Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/23a5e3fe-dbee-463d-a56b-4f3108ad32a7.sh
2020-04-03T17:26:40.5615874Z 
2020-04-03T17:26:40.5619597Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-03T17:26:40.5642642Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70743/merge to s
2020-04-03T17:26:40.5645862Z Task         : Get sources
2020-04-03T17:26:40.5646147Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T17:26:40.5646632Z Version      : 1.0.0
2020-04-03T17:26:40.5646888Z Author       : Microsoft
---
2020-04-03T17:26:41.7571316Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-03T17:26:41.7583842Z ##[command]git config gc.auto 0
2020-04-03T17:26:41.7589116Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-03T17:26:41.7592591Z ##[command]git config --get-all http.proxy
2020-04-03T17:26:41.7600099Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70743/merge:refs/remotes/pull/70743/merge
---
2020-04-03T17:28:29.1465457Z Looks like docker image is the same as before, not uploading
2020-04-03T17:28:36.7994395Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-03T17:28:36.8293756Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-03T17:28:36.8323517Z == clock drift check ==
2020-04-03T17:28:36.8346297Z   local time: Fri Apr  3 17:28:36 UTC 2020
2020-04-03T17:28:36.9088326Z   network time: Fri, 03 Apr 2020 17:28:36 GMT
2020-04-03T17:28:36.9115053Z Starting sccache server...
2020-04-03T17:28:36.9985764Z configure: processing command line
2020-04-03T17:28:36.9987615Z configure: 
2020-04-03T17:28:36.9988963Z configure: rust.dist-src        := False
---
2020-04-03T17:34:07.0323970Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-03T17:34:08.6971052Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-03T17:34:10.4411934Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-03T17:34:12.5343966Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-03T17:34:21.5849970Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-03T17:34:24.9748819Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-03T17:34:29.8941193Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-03T17:34:34.4436961Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-03T17:34:44.1139123Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-03T17:56:32.4343552Z    Compiling rustc_index v0.0.0 (/checkout/src/librustc_index)
2020-04-03T17:56:37.5798749Z error: failed to run custom build command for `backtrace-sys v0.1.35`
2020-04-03T17:56:37.5799279Z 
2020-04-03T17:56:37.5799424Z Caused by:
2020-04-03T17:56:37.5819851Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-716460570fac9848/build-script-build` (exit code: 1)
2020-04-03T17:56:37.5820559Z --- stdout
2020-04-03T17:56:37.5820918Z cargo:rustc-cfg=rbt
2020-04-03T17:56:37.5821336Z --- stderr
2020-04-03T17:56:37.5821456Z 
2020-04-03T17:56:37.5821556Z 
2020-04-03T17:56:37.5821793Z error occurred: Getting object file details failed.
---
2020-04-03T17:56:44.0871613Z expected success, got: exit code: 101
2020-04-03T17:56:44.0880211Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-03T17:56:44.0880642Z Build completed unsuccessfully in 0:26:28
2020-04-03T17:56:44.0930853Z == clock drift check ==
2020-04-03T17:56:44.0951528Z   local time: Fri Apr  3 17:56:44 UTC 2020
2020-04-03T17:56:44.3667275Z   network time: Fri, 03 Apr 2020 17:56:44 GMT
2020-04-03T17:56:45.2622873Z 
2020-04-03T17:56:45.2622873Z 
2020-04-03T17:56:45.2700396Z ##[error]Bash exited with code '1'.
2020-04-03T17:56:45.2716997Z ##[section]Finishing: Run build
2020-04-03T17:56:45.2771464Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70743/merge to s
2020-04-03T17:56:45.2777225Z Task         : Get sources
2020-04-03T17:56:45.2777604Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T17:56:45.2777948Z Version      : 1.0.0
2020-04-03T17:56:45.2778205Z Author       : Microsoft
2020-04-03T17:56:45.2778205Z Author       : Microsoft
2020-04-03T17:56:45.2778590Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-03T17:56:45.2779035Z ==============================================================================
2020-04-03T17:56:45.6434723Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-03T17:56:45.6484479Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70743/merge to s
2020-04-03T17:56:45.6581172Z Cleaning up task key
2020-04-03T17:56:45.6582567Z Start cleaning up orphan processes.
2020-04-03T17:56:45.6784268Z Terminate orphan process: pid (4106) (python)
2020-04-03T17:56:45.6976762Z ##[section]Finishing: Finalize Job
