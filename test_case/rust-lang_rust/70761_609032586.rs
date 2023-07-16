plain
2020-04-04T11:58:20.1379520Z ========================== Starting Command Output ===========================
2020-04-04T11:58:20.1382460Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6974a5ae-2498-4186-884d-aef2cfd98aa9.sh
2020-04-04T11:58:20.1382793Z 
2020-04-04T11:58:20.1387460Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T11:58:20.1410416Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70761/merge to s
2020-04-04T11:58:20.1414545Z Task         : Get sources
2020-04-04T11:58:20.1414937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T11:58:20.1415305Z Version      : 1.0.0
2020-04-04T11:58:20.1415549Z Author       : Microsoft
---
2020-04-04T11:58:21.1478199Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T11:58:21.1483980Z ##[command]git config gc.auto 0
2020-04-04T11:58:21.1488537Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T11:58:21.1497029Z ##[command]git config --get-all http.proxy
2020-04-04T11:58:21.1504312Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70761/merge:refs/remotes/pull/70761/merge
---
2020-04-04T12:01:21.6402576Z  ---> 3fc1b512c57b
2020-04-04T12:01:21.6402838Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T12:01:21.6407523Z  ---> Using cache
2020-04-04T12:01:21.6407884Z  ---> 5ee4295733f4
2020-04-04T12:01:21.6409296Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T12:01:21.6445433Z  ---> 3d07a0fa42fe
2020-04-04T12:01:21.6445678Z Successfully built 3d07a0fa42fe
2020-04-04T12:01:21.6466567Z Successfully tagged rust-ci:latest
2020-04-04T12:01:22.6851438Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T12:01:22.6851438Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T12:01:22.6853019Z Looks like docker image is the same as before, not uploading
2020-04-04T12:01:27.1338866Z [CI_JOB_NAME=mingw-check]
2020-04-04T12:01:27.1532266Z [CI_JOB_NAME=mingw-check]
2020-04-04T12:01:27.1565352Z == clock drift check ==
2020-04-04T12:01:27.1578708Z   local time: Sat Apr  4 12:01:27 UTC 2020
2020-04-04T12:01:27.4409651Z   network time: Sat, 04 Apr 2020 12:01:27 GMT
2020-04-04T12:01:27.4434145Z Starting sccache server...
2020-04-04T12:01:27.5318618Z configure: processing command line
2020-04-04T12:01:27.5319353Z configure: 
2020-04-04T12:01:27.5320639Z configure: rust.parallel-compiler := True
---
2020-04-04T12:04:56.4714440Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T12:04:56.8437659Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T12:04:56.8443254Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T12:04:57.0271339Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T12:04:57.4245351Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T12:04:59.7046824Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T12:05:00.2058648Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T12:05:02.2926560Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T12:05:02.6988112Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T12:06:47.8472871Z configure: build.locked-deps    := True
2020-04-04T12:06:47.8473379Z configure: llvm.ccache          := sccache
2020-04-04T12:06:47.8474059Z configure: build.cargo-native-static := True
2020-04-04T12:06:47.8474963Z configure: dist.missing-tools   := True
2020-04-04T12:06:47.8475864Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T12:06:47.8476801Z configure: writing `config.toml` in current directory
2020-04-04T12:06:47.8477212Z configure: 
2020-04-04T12:06:47.8477821Z configure: run `python /checkout/x.py --help`
2020-04-04T12:06:47.8478266Z configure: 
---
2020-04-04T12:08:06.8059249Z Hugepagesize:       2048 kB
2020-04-04T12:08:06.8059470Z DirectMap4k:      137152 kB
2020-04-04T12:08:06.8059693Z DirectMap2M:     4057088 kB
2020-04-04T12:08:06.8059935Z DirectMap1G:     5242880 kB
2020-04-04T12:08:06.8125621Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T12:08:08.1326838Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T12:08:08.1326838Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T12:08:08.1333183Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T12:08:08.3761903Z    Compiling unicode-xid v0.2.0
2020-04-04T12:08:08.5010358Z    Compiling syn v1.0.11
2020-04-04T12:08:09.2984083Z    Compiling linked-hash-map v0.5.2
2020-04-04T12:08:09.3497611Z    Compiling lazy_static v1.4.0
2020-04-04T12:08:09.3497611Z    Compiling lazy_static v1.4.0
2020-04-04T12:08:09.5250913Z    Compiling yaml-rust v0.4.3
2020-04-04T12:08:13.6224158Z    Compiling quote v1.0.2
2020-04-04T12:08:27.3471348Z    Compiling thiserror-impl v1.0.5
2020-04-04T12:08:31.8615368Z    Compiling thiserror v1.0.5
2020-04-04T12:08:31.9197656Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T12:08:33.0597907Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T12:08:36.8676066Z Build completed successfully in 0:00:30
2020-04-04T12:08:36.8683516Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T12:08:37.1065631Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-04T12:08:38.1834157Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T12:10:33.4328682Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-04T12:10:37.8392958Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-04T12:10:39.0168147Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T12:10:39.0563489Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T12:10:39.2389419Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T12:10:39.9613528Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T12:10:40.0380294Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-04T12:10:41.5246551Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T12:10:42.0185384Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-04T12:14:14.6406481Z    Compiling cargo_metadata v0.9.1
2020-04-04T12:14:18.8305696Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-04T12:14:29.7197096Z     Finished release [optimized] target(s) in 24.19s
2020-04-04T12:14:29.7290743Z tidy check
2020-04-04T12:14:36.0161599Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:98: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0162204Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:137: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0162736Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:160: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0163270Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:194: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0163782Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:222: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0164322Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:245: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0164858Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:470: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0165369Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:587: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0165883Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:696: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0166416Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:1168: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0175015Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:1921: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0175564Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:2020: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0176124Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:2542: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0176641Z tidy error: /checkout/src/librustc_parse_ra/lib.rs:2930: TODO is deprecated; use FIXME
2020-04-04T12:14:36.0178122Z tidy error: /checkout/src/librustc_parse_ra/lib.rs: too many lines (3037) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-04-04T12:14:38.4213383Z Found 491 error codes
2020-04-04T12:14:38.4214190Z Found 0 error codes with no tests
2020-04-04T12:14:38.4214607Z Done!
2020-04-04T12:14:38.4214957Z some tidy checks failed
2020-04-04T12:14:38.4214957Z some tidy checks failed
2020-04-04T12:14:38.4218243Z 
2020-04-04T12:14:38.4218592Z 
2020-04-04T12:14:38.4220149Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-04T12:14:38.4221313Z 
2020-04-04T12:14:38.4221560Z 
2020-04-04T12:14:38.4229007Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-04T12:14:38.4229442Z Build completed unsuccessfully in 0:00:34
2020-04-04T12:14:38.4229442Z Build completed unsuccessfully in 0:00:34
2020-04-04T12:14:38.4283846Z == clock drift check ==
2020-04-04T12:14:38.4306848Z   local time: Sat Apr  4 12:14:38 UTC 2020
2020-04-04T12:14:38.5871579Z   network time: Sat, 04 Apr 2020 12:14:38 GMT
2020-04-04T12:14:40.1345342Z 
2020-04-04T12:14:40.1345342Z 
2020-04-04T12:14:40.1397256Z ##[error]Bash exited with code '1'.
2020-04-04T12:14:40.1407815Z ##[section]Finishing: Run build
2020-04-04T12:14:40.1451672Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70761/merge to s
2020-04-04T12:14:40.1456190Z Task         : Get sources
2020-04-04T12:14:40.1456467Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T12:14:40.1456722Z Version      : 1.0.0
2020-04-04T12:14:40.1456922Z Author       : Microsoft
2020-04-04T12:14:40.1456922Z Author       : Microsoft
2020-04-04T12:14:40.1457206Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T12:14:40.1457531Z ==============================================================================
2020-04-04T12:14:40.4526760Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T12:14:40.4574869Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70761/merge to s
2020-04-04T12:14:40.4658451Z Cleaning up task key
2020-04-04T12:14:40.4659550Z Start cleaning up orphan processes.
2020-04-04T12:14:40.4829668Z Terminate orphan process: pid (4199) (python)
2020-04-04T12:14:40.4993790Z ##[section]Finishing: Finalize Job
