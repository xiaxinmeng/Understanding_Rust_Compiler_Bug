plain
2020-04-18T21:45:10.0380753Z ========================== Starting Command Output ===========================
2020-04-18T21:45:10.0386027Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2825a1f4-238d-4556-ae83-43f57a2e6e9b.sh
2020-04-18T21:45:10.0386500Z 
2020-04-18T21:45:10.0391372Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T21:45:10.0411901Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T21:45:10.0415454Z Task         : Get sources
2020-04-18T21:45:10.0415756Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T21:45:10.0416049Z Version      : 1.0.0
2020-04-18T21:45:10.0416249Z Author       : Microsoft
---
2020-04-18T21:45:11.6574217Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T21:45:11.6586087Z ##[command]git config gc.auto 0
2020-04-18T21:45:11.6592359Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T21:45:11.6598890Z ##[command]git config --get-all http.proxy
2020-04-18T21:45:11.6612889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71288/merge:refs/remotes/pull/71288/merge
---
2020-04-18T21:47:41.4947874Z  ---> 78ad2f4d4aca
2020-04-18T21:47:41.4948263Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-18T21:47:41.4948757Z  ---> Using cache
2020-04-18T21:47:41.4949111Z  ---> 4d2dc61c4d00
2020-04-18T21:47:41.4950521Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-18T21:47:41.4951933Z  ---> 776b6266a8b7
2020-04-18T21:47:41.4991570Z Successfully built 776b6266a8b7
2020-04-18T21:47:41.5043526Z Successfully tagged rust-ci:latest
2020-04-18T21:47:41.5382323Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-18T21:47:41.5382323Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-18T21:47:41.5402889Z Looks like docker image is the same as before, not uploading
2020-04-18T21:47:48.1513417Z [CI_JOB_NAME=mingw-check]
2020-04-18T21:47:48.1793656Z [CI_JOB_NAME=mingw-check]
2020-04-18T21:47:48.1835050Z == clock drift check ==
2020-04-18T21:47:48.1846065Z   local time: Sat Apr 18 21:47:48 UTC 2020
2020-04-18T21:47:48.3771622Z   network time: Sat, 18 Apr 2020 21:47:48 GMT
2020-04-18T21:47:48.3796631Z Starting sccache server...
2020-04-18T21:47:48.5089259Z configure: processing command line
2020-04-18T21:47:48.5089830Z configure: 
2020-04-18T21:47:48.5090748Z configure: rust.parallel-compiler := True
---
2020-04-18T21:51:49.4234409Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T21:51:49.4422688Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T21:51:49.6404985Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T21:51:49.8574616Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T21:51:50.3119355Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T21:51:52.7453893Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T21:51:53.2628699Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T21:51:55.4463234Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T21:51:55.9218960Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T21:53:50.2729617Z configure: rust.debug-assertions := True
2020-04-18T21:53:50.2730113Z configure: rust.codegen-units-std := 1
2020-04-18T21:53:50.2730576Z configure: rust.verify-llvm-ir  := True
2020-04-18T21:53:50.2730864Z configure: build.submodules     := False
2020-04-18T21:53:50.2731503Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-18T21:53:50.2732062Z configure: writing `config.toml` in current directory
2020-04-18T21:53:50.2732323Z configure: 
2020-04-18T21:53:50.2732745Z configure: run `python /checkout/x.py --help`
2020-04-18T21:53:50.2732968Z configure: 
---
2020-04-18T21:55:27.0029520Z Hugepagesize:       2048 kB
2020-04-18T21:55:27.0029745Z DirectMap4k:      139200 kB
2020-04-18T21:55:27.0029969Z DirectMap2M:     4055040 kB
2020-04-18T21:55:27.0030210Z DirectMap1G:     5242880 kB
2020-04-18T21:55:27.0046507Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-18T21:55:28.5253114Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-18T21:55:28.5253114Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-18T21:55:28.5259122Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-18T21:55:28.7604030Z    Compiling unicode-xid v0.2.0
2020-04-18T21:55:28.8805013Z    Compiling syn v1.0.11
2020-04-18T21:55:29.7936421Z    Compiling linked-hash-map v0.5.2
2020-04-18T21:55:29.8399362Z    Compiling lazy_static v1.4.0
2020-04-18T21:55:29.8399362Z    Compiling lazy_static v1.4.0
2020-04-18T21:55:30.0524435Z    Compiling yaml-rust v0.4.3
2020-04-18T21:55:34.7350369Z    Compiling quote v1.0.2
2020-04-18T21:55:50.4377022Z    Compiling thiserror-impl v1.0.5
2020-04-18T21:55:55.5440668Z    Compiling thiserror v1.0.5
2020-04-18T21:55:55.6017525Z    Compiling yaml-merge-keys v0.4.0
2020-04-18T21:55:56.8634947Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-18T21:55:58.5758725Z Build completed successfully in 0:00:31
2020-04-18T21:55:58.5843605Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-18T21:55:58.8742381Z     Finished dev [unoptimized] target(s) in 0.20s
2020-04-18T21:56:00.0458686Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-18T21:58:15.4778155Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T21:58:15.8985075Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T21:58:15.9613439Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T21:58:16.1090360Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T21:58:16.6182420Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T21:58:19.2050391Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T21:58:19.7141828Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T21:58:21.8685613Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T21:58:22.3544964Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T22:02:16.7992666Z    Compiling cargo_metadata v0.9.1
2020-04-18T22:02:21.5498945Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-18T22:02:32.7803176Z     Finished release [optimized] target(s) in 26.47s
2020-04-18T22:02:32.7893286Z tidy check
2020-04-18T22:02:38.7122584Z tidy error: /checkout/src/test/ui/parser/mod_file_not_exist.rs:5: line longer than 100 chars
2020-04-18T22:02:38.8647130Z tidy error: /checkout/src/test/ui/parser/mod_file_not_exist_windows.rs:5: line longer than 100 chars
2020-04-18T22:02:41.9727585Z some tidy checks failed
2020-04-18T22:02:41.9731387Z Found 490 error codes
2020-04-18T22:02:41.9731925Z Found 0 error codes with no tests
2020-04-18T22:02:41.9732339Z Done!
2020-04-18T22:02:41.9732339Z Done!
2020-04-18T22:02:41.9732697Z 
2020-04-18T22:02:41.9733029Z 
2020-04-18T22:02:41.9737419Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-18T22:02:41.9738902Z 
2020-04-18T22:02:41.9739220Z 
2020-04-18T22:02:41.9742390Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-18T22:02:41.9743087Z Build completed unsuccessfully in 0:00:37
2020-04-18T22:02:41.9743087Z Build completed unsuccessfully in 0:00:37
2020-04-18T22:02:41.9851081Z == clock drift check ==
2020-04-18T22:02:41.9871074Z   local time: Sat Apr 18 22:02:41 UTC 2020
2020-04-18T22:02:42.1072659Z   network time: Sat, 18 Apr 2020 22:02:42 GMT
2020-04-18T22:02:43.6519225Z 
2020-04-18T22:02:43.6519225Z 
2020-04-18T22:02:43.6604611Z ##[error]Bash exited with code '1'.
2020-04-18T22:02:43.6628852Z ##[section]Finishing: Run build
2020-04-18T22:02:43.6709377Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T22:02:43.6715763Z Task         : Get sources
2020-04-18T22:02:43.6716120Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T22:02:43.6716443Z Version      : 1.0.0
2020-04-18T22:02:43.6716701Z Author       : Microsoft
2020-04-18T22:02:43.6716701Z Author       : Microsoft
2020-04-18T22:02:43.6717056Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T22:02:43.6717450Z ==============================================================================
2020-04-18T22:02:44.0401199Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T22:02:44.0444185Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T22:02:44.0539256Z Cleaning up task key
2020-04-18T22:02:44.0540557Z Start cleaning up orphan processes.
2020-04-18T22:02:44.0740786Z Terminate orphan process: pid (4392) (python)
2020-04-18T22:02:44.0979776Z ##[section]Finishing: Finalize Job
