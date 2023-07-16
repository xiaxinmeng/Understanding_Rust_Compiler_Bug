plain
2020-04-02T21:10:27.1093389Z ========================== Starting Command Output ===========================
2020-04-02T21:10:27.1095796Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/132ea8ee-4364-44ce-9691-2e8cc2571951.sh
2020-04-02T21:10:27.1096068Z 
2020-04-02T21:10:27.1099296Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T21:10:27.1119089Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-02T21:10:27.1122051Z Task         : Get sources
2020-04-02T21:10:27.1122301Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T21:10:27.1122591Z Version      : 1.0.0
2020-04-02T21:10:27.1122758Z Author       : Microsoft
---
2020-04-02T21:10:28.4026619Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T21:10:28.4032973Z ##[command]git config gc.auto 0
2020-04-02T21:10:28.4036559Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T21:10:28.4039908Z ##[command]git config --get-all http.proxy
2020-04-02T21:10:28.4046896Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70721/merge:refs/remotes/pull/70721/merge
---
2020-04-02T21:13:58.0836264Z  ---> 3fc1b512c57b
2020-04-02T21:13:58.0836489Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-02T21:13:58.0840818Z  ---> Using cache
2020-04-02T21:13:58.0841162Z  ---> 5ee4295733f4
2020-04-02T21:13:58.0842695Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-02T21:13:58.0848221Z  ---> 3d07a0fa42fe
2020-04-02T21:13:58.0885248Z Successfully built 3d07a0fa42fe
2020-04-02T21:13:58.0914173Z Successfully tagged rust-ci:latest
2020-04-02T21:13:58.1199728Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-02T21:13:58.1199728Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-02T21:13:58.1208105Z Looks like docker image is the same as before, not uploading
2020-04-02T21:14:04.8826504Z [CI_JOB_NAME=mingw-check]
2020-04-02T21:14:04.9036023Z [CI_JOB_NAME=mingw-check]
2020-04-02T21:14:04.9074351Z == clock drift check ==
2020-04-02T21:14:04.9090567Z   local time: Thu Apr  2 21:14:04 UTC 2020
2020-04-02T21:14:05.1707668Z   network time: Thu, 02 Apr 2020 21:14:05 GMT
2020-04-02T21:14:05.1731206Z Starting sccache server...
2020-04-02T21:14:05.2573856Z configure: processing command line
2020-04-02T21:14:05.2574130Z configure: 
2020-04-02T21:14:05.2574905Z configure: rust.parallel-compiler := True
---
2020-04-02T21:17:18.3431446Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-02T21:17:22.4109561Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-02T21:17:23.5027458Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T21:17:23.6431537Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T21:17:23.8257824Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T21:17:24.4699766Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T21:17:24.6321672Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-02T21:17:26.0460407Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T21:17:26.5634828Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-02T21:19:10.3054110Z configure: build.locked-deps    := True
2020-04-02T21:19:10.3054520Z configure: llvm.ccache          := sccache
2020-04-02T21:19:10.3055100Z configure: build.cargo-native-static := True
2020-04-02T21:19:10.3055685Z configure: dist.missing-tools   := True
2020-04-02T21:19:10.3056391Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-02T21:19:10.3057172Z configure: writing `config.toml` in current directory
2020-04-02T21:19:10.3057503Z configure: 
2020-04-02T21:19:10.3057998Z configure: run `python /checkout/x.py --help`
2020-04-02T21:19:10.3058354Z configure: 
---
2020-04-02T21:20:26.4611771Z Hugepagesize:       2048 kB
2020-04-02T21:20:26.4612037Z DirectMap4k:      112576 kB
2020-04-02T21:20:26.4612663Z DirectMap2M:     4081664 kB
2020-04-02T21:20:26.4612992Z DirectMap1G:     5242880 kB
2020-04-02T21:20:26.4613780Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-02T21:20:27.6428526Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-02T21:20:27.6428526Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-02T21:20:27.6462237Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-02T21:20:27.8495955Z    Compiling unicode-xid v0.2.0
2020-04-02T21:20:27.9763450Z    Compiling syn v1.0.11
2020-04-02T21:20:28.7757633Z    Compiling linked-hash-map v0.5.2
2020-04-02T21:20:28.8460049Z    Compiling lazy_static v1.4.0
2020-04-02T21:20:28.8460049Z    Compiling lazy_static v1.4.0
2020-04-02T21:20:29.0112700Z    Compiling yaml-rust v0.4.3
2020-04-02T21:20:33.2378841Z    Compiling quote v1.0.2
2020-04-02T21:20:47.3552701Z    Compiling thiserror-impl v1.0.5
2020-04-02T21:20:52.0704589Z    Compiling thiserror v1.0.5
2020-04-02T21:20:52.1272932Z    Compiling yaml-merge-keys v0.4.0
2020-04-02T21:20:53.2642446Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-02T21:20:55.7481507Z Build completed successfully in 0:00:29
2020-04-02T21:20:55.7490933Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-02T21:20:55.9543795Z     Finished dev [unoptimized] target(s) in 0.15s
2020-04-02T21:20:56.9339671Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-02T21:22:56.1040837Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T21:22:56.3834362Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T21:22:56.5140289Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T21:22:56.5882565Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T21:22:57.1002882Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T21:22:59.2308380Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T21:22:59.7143012Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T21:23:01.7268092Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T21:23:02.1659355Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T21:26:30.6807509Z    Compiling cargo_metadata v0.9.1
2020-04-02T21:26:34.2363978Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-02T21:26:44.5464452Z     Finished release [optimized] target(s) in 23.38s
2020-04-02T21:26:44.5556561Z tidy check
2020-04-02T21:26:48.8748895Z tidy error: /checkout/src/test/mir-opt/graphviz.rs: ignoring line length unnecessarily
2020-04-02T21:26:48.8754449Z tidy error: /checkout/src/test/mir-opt/unreachable_asm_2.rs: ignoring line length unnecessarily
2020-04-02T21:26:48.8798973Z tidy error: /checkout/src/test/mir-opt/issue-62289.rs: ignoring line length unnecessarily
2020-04-02T21:26:48.8806265Z tidy error: /checkout/src/test/mir-opt/unusual-item-types.rs: ignoring line length unnecessarily
2020-04-02T21:26:48.8836757Z tidy error: /checkout/src/test/mir-opt/storage_live_dead_in_statics.rs: ignoring line length unnecessarily
2020-04-02T21:26:48.8980110Z tidy error: /checkout/src/test/mir-opt/exponential-or.rs: ignoring line length unnecessarily
2020-04-02T21:26:48.9088056Z tidy error: /checkout/src/test/mir-opt/unreachable_asm.rs: ignoring line length unnecessarily
2020-04-02T21:26:48.9116821Z tidy error: /checkout/src/test/mir-opt/byte_slice.rs: too many trailing newlines (2)
2020-04-02T21:26:51.7420383Z Found 491 error codes
2020-04-02T21:26:51.7420825Z Found 0 error codes with no tests
2020-04-02T21:26:51.7421138Z Done!
2020-04-02T21:26:51.7424820Z some tidy checks failed
2020-04-02T21:26:51.7424820Z some tidy checks failed
2020-04-02T21:26:51.7425501Z 
2020-04-02T21:26:51.7425869Z 
2020-04-02T21:26:51.7427136Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-02T21:26:51.7427766Z 
2020-04-02T21:26:51.7427840Z 
2020-04-02T21:26:51.7435263Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-02T21:26:51.7435572Z Build completed unsuccessfully in 0:00:31
2020-04-02T21:26:51.7435572Z Build completed unsuccessfully in 0:00:31
2020-04-02T21:26:51.7485093Z == clock drift check ==
2020-04-02T21:26:51.7510065Z   local time: Thu Apr  2 21:26:51 UTC 2020
2020-04-02T21:26:52.2695734Z   network time: Thu, 02 Apr 2020 21:26:52 GMT
2020-04-02T21:26:54.1740723Z 
2020-04-02T21:26:54.1740723Z 
2020-04-02T21:26:54.1813512Z ##[error]Bash exited with code '1'.
2020-04-02T21:26:54.1825179Z ##[section]Finishing: Run build
2020-04-02T21:26:54.1867349Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-02T21:26:54.1872052Z Task         : Get sources
2020-04-02T21:26:54.1872315Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T21:26:54.1872575Z Version      : 1.0.0
2020-04-02T21:26:54.1872740Z Author       : Microsoft
2020-04-02T21:26:54.1872740Z Author       : Microsoft
2020-04-02T21:26:54.1873000Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T21:26:54.1873326Z ==============================================================================
2020-04-02T21:26:54.4833791Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T21:26:54.4871708Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-02T21:26:54.4959358Z Cleaning up task key
2020-04-02T21:26:54.4960436Z Start cleaning up orphan processes.
2020-04-02T21:26:54.6205647Z Terminate orphan process: pid (7452) (python)
2020-04-02T21:26:54.6238655Z ##[section]Finishing: Finalize Job
