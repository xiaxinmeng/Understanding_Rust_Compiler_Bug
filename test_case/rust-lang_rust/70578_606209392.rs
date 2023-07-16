plain
2020-03-30T18:01:45.7663890Z ========================== Starting Command Output ===========================
2020-03-30T18:01:45.7680606Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c27616f9-a698-4371-a950-95bfc18594de.sh
2020-03-30T18:01:45.7898261Z 
2020-03-30T18:01:45.7950140Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-30T18:01:45.7968068Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70578/merge to s
2020-03-30T18:01:45.7971384Z Task         : Get sources
2020-03-30T18:01:45.7971672Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T18:01:45.7971972Z Version      : 1.0.0
2020-03-30T18:01:45.7972166Z Author       : Microsoft
---
2020-03-30T18:01:47.0796910Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-30T18:01:47.0805799Z ##[command]git config gc.auto 0
2020-03-30T18:01:47.0826103Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-30T18:01:47.0832446Z ##[command]git config --get-all http.proxy
2020-03-30T18:01:47.0844280Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70578/merge:refs/remotes/pull/70578/merge
---
2020-03-30T18:04:14.8565281Z  ---> 3fc1b512c57b
2020-03-30T18:04:14.8565545Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-30T18:04:14.8565924Z  ---> Using cache
2020-03-30T18:04:14.8566252Z  ---> 5ee4295733f4
2020-03-30T18:04:14.8567642Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-30T18:04:14.8568983Z  ---> 3d07a0fa42fe
2020-03-30T18:04:14.8681476Z Successfully built 3d07a0fa42fe
2020-03-30T18:04:14.8716701Z Successfully tagged rust-ci:latest
2020-03-30T18:04:14.8996648Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-30T18:07:27.7322909Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T18:07:27.7931007Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T18:07:27.9593873Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T18:07:28.0818282Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T18:07:28.4811539Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T18:07:30.4883061Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T18:07:30.9117640Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T18:07:32.7097015Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T18:07:33.1027172Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T18:09:07.5130062Z configure: build.locked-deps    := True
2020-03-30T18:09:07.5130375Z configure: llvm.ccache          := sccache
2020-03-30T18:09:07.5130854Z configure: build.cargo-native-static := True
2020-03-30T18:09:07.5131428Z configure: dist.missing-tools   := True
2020-03-30T18:09:07.5131964Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-30T18:09:07.5132497Z configure: writing `config.toml` in current directory
2020-03-30T18:09:07.5132716Z configure: 
2020-03-30T18:09:07.5133109Z configure: run `python /checkout/x.py --help`
2020-03-30T18:09:07.5133320Z configure: 
---
2020-03-30T18:10:21.3024628Z Hugepagesize:       2048 kB
2020-03-30T18:10:21.3024873Z DirectMap4k:      143296 kB
2020-03-30T18:10:21.3025094Z DirectMap2M:     4050944 kB
2020-03-30T18:10:21.3025300Z DirectMap1G:     5242880 kB
2020-03-30T18:10:21.3066659Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-30T18:10:22.5656246Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-30T18:10:22.5656246Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-30T18:10:22.5659566Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-30T18:10:22.7758286Z    Compiling unicode-xid v0.2.0
2020-03-30T18:10:22.8868446Z    Compiling syn v1.0.11
2020-03-30T18:10:23.5991853Z    Compiling linked-hash-map v0.5.2
2020-03-30T18:10:23.6370409Z    Compiling lazy_static v1.4.0
2020-03-30T18:10:23.6370409Z    Compiling lazy_static v1.4.0
2020-03-30T18:10:23.8039538Z    Compiling yaml-rust v0.4.3
2020-03-30T18:10:27.5908547Z    Compiling quote v1.0.2
2020-03-30T18:10:40.1351487Z    Compiling thiserror-impl v1.0.5
2020-03-30T18:10:44.1587616Z    Compiling thiserror v1.0.5
2020-03-30T18:10:44.2167155Z    Compiling yaml-merge-keys v0.4.0
2020-03-30T18:10:45.2279714Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-30T18:10:46.6458378Z Build completed successfully in 0:00:25
2020-03-30T18:10:46.6466512Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-30T18:10:46.8456116Z     Finished dev [unoptimized] target(s) in 0.14s
2020-03-30T18:10:47.8116231Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-30T18:12:37.7856905Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T18:12:37.8772111Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T18:12:38.0542525Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T18:12:38.1377998Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T18:12:38.5630070Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T18:12:40.4018617Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T18:12:40.8105733Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T18:12:42.6267950Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T18:12:42.9948456Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T18:16:14.8910098Z Found 0 error codes with no tests
2020-03-30T18:16:14.8910496Z Done!
2020-03-30T18:16:14.8910693Z 
2020-03-30T18:16:14.8910869Z 
2020-03-30T18:16:14.8912846Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-30T18:16:14.8914170Z 
2020-03-30T18:16:14.8914350Z 
2020-03-30T18:16:14.8914920Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-30T18:16:14.8915548Z Build completed unsuccessfully in 0:00:31
2020-03-30T18:16:14.8915548Z Build completed unsuccessfully in 0:00:31
2020-03-30T18:16:14.8966829Z == clock drift check ==
2020-03-30T18:16:14.9007575Z   local time: Mon Mar 30 18:16:14 UTC 2020
2020-03-30T18:16:15.1619316Z   network time: Mon, 30 Mar 2020 18:16:15 GMT
2020-03-30T18:16:15.1627188Z == end clock drift check ==
2020-03-30T18:16:16.7530121Z 
2020-03-30T18:16:16.7599295Z ##[error]Bash exited with code '1'.
2020-03-30T18:16:16.7612487Z ##[section]Finishing: Run build
2020-03-30T18:16:16.7651746Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70578/merge to s
2020-03-30T18:16:16.7655936Z Task         : Get sources
2020-03-30T18:16:16.7656233Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T18:16:16.7656492Z Version      : 1.0.0
2020-03-30T18:16:16.7656677Z Author       : Microsoft
2020-03-30T18:16:16.7656677Z Author       : Microsoft
2020-03-30T18:16:16.7656980Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-30T18:16:16.7657306Z ==============================================================================
2020-03-30T18:16:17.0713344Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-30T18:16:17.0758106Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70578/merge to s
2020-03-30T18:16:17.0835040Z Cleaning up task key
2020-03-30T18:16:17.0836319Z Start cleaning up orphan processes.
2020-03-30T18:16:17.1012818Z Terminate orphan process: pid (5728) (python)
2020-03-30T18:16:17.1184252Z ##[section]Finishing: Finalize Job
