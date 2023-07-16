plain
2020-03-31T06:01:56.4016959Z ========================== Starting Command Output ===========================
2020-03-31T06:01:56.4020640Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/47679166-6b31-4d82-8069-c6a47108b944.sh
2020-03-31T06:01:56.4020841Z 
2020-03-31T06:01:56.4023759Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T06:01:56.4038541Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-31T06:01:56.4041234Z Task         : Get sources
2020-03-31T06:01:56.4041494Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T06:01:56.4041705Z Version      : 1.0.0
2020-03-31T06:01:56.4041850Z Author       : Microsoft
---
2020-03-31T06:01:57.7242995Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T06:01:57.7298459Z ##[command]git config gc.auto 0
2020-03-31T06:01:57.7301232Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T06:01:57.7304013Z ##[command]git config --get-all http.proxy
2020-03-31T06:01:57.7309375Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
---
2020-03-31T06:05:06.0780497Z  ---> 3fc1b512c57b
2020-03-31T06:05:06.0780679Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-31T06:05:06.0784695Z  ---> Using cache
2020-03-31T06:05:06.0784975Z  ---> 5ee4295733f4
2020-03-31T06:05:06.0790046Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-31T06:05:06.0795734Z  ---> 3d07a0fa42fe
2020-03-31T06:05:06.0796380Z Successfully built 3d07a0fa42fe
2020-03-31T06:05:06.0831899Z Successfully tagged rust-ci:latest
2020-03-31T06:05:06.1115329Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-31T06:08:12.4072078Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-03-31T06:08:16.1085047Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-03-31T06:08:17.1543424Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T06:08:17.3415108Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T06:08:17.5001726Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T06:08:18.0836154Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T06:08:18.2108643Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-31T06:08:19.4404777Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T06:08:19.8714407Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-31T06:09:58.3683497Z configure: build.locked-deps    := True
2020-03-31T06:09:58.3683843Z configure: llvm.ccache          := sccache
2020-03-31T06:09:58.3684383Z configure: build.cargo-native-static := True
2020-03-31T06:09:58.3684896Z configure: dist.missing-tools   := True
2020-03-31T06:09:58.3770170Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-31T06:09:58.3771001Z configure: writing `config.toml` in current directory
2020-03-31T06:09:58.3771321Z configure: 
2020-03-31T06:09:58.3771818Z configure: run `python /checkout/x.py --help`
2020-03-31T06:09:58.3772151Z configure: 
---
2020-03-31T06:11:17.2180339Z Hugepagesize:       2048 kB
2020-03-31T06:11:17.2180503Z DirectMap4k:      143296 kB
2020-03-31T06:11:17.2180666Z DirectMap2M:     3002368 kB
2020-03-31T06:11:17.2180848Z DirectMap1G:     6291456 kB
2020-03-31T06:11:17.2205842Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-31T06:11:18.2989707Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-31T06:11:18.2989707Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-31T06:11:18.2994185Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-31T06:11:18.4798527Z    Compiling unicode-xid v0.2.0
2020-03-31T06:11:18.5879937Z    Compiling syn v1.0.11
2020-03-31T06:11:19.3070614Z    Compiling linked-hash-map v0.5.2
2020-03-31T06:11:19.3265421Z    Compiling lazy_static v1.4.0
2020-03-31T06:11:19.3265421Z    Compiling lazy_static v1.4.0
2020-03-31T06:11:19.5279754Z    Compiling yaml-rust v0.4.3
2020-03-31T06:11:23.2860092Z    Compiling quote v1.0.2
2020-03-31T06:11:36.4268845Z    Compiling thiserror-impl v1.0.5
2020-03-31T06:11:39.9173920Z    Compiling thiserror v1.0.5
2020-03-31T06:11:39.9704041Z    Compiling yaml-merge-keys v0.4.0
2020-03-31T06:11:40.9360478Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-31T06:11:42.3759905Z Build completed successfully in 0:00:25
2020-03-31T06:11:42.3766696Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-31T06:11:42.5750851Z     Finished dev [unoptimized] target(s) in 0.14s
2020-03-31T06:11:43.4867579Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-31T06:13:29.0225151Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-03-31T06:13:32.9566438Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-03-31T06:13:34.0252794Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T06:13:34.0354431Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T06:13:34.2180577Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T06:13:34.9503228Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T06:13:34.9581808Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-31T06:13:36.2481570Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T06:13:36.6925502Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-31T06:17:04.7758330Z Done!
2020-03-31T06:17:04.7761233Z some tidy checks failed
2020-03-31T06:17:04.7761353Z 
2020-03-31T06:17:04.7761451Z 
2020-03-31T06:17:04.7762484Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-31T06:17:04.7763200Z 
2020-03-31T06:17:04.7763295Z 
2020-03-31T06:17:04.7790049Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-31T06:17:04.7790382Z Build completed unsuccessfully in 0:00:29
2020-03-31T06:17:04.7790382Z Build completed unsuccessfully in 0:00:29
2020-03-31T06:17:04.7828471Z == clock drift check ==
2020-03-31T06:17:04.7838762Z   local time: Tue Mar 31 06:17:04 UTC 2020
2020-03-31T06:17:05.0534078Z   network time: Tue, 31 Mar 2020 06:17:05 GMT
2020-03-31T06:17:05.0534325Z == end clock drift check ==
2020-03-31T06:17:06.8021556Z 
2020-03-31T06:17:06.8081740Z ##[error]Bash exited with code '1'.
2020-03-31T06:17:06.8093993Z ##[section]Finishing: Run build
2020-03-31T06:17:06.8135077Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-31T06:17:06.8139047Z Task         : Get sources
2020-03-31T06:17:06.8139314Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T06:17:06.8139556Z Version      : 1.0.0
2020-03-31T06:17:06.8139742Z Author       : Microsoft
2020-03-31T06:17:06.8139742Z Author       : Microsoft
2020-03-31T06:17:06.8140012Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T06:17:06.8140327Z ==============================================================================
2020-03-31T06:17:07.0888092Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T06:17:07.0943773Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-31T06:17:07.1023449Z Cleaning up task key
2020-03-31T06:17:07.1024961Z Start cleaning up orphan processes.
2020-03-31T06:17:07.1215164Z Terminate orphan process: pid (3489) (python)
2020-03-31T06:17:07.1392741Z ##[section]Finishing: Finalize Job
