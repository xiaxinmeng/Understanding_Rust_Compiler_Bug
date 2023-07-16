plain
2020-04-02T07:20:05.7142906Z ========================== Starting Command Output ===========================
2020-04-02T07:20:05.7146377Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/822f1598-9e46-4013-9af3-411415249b5e.sh
2020-04-02T07:20:05.7146776Z 
2020-04-02T07:20:05.7151018Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T07:20:05.7169856Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70633/merge to s
2020-04-02T07:20:05.7173089Z Task         : Get sources
2020-04-02T07:20:05.7173415Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T07:20:05.7173715Z Version      : 1.0.0
2020-04-02T07:20:05.7173916Z Author       : Microsoft
---
2020-04-02T07:20:06.7064956Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T07:20:06.7070419Z ##[command]git config gc.auto 0
2020-04-02T07:20:06.7074030Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T07:20:06.7077391Z ##[command]git config --get-all http.proxy
2020-04-02T07:20:06.7083409Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70633/merge:refs/remotes/pull/70633/merge
---
2020-04-02T07:22:10.4817877Z  ---> 3fc1b512c57b
2020-04-02T07:22:10.4818171Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-02T07:22:10.4822009Z  ---> Using cache
2020-04-02T07:22:10.4822478Z  ---> 5ee4295733f4
2020-04-02T07:22:10.4824139Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-02T07:22:10.4827901Z  ---> 3d07a0fa42fe
2020-04-02T07:22:10.4872309Z Successfully built 3d07a0fa42fe
2020-04-02T07:22:10.4934252Z Successfully tagged rust-ci:latest
2020-04-02T07:22:10.5219923Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-02T07:22:10.5219923Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-02T07:22:10.5235713Z Looks like docker image is the same as before, not uploading
2020-04-02T07:22:15.9402785Z [CI_JOB_NAME=mingw-check]
2020-04-02T07:22:15.9614623Z [CI_JOB_NAME=mingw-check]
2020-04-02T07:22:15.9643928Z == clock drift check ==
2020-04-02T07:22:15.9653830Z   local time: Thu Apr  2 07:22:15 UTC 2020
2020-04-02T07:22:16.1251143Z   network time: Thu, 02 Apr 2020 07:22:16 GMT
2020-04-02T07:22:16.1277550Z Starting sccache server...
2020-04-02T07:22:16.2153696Z configure: processing command line
2020-04-02T07:22:16.2154024Z configure: 
2020-04-02T07:22:16.2154978Z configure: rust.parallel-compiler := True
---
2020-04-02T07:25:36.6697857Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T07:25:36.8103852Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T07:25:36.9927731Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T07:25:37.0434268Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T07:25:37.5654000Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T07:25:39.6854371Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T07:25:40.1374865Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T07:25:41.9826035Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T07:25:42.4013503Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T07:27:22.2885490Z configure: build.locked-deps    := True
2020-04-02T07:27:22.2885931Z configure: llvm.ccache          := sccache
2020-04-02T07:27:22.2886549Z configure: build.cargo-native-static := True
2020-04-02T07:27:22.2887159Z configure: dist.missing-tools   := True
2020-04-02T07:27:22.2887901Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-02T07:27:22.2888738Z configure: writing `config.toml` in current directory
2020-04-02T07:27:22.2889090Z configure: 
2020-04-02T07:27:22.2889620Z configure: run `python /checkout/x.py --help`
2020-04-02T07:27:22.2889989Z configure: 
---
2020-04-02T07:28:39.0567812Z Hugepagesize:       2048 kB
2020-04-02T07:28:39.0568049Z DirectMap4k:      126912 kB
2020-04-02T07:28:39.0568274Z DirectMap2M:     1970176 kB
2020-04-02T07:28:39.0568498Z DirectMap1G:     7340032 kB
2020-04-02T07:28:39.0569821Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-02T07:28:40.3166692Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-02T07:28:40.3166692Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-02T07:28:40.3177841Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-02T07:28:40.5407552Z    Compiling unicode-xid v0.2.0
2020-04-02T07:28:40.6557660Z    Compiling syn v1.0.11
2020-04-02T07:28:41.4233969Z    Compiling linked-hash-map v0.5.2
2020-04-02T07:28:41.4510613Z    Compiling lazy_static v1.4.0
2020-04-02T07:28:41.4510613Z    Compiling lazy_static v1.4.0
2020-04-02T07:28:41.6320901Z    Compiling yaml-rust v0.4.3
2020-04-02T07:28:45.6739728Z    Compiling quote v1.0.2
2020-04-02T07:28:58.9064184Z    Compiling thiserror-impl v1.0.5
2020-04-02T07:29:03.2095599Z    Compiling thiserror v1.0.5
2020-04-02T07:29:03.2703610Z    Compiling yaml-merge-keys v0.4.0
2020-04-02T07:29:04.3340618Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-02T07:29:05.8386025Z Build completed successfully in 0:00:26
2020-04-02T07:29:05.8393764Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-02T07:29:06.0667928Z     Finished dev [unoptimized] target(s) in 0.16s
2020-04-02T07:29:07.1265499Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-02T07:31:05.1849284Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T07:31:05.4353637Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T07:31:05.5650706Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T07:31:05.6290555Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T07:31:06.1238873Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T07:31:08.2146204Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T07:31:08.6565059Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T07:31:10.6767307Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T07:31:11.1146576Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T07:34:34.6881849Z    Compiling cargo_metadata v0.9.1
2020-04-02T07:34:38.0974334Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-02T07:34:48.2259043Z     Finished release [optimized] target(s) in 22.93s
2020-04-02T07:34:48.2360674Z tidy check
2020-04-02T07:34:48.5800103Z tidy error: /checkout/src/test/ui/parser/issue-70583-block-is-empty-2.rs:9: trailing whitespace
2020-04-02T07:34:48.5801020Z tidy error: /checkout/src/test/ui/parser/issue-70583-block-is-empty-2.rs:10: line longer than 100 chars
2020-04-02T07:34:48.5959250Z tidy error: /checkout/src/test/ui/parser/issue-70583-block-is-empty-1.rs:19: trailing whitespace
2020-04-02T07:34:55.8765156Z Found 491 error codes
2020-04-02T07:34:55.8765978Z Found 0 error codes with no tests
2020-04-02T07:34:55.8766399Z Done!
2020-04-02T07:34:55.8771095Z some tidy checks failed
2020-04-02T07:34:55.8771095Z some tidy checks failed
2020-04-02T07:34:55.8774016Z 
2020-04-02T07:34:55.8774346Z 
2020-04-02T07:34:55.8776306Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-02T07:34:55.8777528Z 
2020-04-02T07:34:55.8777796Z 
2020-04-02T07:34:55.8780988Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-02T07:34:55.8781643Z Build completed unsuccessfully in 0:00:31
2020-04-02T07:34:55.8781643Z Build completed unsuccessfully in 0:00:31
2020-04-02T07:34:55.8822358Z == clock drift check ==
2020-04-02T07:34:55.8835992Z   local time: Thu Apr  2 07:34:55 UTC 2020
2020-04-02T07:34:55.9430709Z   network time: Thu, 02 Apr 2020 07:34:55 GMT
2020-04-02T07:34:57.5217839Z 
2020-04-02T07:34:57.5217839Z 
2020-04-02T07:34:57.5279374Z ##[error]Bash exited with code '1'.
2020-04-02T07:34:57.5303949Z ##[section]Finishing: Run build
2020-04-02T07:34:57.5349558Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70633/merge to s
2020-04-02T07:34:57.5354319Z Task         : Get sources
2020-04-02T07:34:57.5354673Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T07:34:57.5355014Z Version      : 1.0.0
2020-04-02T07:34:57.5355247Z Author       : Microsoft
2020-04-02T07:34:57.5355247Z Author       : Microsoft
2020-04-02T07:34:57.5355731Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T07:34:57.5356167Z ==============================================================================
2020-04-02T07:34:57.8468520Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T07:34:57.8519060Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70633/merge to s
2020-04-02T07:34:57.8607746Z Cleaning up task key
2020-04-02T07:34:57.8609037Z Start cleaning up orphan processes.
2020-04-02T07:34:57.8784046Z Terminate orphan process: pid (3688) (python)
2020-04-02T07:34:57.9006983Z ##[section]Finishing: Finalize Job
