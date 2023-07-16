plain
2020-04-24T22:41:34.0420369Z ========================== Starting Command Output ===========================
2020-04-24T22:41:34.0423847Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6ec5ee5b-e0f7-4ab9-850b-3becc06c013c.sh
2020-04-24T22:41:34.0424334Z 
2020-04-24T22:41:34.0429254Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T22:41:34.0447660Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71537/merge to s
2020-04-24T22:41:34.0450787Z Task         : Get sources
2020-04-24T22:41:34.0451070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T22:41:34.0451344Z Version      : 1.0.0
2020-04-24T22:41:34.0451552Z Author       : Microsoft
---
2020-04-24T22:41:35.0372727Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T22:41:35.0378489Z ##[command]git config gc.auto 0
2020-04-24T22:41:35.0382055Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T22:41:35.0385328Z ##[command]git config --get-all http.proxy
2020-04-24T22:41:35.0391325Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71537/merge:refs/remotes/pull/71537/merge
---
2020-04-24T22:44:05.8229608Z  ---> f7353ccad5b1
2020-04-24T22:44:05.8229827Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T22:44:05.8230200Z  ---> Using cache
2020-04-24T22:44:05.8230517Z  ---> ed38efbaa060
2020-04-24T22:44:05.8231740Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T22:44:05.8232992Z  ---> c5008ef7ae8e
2020-04-24T22:44:05.8259121Z Successfully built c5008ef7ae8e
2020-04-24T22:44:05.8284577Z Successfully tagged rust-ci:latest
2020-04-24T22:44:05.8539297Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T22:44:05.8539297Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T22:44:05.8553928Z Looks like docker image is the same as before, not uploading
2020-04-24T22:44:13.7224312Z [CI_JOB_NAME=mingw-check]
2020-04-24T22:44:13.7461772Z [CI_JOB_NAME=mingw-check]
2020-04-24T22:44:13.7485721Z == clock drift check ==
2020-04-24T22:44:13.7495151Z   local time: Fri Apr 24 22:44:13 UTC 2020
2020-04-24T22:44:13.7900818Z   network time: Fri, 24 Apr 2020 22:44:13 GMT
2020-04-24T22:44:13.7926543Z Starting sccache server...
2020-04-24T22:44:13.9010565Z configure: processing command line
2020-04-24T22:44:13.9011259Z configure: 
2020-04-24T22:44:13.9012292Z configure: rust.parallel-compiler := True
---
2020-04-24T22:47:58.3576068Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T22:47:58.5004023Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T22:47:58.6823407Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T22:47:58.7521424Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T22:47:59.2568683Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T22:48:01.4198102Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T22:48:01.8776337Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T22:48:03.7974436Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T22:48:04.2085346Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T22:49:47.4641108Z configure: llvm.ccache          := sccache
2020-04-24T22:49:47.4642119Z configure: rust.debug-assertions := True
2020-04-24T22:49:47.4643513Z configure: dist.missing-tools   := True
2020-04-24T22:49:47.4644240Z configure: build.submodules     := False
2020-04-24T22:49:47.4645466Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-24T22:49:47.4646818Z configure: writing `config.toml` in current directory
2020-04-24T22:49:47.4647426Z configure: 
2020-04-24T22:49:47.4648329Z configure: run `python /checkout/x.py --help`
2020-04-24T22:49:47.4648955Z configure: 
---
2020-04-24T22:51:18.1639481Z Hugepagesize:       2048 kB
2020-04-24T22:51:18.1639702Z DirectMap4k:      147392 kB
2020-04-24T22:51:18.1639904Z DirectMap2M:     2998272 kB
2020-04-24T22:51:18.1640106Z DirectMap1G:     6291456 kB
2020-04-24T22:51:18.1653017Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-24T22:51:19.4774066Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T22:51:19.4774066Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T22:51:19.4780930Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-24T22:51:19.7025402Z    Compiling unicode-xid v0.2.0
2020-04-24T22:51:19.8248021Z    Compiling syn v1.0.11
2020-04-24T22:51:20.6120726Z    Compiling linked-hash-map v0.5.2
2020-04-24T22:51:20.6415597Z    Compiling lazy_static v1.4.0
2020-04-24T22:51:20.6415597Z    Compiling lazy_static v1.4.0
2020-04-24T22:51:20.8389996Z    Compiling yaml-rust v0.4.3
2020-04-24T22:51:24.9232430Z    Compiling quote v1.0.2
2020-04-24T22:51:38.8169716Z    Compiling thiserror-impl v1.0.5
2020-04-24T22:51:43.3336957Z    Compiling thiserror v1.0.5
2020-04-24T22:51:43.3969524Z    Compiling yaml-merge-keys v0.4.0
2020-04-24T22:51:44.4858272Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-24T22:51:45.9999131Z Build completed successfully in 0:00:27
2020-04-24T22:51:46.0088813Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-24T22:51:46.2709060Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-24T22:51:47.3148789Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-24T22:53:47.2267004Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T22:53:47.4307031Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T22:53:47.6285936Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T22:53:47.6381421Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T22:53:48.2206190Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T22:53:50.4031956Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T22:53:50.9067870Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T22:53:52.9417079Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T22:53:53.4010451Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-24T22:53:53.4010451Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-24T22:53:54.5904412Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-24T22:53:55.3026092Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-24T22:53:56.8815268Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-24T22:54:11.2981609Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-04-24T22:54:11.5896599Z error: unused import: `GetModuleHandleExW`
2020-04-24T22:54:11.5897274Z    --> src/librustc_metadata/dynamic_lib.rs:115:49
2020-04-24T22:54:11.5897786Z     |
2020-04-24T22:54:11.5899005Z 115 |     use winapi::um::libloaderapi::{FreeLibrary, GetModuleHandleExW, GetProcAddress, LoadLibraryW};
2020-04-24T22:54:11.5900416Z     |
2020-04-24T22:54:11.5900996Z     = note: `-D unused-imports` implied by `-D warnings`
2020-04-24T22:54:11.5901291Z 
2020-04-24T22:54:12.5528218Z error: aborting due to previous error
2020-04-24T22:54:12.5528218Z error: aborting due to previous error
2020-04-24T22:54:12.5528485Z 
2020-04-24T22:54:12.5607031Z error: could not compile `rustc_metadata`.
2020-04-24T22:54:12.5610973Z 
2020-04-24T22:54:12.5611588Z To learn more, run the command again with --verbose.
2020-04-24T22:54:12.5631015Z warning: build failed, waiting for other jobs to finish...
2020-04-24T22:54:14.1279803Z error: build failed
2020-04-24T22:54:14.1301207Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-24T22:54:14.1307978Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-24T22:54:14.1308390Z Build completed unsuccessfully in 0:02:28
2020-04-24T22:54:14.1417909Z == clock drift check ==
2020-04-24T22:54:14.1417909Z == clock drift check ==
2020-04-24T22:54:14.1432058Z   local time: Fri Apr 24 22:54:14 UTC 2020
2020-04-24T22:54:14.4317005Z   network time: Fri, 24 Apr 2020 22:54:14 GMT
2020-04-24T22:54:15.1837677Z 
2020-04-24T22:54:15.1837677Z 
2020-04-24T22:54:15.1910306Z ##[error]Bash exited with code '1'.
2020-04-24T22:54:15.1924423Z ##[section]Finishing: Run build
2020-04-24T22:54:15.1968176Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71537/merge to s
2020-04-24T22:54:15.1973208Z Task         : Get sources
2020-04-24T22:54:15.1973528Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T22:54:15.1973814Z Version      : 1.0.0
2020-04-24T22:54:15.1974037Z Author       : Microsoft
2020-04-24T22:54:15.1974037Z Author       : Microsoft
2020-04-24T22:54:15.1974372Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T22:54:15.1974743Z ==============================================================================
2020-04-24T22:54:15.5297835Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T22:54:15.5343989Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71537/merge to s
2020-04-24T22:54:15.5448486Z Cleaning up task key
2020-04-24T22:54:15.5449777Z Start cleaning up orphan processes.
2020-04-24T22:54:15.5658938Z Terminate orphan process: pid (4195) (python)
2020-04-24T22:54:15.5891000Z ##[section]Finishing: Finalize Job
