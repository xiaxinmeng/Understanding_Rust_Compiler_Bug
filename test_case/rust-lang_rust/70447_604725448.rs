plain
2020-03-26T21:46:38.5442305Z ========================== Starting Command Output ===========================
2020-03-26T21:46:38.5446381Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bd5eb471-157f-43dd-b829-e77b86d8ad8b.sh
2020-03-26T21:46:38.5447232Z 
2020-03-26T21:46:38.5453172Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-26T21:46:38.5483898Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70447/merge to s
2020-03-26T21:46:38.5487513Z Task         : Get sources
2020-03-26T21:46:38.5487859Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T21:46:38.5488239Z Version      : 1.0.0
2020-03-26T21:46:38.5488453Z Author       : Microsoft
---
2020-03-26T21:46:40.1649836Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-26T21:46:40.1661864Z ##[command]git config gc.auto 0
2020-03-26T21:46:40.1669666Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-26T21:46:40.1676764Z ##[command]git config --get-all http.proxy
2020-03-26T21:46:40.1686574Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70447/merge:refs/remotes/pull/70447/merge
---
2020-03-26T21:54:38.5086979Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-26T21:54:50.5184884Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-26T21:54:56.5438617Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-26T21:55:12.5934828Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-26T21:55:14.1555169Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-26T21:55:18.3286164Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-26T21:55:56.8942093Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-26T21:56:06.8492580Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-26T21:57:05.3341401Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-26T22:21:04.4317657Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-26T22:21:06.3264101Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-26T22:21:18.8198360Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-26T22:21:33.0175765Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-26T22:21:39.3601807Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-26T22:21:41.1913534Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-26T22:22:31.7093910Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-26T22:22:43.6431975Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-26T22:23:57.7681357Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-26T22:39:12.6580612Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-03-26T22:40:01.6880566Z    Compiling rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-03-26T22:42:28.5958537Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-03-26T22:43:11.2494965Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-03-26T22:43:34.9418369Z thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-03-26T22:43:34.9418770Z   left: `_1`,
2020-03-26T22:43:34.9419289Z  right: `_1`', src/librustc_mir/transform/generator.rs:111:9
2020-03-26T22:43:34.9428526Z 
2020-03-26T22:43:34.9432709Z error: internal compiler error: unexpected panic
2020-03-26T22:43:34.9432966Z 
2020-03-26T22:43:34.9433716Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-26T22:43:34.9433716Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-26T22:43:34.9433937Z 
2020-03-26T22:43:34.9467856Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-26T22:43:34.9468923Z note: rustc 1.44.0-nightly (4c367df91 2020-03-26) running on x86_64-unknown-linux-gnu
2020-03-26T22:43:34.9469363Z 
2020-03-26T22:43:34.9469363Z 
2020-03-26T22:43:34.9470444Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-26T22:43:34.9471308Z note: some of the compiler flags provided by cargo are hidden
2020-03-26T22:43:34.9471553Z 
2020-03-26T22:43:34.9641314Z error: could not compile `rustc_interface`.
2020-03-26T22:43:34.9641603Z 
---
2020-03-26T22:44:43.4588034Z   local time: Thu Mar 26 22:44:43 UTC 2020
2020-03-26T22:44:43.7570057Z   network time: Thu, 26 Mar 2020 22:44:43 GMT
2020-03-26T22:44:43.7573771Z == end clock drift check ==
2020-03-26T22:44:44.3655858Z 
2020-03-26T22:44:44.3736873Z ##[error]Bash exited with code '1'.
2020-03-26T22:44:44.3754205Z ##[section]Finishing: Run build
2020-03-26T22:44:44.3805633Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70447/merge to s
2020-03-26T22:44:44.3812514Z Task         : Get sources
2020-03-26T22:44:44.3813135Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T22:44:44.3815104Z Version      : 1.0.0
2020-03-26T22:44:44.3815552Z Author       : Microsoft
2020-03-26T22:44:44.3815552Z Author       : Microsoft
2020-03-26T22:44:44.3815981Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-26T22:44:44.3816445Z ==============================================================================
2020-03-26T22:44:44.7612572Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-26T22:44:44.7674095Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70447/merge to s
2020-03-26T22:44:44.7782581Z Cleaning up task key
2020-03-26T22:44:44.7783962Z Start cleaning up orphan processes.
2020-03-26T22:44:44.8023023Z Terminate orphan process: pid (3803) (python)
2020-03-26T22:44:44.8225179Z ##[section]Finishing: Finalize Job
