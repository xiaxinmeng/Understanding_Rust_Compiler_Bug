plain
2020-04-07T20:11:51.7055764Z ========================== Starting Command Output ===========================
2020-04-07T20:11:51.7058389Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/21a85997-edfa-4495-a958-9a4c4c6ada59.sh
2020-04-07T20:11:51.7058695Z 
2020-04-07T20:11:51.7062625Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T20:11:51.7082019Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70899/merge to s
2020-04-07T20:11:51.7085344Z Task         : Get sources
2020-04-07T20:11:51.7085998Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T20:11:51.7086318Z Version      : 1.0.0
2020-04-07T20:11:51.7086534Z Author       : Microsoft
---
2020-04-07T20:11:52.7007964Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T20:11:52.7013631Z ##[command]git config gc.auto 0
2020-04-07T20:11:52.7017772Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T20:11:52.7022320Z ##[command]git config --get-all http.proxy
2020-04-07T20:11:52.7029755Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70899/merge:refs/remotes/pull/70899/merge
---
2020-04-07T20:14:27.2212587Z  ---> 3fc1b512c57b
2020-04-07T20:14:27.2212845Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-07T20:14:27.2216696Z  ---> Using cache
2020-04-07T20:14:27.2217057Z  ---> 5ee4295733f4
2020-04-07T20:14:27.2220497Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-07T20:14:27.2222112Z  ---> 3d07a0fa42fe
2020-04-07T20:14:27.2289518Z Successfully built 3d07a0fa42fe
2020-04-07T20:14:27.2317829Z Successfully tagged rust-ci:latest
2020-04-07T20:14:27.2600269Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-07T20:14:27.2600269Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-07T20:14:27.2616962Z Looks like docker image is the same as before, not uploading
2020-04-07T20:14:32.7373522Z [CI_JOB_NAME=mingw-check]
2020-04-07T20:14:32.7603784Z [CI_JOB_NAME=mingw-check]
2020-04-07T20:14:32.7631258Z == clock drift check ==
2020-04-07T20:14:32.7640962Z   local time: Tue Apr  7 20:14:32 UTC 2020
2020-04-07T20:14:32.8268761Z   network time: Tue, 07 Apr 2020 20:14:32 GMT
2020-04-07T20:14:32.8297138Z Starting sccache server...
2020-04-07T20:14:32.9220032Z configure: processing command line
2020-04-07T20:14:32.9220793Z configure: 
2020-04-07T20:14:32.9221898Z configure: rust.parallel-compiler := True
---
2020-04-07T20:18:05.5545722Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T20:18:05.7879195Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T20:18:05.9545211Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T20:18:05.9931320Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T20:18:06.5865205Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T20:18:08.8071736Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T20:18:09.3096083Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T20:18:11.2963100Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T20:18:11.7386512Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T20:19:56.7541363Z configure: build.locked-deps    := True
2020-04-07T20:19:56.7541674Z configure: llvm.ccache          := sccache
2020-04-07T20:19:56.7542177Z configure: build.cargo-native-static := True
2020-04-07T20:19:56.7542689Z configure: dist.missing-tools   := True
2020-04-07T20:19:56.7543316Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-07T20:19:56.7544076Z configure: writing `config.toml` in current directory
2020-04-07T20:19:56.7544347Z configure: 
2020-04-07T20:19:56.7544801Z configure: run `python /checkout/x.py --help`
2020-04-07T20:19:56.7545316Z configure: 
---
2020-04-07T20:21:19.9050051Z Hugepagesize:       2048 kB
2020-04-07T20:21:19.9050305Z DirectMap4k:      141248 kB
2020-04-07T20:21:19.9050542Z DirectMap2M:     5101568 kB
2020-04-07T20:21:19.9050964Z DirectMap1G:     4194304 kB
2020-04-07T20:21:19.9051779Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-07T20:21:21.2110657Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-07T20:21:21.2110657Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-07T20:21:21.2112152Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-07T20:21:21.4481073Z    Compiling unicode-xid v0.2.0
2020-04-07T20:21:21.5727436Z    Compiling syn v1.0.11
2020-04-07T20:21:22.4173745Z    Compiling linked-hash-map v0.5.2
2020-04-07T20:21:22.4532243Z    Compiling lazy_static v1.4.0
2020-04-07T20:21:22.4532243Z    Compiling lazy_static v1.4.0
2020-04-07T20:21:22.6505901Z    Compiling yaml-rust v0.4.3
2020-04-07T20:21:26.9011550Z    Compiling quote v1.0.2
2020-04-07T20:21:41.1849601Z    Compiling thiserror-impl v1.0.5
2020-04-07T20:21:45.8699912Z    Compiling thiserror v1.0.5
2020-04-07T20:21:45.9305498Z    Compiling yaml-merge-keys v0.4.0
2020-04-07T20:21:47.1169560Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-07T20:21:49.4032335Z Build completed successfully in 0:00:28
2020-04-07T20:21:49.4033910Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-07T20:21:49.4034611Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-07T20:21:50.0891804Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-07T20:23:53.8976452Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T20:23:54.1388306Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T20:23:54.3087106Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T20:23:54.3441902Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T20:23:54.9106775Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T20:23:57.0783109Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T20:23:57.5616437Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T20:23:59.6218551Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T20:24:00.0571582Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T20:27:37.7927159Z    Compiling cargo_metadata v0.9.1
2020-04-07T20:27:41.4033832Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-07T20:27:52.1972456Z     Finished release [optimized] target(s) in 24.36s
2020-04-07T20:27:52.2082423Z tidy check
2020-04-07T20:27:58.3838550Z tidy error: /checkout/src/ci/scripts/install-mingw.sh:65: TODO is deprecated; use FIXME
2020-04-07T20:28:00.7359007Z Found 491 error codes
2020-04-07T20:28:00.7359328Z Found 0 error codes with no tests
2020-04-07T20:28:00.7359534Z Done!
2020-04-07T20:28:00.7359731Z some tidy checks failed
2020-04-07T20:28:00.7359731Z some tidy checks failed
2020-04-07T20:28:00.7359883Z 
2020-04-07T20:28:00.7359987Z 
2020-04-07T20:28:00.7361264Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-07T20:28:00.7362258Z 
2020-04-07T20:28:00.7362361Z 
2020-04-07T20:28:00.7362655Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-07T20:28:00.7363033Z Build completed unsuccessfully in 0:00:34
2020-04-07T20:28:00.7363033Z Build completed unsuccessfully in 0:00:34
2020-04-07T20:28:00.7391189Z == clock drift check ==
2020-04-07T20:28:00.7399897Z   local time: Tue Apr  7 20:28:00 UTC 2020
2020-04-07T20:28:01.0315246Z   network time: Tue, 07 Apr 2020 20:28:01 GMT
2020-04-07T20:28:02.6616159Z 
2020-04-07T20:28:02.6616159Z 
2020-04-07T20:28:02.6685507Z ##[error]Bash exited with code '1'.
2020-04-07T20:28:02.6700312Z ##[section]Finishing: Run build
2020-04-07T20:28:02.6751047Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70899/merge to s
2020-04-07T20:28:02.6756140Z Task         : Get sources
2020-04-07T20:28:02.6756541Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T20:28:02.6756883Z Version      : 1.0.0
2020-04-07T20:28:02.6757121Z Author       : Microsoft
2020-04-07T20:28:02.6757121Z Author       : Microsoft
2020-04-07T20:28:02.6757516Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T20:28:02.6757952Z ==============================================================================
2020-04-07T20:28:03.0141438Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T20:28:03.0183849Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70899/merge to s
2020-04-07T20:28:03.0275213Z Cleaning up task key
2020-04-07T20:28:03.0276523Z Start cleaning up orphan processes.
2020-04-07T20:28:03.0481905Z Terminate orphan process: pid (3458) (python)
2020-04-07T20:28:03.0665824Z ##[section]Finishing: Finalize Job
