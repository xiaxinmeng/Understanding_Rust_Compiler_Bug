plain
2020-04-06T15:40:26.7579477Z ========================== Starting Command Output ===========================
2020-04-06T15:40:26.7586321Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ece6e5d7-4176-4f96-9036-3e2098c1fb78.sh
2020-04-06T15:40:26.7586972Z 
2020-04-06T15:40:26.7592770Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T15:40:26.7615367Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70690/merge to s
2020-04-06T15:40:26.7619235Z Task         : Get sources
2020-04-06T15:40:26.7619592Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T15:40:26.7619905Z Version      : 1.0.0
2020-04-06T15:40:26.7620118Z Author       : Microsoft
---
2020-04-06T15:40:27.7437439Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T15:40:27.7443756Z ##[command]git config gc.auto 0
2020-04-06T15:40:27.7447929Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T15:40:27.7451674Z ##[command]git config --get-all http.proxy
2020-04-06T15:40:27.7458537Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70690/merge:refs/remotes/pull/70690/merge
---
2020-04-06T15:44:04.8820921Z  ---> 3fc1b512c57b
2020-04-06T15:44:04.8822525Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-06T15:44:04.8826389Z  ---> Using cache
2020-04-06T15:44:04.8826797Z  ---> 5ee4295733f4
2020-04-06T15:44:04.8832072Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-06T15:44:04.8833982Z  ---> 3d07a0fa42fe
2020-04-06T15:44:04.8970915Z Successfully built 3d07a0fa42fe
2020-04-06T15:44:04.8971779Z Successfully tagged rust-ci:latest
2020-04-06T15:44:04.9427116Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T15:44:04.9427116Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T15:44:04.9443816Z Looks like docker image is the same as before, not uploading
2020-04-06T15:44:11.7789549Z [CI_JOB_NAME=mingw-check]
2020-04-06T15:44:11.8053114Z [CI_JOB_NAME=mingw-check]
2020-04-06T15:44:11.8093462Z == clock drift check ==
2020-04-06T15:44:11.8097967Z   local time: Mon Apr  6 15:44:11 UTC 2020
2020-04-06T15:44:12.0976483Z   network time: Mon, 06 Apr 2020 15:44:12 GMT
2020-04-06T15:44:12.1009127Z Starting sccache server...
2020-04-06T15:44:12.2049900Z configure: processing command line
2020-04-06T15:44:12.2050787Z configure: 
2020-04-06T15:44:12.2052098Z configure: rust.parallel-compiler := True
---
2020-04-06T15:47:48.1730259Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-06T15:47:52.7443406Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-06T15:47:53.9963919Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T15:47:54.0851383Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T15:47:54.2804054Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T15:47:55.0344681Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T15:47:55.1597061Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-06T15:47:56.6106513Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T15:47:57.1150026Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-06T15:49:47.8439216Z configure: build.locked-deps    := True
2020-04-06T15:49:47.8439574Z configure: llvm.ccache          := sccache
2020-04-06T15:49:47.8440110Z configure: build.cargo-native-static := True
2020-04-06T15:49:47.8440624Z configure: dist.missing-tools   := True
2020-04-06T15:49:47.8441706Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-06T15:49:47.8442336Z configure: writing `config.toml` in current directory
2020-04-06T15:49:47.8442602Z configure: 
2020-04-06T15:49:47.8443127Z configure: run `python /checkout/x.py --help`
2020-04-06T15:49:47.8443381Z configure: 
---
2020-04-06T15:51:13.2486439Z Hugepagesize:       2048 kB
2020-04-06T15:51:13.2486687Z DirectMap4k:      126912 kB
2020-04-06T15:51:13.2486954Z DirectMap2M:     5115904 kB
2020-04-06T15:51:13.2487202Z DirectMap1G:     4194304 kB
2020-04-06T15:51:13.2497770Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-06T15:51:14.6645911Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-06T15:51:14.6645911Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-06T15:51:14.6654812Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-06T15:51:14.9149251Z    Compiling unicode-xid v0.2.0
2020-04-06T15:51:15.0666167Z    Compiling syn v1.0.11
2020-04-06T15:51:15.9359553Z    Compiling linked-hash-map v0.5.2
2020-04-06T15:51:15.9876343Z    Compiling lazy_static v1.4.0
2020-04-06T15:51:15.9876343Z    Compiling lazy_static v1.4.0
2020-04-06T15:51:16.1553177Z    Compiling yaml-rust v0.4.3
2020-04-06T15:51:20.6305570Z    Compiling quote v1.0.2
2020-04-06T15:51:35.3576782Z    Compiling thiserror-impl v1.0.5
2020-04-06T15:51:40.1902205Z    Compiling thiserror v1.0.5
2020-04-06T15:51:41.2098853Z    Compiling yaml-merge-keys v0.4.0
2020-04-06T15:51:42.5141507Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-06T15:51:44.0891221Z Build completed successfully in 0:00:30
2020-04-06T15:51:44.0899152Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-06T15:51:44.3197929Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-06T15:51:45.4718769Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-06T15:53:47.1133104Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-06T15:53:51.7474375Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-06T15:53:52.9800914Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T15:53:53.1883475Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T15:53:53.3843509Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T15:53:53.9761512Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T15:53:54.1366962Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-06T15:53:55.5187529Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T15:53:56.0327318Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-06T15:57:56.6603053Z Done!
2020-04-06T15:57:56.6603222Z some tidy checks failed
2020-04-06T15:57:56.6603344Z 
2020-04-06T15:57:56.6603431Z 
2020-04-06T15:57:56.6604595Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-06T15:57:56.6605279Z 
2020-04-06T15:57:56.6605365Z 
2020-04-06T15:57:56.6608583Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-06T15:57:56.6608908Z Build completed unsuccessfully in 0:00:33
2020-04-06T15:57:56.6608908Z Build completed unsuccessfully in 0:00:33
2020-04-06T15:57:56.6664739Z == clock drift check ==
2020-04-06T15:57:56.6684442Z   local time: Mon Apr  6 15:57:56 UTC 2020
2020-04-06T15:57:56.7629861Z   network time: Mon, 06 Apr 2020 15:57:56 GMT
2020-04-06T15:57:58.3603117Z 
2020-04-06T15:57:58.3603117Z 
2020-04-06T15:57:58.3677562Z ##[error]Bash exited with code '1'.
2020-04-06T15:57:58.3701257Z ##[section]Finishing: Run build
2020-04-06T15:57:58.3760633Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70690/merge to s
2020-04-06T15:57:58.3767274Z Task         : Get sources
2020-04-06T15:57:58.3767663Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T15:57:58.3768061Z Version      : 1.0.0
2020-04-06T15:57:58.3768315Z Author       : Microsoft
2020-04-06T15:57:58.3768315Z Author       : Microsoft
2020-04-06T15:57:58.3768710Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T15:57:58.3769192Z ==============================================================================
2020-04-06T15:57:58.7373229Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T15:57:58.7420352Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70690/merge to s
2020-04-06T15:57:58.7523613Z Cleaning up task key
2020-04-06T15:57:58.7525315Z Start cleaning up orphan processes.
2020-04-06T15:57:58.7743202Z Terminate orphan process: pid (3856) (python)
2020-04-06T15:57:58.7972448Z ##[section]Finishing: Finalize Job
