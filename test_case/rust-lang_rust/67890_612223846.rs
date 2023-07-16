plain
2020-04-10T20:57:33.7670462Z ========================== Starting Command Output ===========================
2020-04-10T20:57:33.7676021Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/54cbe80a-f8b2-427a-b926-87bb7e065b9e.sh
2020-04-10T20:57:33.7676545Z 
2020-04-10T20:57:33.7682033Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T20:57:33.7702236Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67890/merge to s
2020-04-10T20:57:33.7705787Z Task         : Get sources
2020-04-10T20:57:33.7706088Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T20:57:33.7706377Z Version      : 1.0.0
2020-04-10T20:57:33.7706685Z Author       : Microsoft
---
2020-04-10T20:57:34.9373226Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T20:57:34.9380653Z ##[command]git config gc.auto 0
2020-04-10T20:57:34.9384934Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T20:57:34.9388957Z ##[command]git config --get-all http.proxy
2020-04-10T20:57:34.9397542Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67890/merge:refs/remotes/pull/67890/merge
---
2020-04-10T21:00:02.2668403Z  ---> 78ad2f4d4aca
2020-04-10T21:00:02.2668649Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-10T21:00:02.2669053Z  ---> Using cache
2020-04-10T21:00:02.2669387Z  ---> 4d2dc61c4d00
2020-04-10T21:00:02.2671246Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-10T21:00:02.2673065Z  ---> 776b6266a8b7
2020-04-10T21:00:02.2673442Z Successfully built 776b6266a8b7
2020-04-10T21:00:02.2708313Z Successfully tagged rust-ci:latest
2020-04-10T21:00:02.3019290Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-10T21:00:02.3019290Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-10T21:00:02.3035619Z Looks like docker image is the same as before, not uploading
2020-04-10T21:00:02.9486974Z [CI_JOB_NAME=mingw-check]
2020-04-10T21:00:02.9791163Z [CI_JOB_NAME=mingw-check]
2020-04-10T21:00:02.9890804Z == clock drift check ==
2020-04-10T21:00:02.9900735Z   local time: Fri Apr 10 21:00:02 UTC 2020
2020-04-10T21:00:03.1907192Z   network time: Fri, 10 Apr 2020 21:00:03 GMT
2020-04-10T21:00:03.1914118Z Starting sccache server...
2020-04-10T21:00:03.3097575Z configure: processing command line
2020-04-10T21:00:03.3097893Z configure: 
2020-04-10T21:00:03.3099007Z configure: rust.parallel-compiler := True
---
2020-04-10T21:04:16.5312919Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T21:04:16.5667775Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T21:04:16.8057849Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T21:04:16.9851065Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T21:04:17.5882879Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T21:04:20.3148220Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T21:04:20.8847269Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T21:04:23.3769665Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T21:04:23.8648095Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T21:06:25.0999121Z configure: build.locked-deps    := True
2020-04-10T21:06:25.1006259Z configure: rust.codegen-units-std := 1
2020-04-10T21:06:25.1007195Z configure: rust.verify-llvm-ir  := True
2020-04-10T21:06:25.1007909Z configure: rust.channel         := nightly
2020-04-10T21:06:25.1008726Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-10T21:06:25.1009540Z configure: writing `config.toml` in current directory
2020-04-10T21:06:25.1009909Z configure: 
2020-04-10T21:06:25.1010456Z configure: run `python /checkout/x.py --help`
2020-04-10T21:06:25.1010840Z configure: 
---
2020-04-10T21:08:08.8462741Z Hugepagesize:       2048 kB
2020-04-10T21:08:08.8462986Z DirectMap4k:      143296 kB
2020-04-10T21:08:08.8463209Z DirectMap2M:     3002368 kB
2020-04-10T21:08:08.8463433Z DirectMap1G:     6291456 kB
2020-04-10T21:08:08.8488748Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-10T21:08:10.3812458Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-10T21:08:10.3812458Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-10T21:08:10.3820239Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-10T21:08:10.6396836Z    Compiling unicode-xid v0.2.0
2020-04-10T21:08:10.7751303Z    Compiling syn v1.0.11
2020-04-10T21:08:11.7436807Z    Compiling linked-hash-map v0.5.2
2020-04-10T21:08:11.7723129Z    Compiling lazy_static v1.4.0
2020-04-10T21:08:11.7723129Z    Compiling lazy_static v1.4.0
2020-04-10T21:08:12.0126543Z    Compiling yaml-rust v0.4.3
2020-04-10T21:08:16.9767120Z    Compiling quote v1.0.2
2020-04-10T21:08:33.1478777Z    Compiling thiserror-impl v1.0.5
2020-04-10T21:08:38.4455374Z    Compiling thiserror v1.0.5
2020-04-10T21:08:38.5088868Z    Compiling yaml-merge-keys v0.4.0
2020-04-10T21:08:39.8246398Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-10T21:08:41.6185781Z Build completed successfully in 0:00:32
2020-04-10T21:08:41.6282337Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-10T21:08:41.9168788Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-10T21:08:43.1083414Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-10T21:11:06.2068757Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T21:11:06.4561652Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T21:11:06.6843773Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T21:11:06.7094714Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T21:11:07.4140490Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T21:11:10.0455405Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T21:11:10.6155380Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T21:11:13.0949501Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T21:11:13.6555460Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T21:15:22.8256654Z    Compiling cargo_metadata v0.9.1
2020-04-10T21:15:27.9109799Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-10T21:15:39.6609544Z     Finished release [optimized] target(s) in 27.82s
2020-04-10T21:15:39.6713994Z tidy check
2020-04-10T21:15:42.6097613Z tidy error: /checkout/src/librustc_feature/active.rs:561: trailing whitespace
2020-04-10T21:15:42.9163265Z tidy error: /checkout/src/librustc_trait_selection/traits/error_reporting/mod.rs:561: trailing whitespace
2020-04-10T21:15:44.1445215Z tidy error: Found 1 features without a gate test.
2020-04-10T21:15:44.1446316Z Expected a gate test for the feature 'lazy_normalization_consts'.
2020-04-10T21:15:44.1447030Z Hint: create a failing test file named 'feature-gate-lazy_normalization_consts.rs'
2020-04-10T21:15:44.1447650Z       in the 'ui' test suite, with its failures due to
2020-04-10T21:15:44.1447990Z       missing usage of `#![feature(lazy_normalization_consts)]`.
2020-04-10T21:15:44.1448856Z Hint: If you already have such a test and don't want to rename it,
2020-04-10T21:15:44.1449467Z       you can also add a // gate-test-lazy_normalization_consts line to the test file.
2020-04-10T21:15:47.7399374Z Found 490 error codes
2020-04-10T21:15:47.7399664Z Found 0 error codes with no tests
2020-04-10T21:15:47.7399857Z Done!
2020-04-10T21:15:47.7400052Z some tidy checks failed
2020-04-10T21:15:47.7400052Z some tidy checks failed
2020-04-10T21:15:47.7400200Z 
2020-04-10T21:15:47.7400301Z 
2020-04-10T21:15:47.7401889Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-10T21:15:47.7402672Z 
2020-04-10T21:15:47.7402773Z 
2020-04-10T21:15:47.7403044Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-10T21:15:47.7403438Z Build completed unsuccessfully in 0:00:37
2020-04-10T21:15:47.7403438Z Build completed unsuccessfully in 0:00:37
2020-04-10T21:15:47.7513375Z == clock drift check ==
2020-04-10T21:15:47.7526736Z   local time: Fri Apr 10 21:15:47 UTC 2020
2020-04-10T21:15:48.0801421Z   network time: Fri, 10 Apr 2020 21:15:47 GMT
2020-04-10T21:15:50.4021743Z 
2020-04-10T21:15:50.4021743Z 
2020-04-10T21:15:50.4108676Z ##[error]Bash exited with code '1'.
2020-04-10T21:15:50.4122705Z ##[section]Finishing: Run build
2020-04-10T21:15:50.4173089Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67890/merge to s
2020-04-10T21:15:50.4178246Z Task         : Get sources
2020-04-10T21:15:50.4178613Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T21:15:50.4178958Z Version      : 1.0.0
2020-04-10T21:15:50.4179185Z Author       : Microsoft
2020-04-10T21:15:50.4179185Z Author       : Microsoft
2020-04-10T21:15:50.4179557Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T21:15:50.4180021Z ==============================================================================
2020-04-10T21:15:50.7793234Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T21:15:50.7840114Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67890/merge to s
2020-04-10T21:15:50.7938073Z Cleaning up task key
2020-04-10T21:15:50.7939556Z Start cleaning up orphan processes.
2020-04-10T21:15:50.8140628Z Terminate orphan process: pid (3884) (python)
2020-04-10T21:15:50.8327830Z ##[section]Finishing: Finalize Job
