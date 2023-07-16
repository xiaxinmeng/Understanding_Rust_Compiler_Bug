plain
2020-04-04T19:10:33.4207697Z ========================== Starting Command Output ===========================
2020-04-04T19:10:33.4212559Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/84faf737-26a0-4f39-84b3-885285edd184.sh
2020-04-04T19:10:33.4213142Z 
2020-04-04T19:10:33.4217589Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T19:10:33.4248642Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70665/merge to s
2020-04-04T19:10:33.4253977Z Task         : Get sources
2020-04-04T19:10:33.4254572Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T19:10:33.4254888Z Version      : 1.0.0
2020-04-04T19:10:33.4255091Z Author       : Microsoft
---
2020-04-04T19:10:35.8072002Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T19:10:35.8078240Z ##[command]git config gc.auto 0
2020-04-04T19:10:35.8081826Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T19:10:35.8084571Z ##[command]git config --get-all http.proxy
2020-04-04T19:10:35.8092100Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70665/merge:refs/remotes/pull/70665/merge
---
2020-04-04T19:14:29.8928325Z  ---> 3fc1b512c57b
2020-04-04T19:14:29.8928833Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T19:14:29.8930910Z  ---> Using cache
2020-04-04T19:14:29.8931765Z  ---> 5ee4295733f4
2020-04-04T19:14:29.8933570Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T19:14:29.8937576Z  ---> 3d07a0fa42fe
2020-04-04T19:14:29.8971644Z Successfully built 3d07a0fa42fe
2020-04-04T19:14:29.9007726Z Successfully tagged rust-ci:latest
2020-04-04T19:14:29.9278762Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T19:14:29.9278762Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T19:14:29.9295466Z Looks like docker image is the same as before, not uploading
2020-04-04T19:14:36.0169240Z [CI_JOB_NAME=mingw-check]
2020-04-04T19:14:36.0486406Z [CI_JOB_NAME=mingw-check]
2020-04-04T19:14:36.0519958Z == clock drift check ==
2020-04-04T19:14:36.0528025Z   local time: Sat Apr  4 19:14:36 UTC 2020
2020-04-04T19:14:36.2152517Z   network time: Sat, 04 Apr 2020 19:14:36 GMT
2020-04-04T19:14:36.2177449Z Starting sccache server...
2020-04-04T19:14:36.3068904Z configure: processing command line
2020-04-04T19:14:36.3077404Z configure: 
2020-04-04T19:14:36.3096198Z configure: rust.parallel-compiler := True
---
2020-04-04T19:18:28.9366379Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-04T19:18:33.8402916Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-04T19:18:35.1206206Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T19:18:35.1874506Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T19:18:35.3909752Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T19:18:36.2402919Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T19:18:36.2898611Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-04T19:18:37.9394619Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T19:18:38.4480911Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-04T19:20:39.2002449Z configure: build.locked-deps    := True
2020-04-04T19:20:39.2002959Z configure: llvm.ccache          := sccache
2020-04-04T19:20:39.2003653Z configure: build.cargo-native-static := True
2020-04-04T19:20:39.2004386Z configure: dist.missing-tools   := True
2020-04-04T19:20:39.2005226Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T19:20:39.2006333Z configure: writing `config.toml` in current directory
2020-04-04T19:20:39.2006885Z configure: 
2020-04-04T19:20:39.2007643Z configure: run `python /checkout/x.py --help`
2020-04-04T19:20:39.2007910Z configure: 
---
2020-04-04T19:22:10.2114725Z Hugepagesize:       2048 kB
2020-04-04T19:22:10.2114973Z DirectMap4k:      155584 kB
2020-04-04T19:22:10.2115237Z DirectMap2M:     4038656 kB
2020-04-04T19:22:10.2115485Z DirectMap1G:     5242880 kB
2020-04-04T19:22:10.2179096Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T19:22:11.6162672Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T19:22:11.6162672Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T19:22:11.6170341Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T19:22:11.8838075Z    Compiling unicode-xid v0.2.0
2020-04-04T19:22:12.0258191Z    Compiling syn v1.0.11
2020-04-04T19:22:12.9815839Z    Compiling linked-hash-map v0.5.2
2020-04-04T19:22:12.9979887Z    Compiling lazy_static v1.4.0
2020-04-04T19:22:12.9979887Z    Compiling lazy_static v1.4.0
2020-04-04T19:22:13.2269467Z    Compiling yaml-rust v0.4.3
2020-04-04T19:22:17.9746998Z    Compiling quote v1.0.2
2020-04-04T19:22:34.3620482Z    Compiling thiserror-impl v1.0.5
2020-04-04T19:22:39.5961837Z    Compiling thiserror v1.0.5
2020-04-04T19:22:39.6566342Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T19:22:40.9384956Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T19:22:42.7066630Z Build completed successfully in 0:00:32
2020-04-04T19:22:42.7074284Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T19:22:42.9680247Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-04T19:22:44.1507113Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T19:24:56.3213436Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-04T19:25:01.2909361Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-04T19:25:02.6375423Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T19:25:02.6563925Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T19:25:02.8689600Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T19:25:03.6409581Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T19:25:03.7610171Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-04T19:25:05.4034850Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T19:25:05.9243918Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-04T19:29:43.4454988Z Build completed successfully in 0:00:46
2020-04-04T19:29:43.4462250Z + /scripts/validate-toolstate.sh
2020-04-04T19:29:43.4515780Z Cloning into 'rust-toolstate'...
2020-04-04T19:29:44.1441338Z Traceback (most recent call last):
2020-04-04T19:29:44.1442032Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T19:29:44.1442532Z     cur_datetime
2020-04-04T19:29:44.1442974Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T19:29:44.1444301Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T19:29:44.1445443Z KeyError: u'rustc-dev-guide'
2020-04-04T19:29:44.1495501Z   local time: Sat Apr  4 19:29:44 UTC 2020
2020-04-04T19:29:44.1495501Z   local time: Sat Apr  4 19:29:44 UTC 2020
2020-04-04T19:29:44.3115355Z   network time: Sat, 04 Apr 2020 19:29:44 GMT
2020-04-04T19:29:45.7114234Z 
2020-04-04T19:29:45.7114234Z 
2020-04-04T19:29:45.7195840Z ##[error]Bash exited with code '1'.
2020-04-04T19:29:45.7211700Z ##[section]Finishing: Run build
2020-04-04T19:29:45.7270625Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70665/merge to s
2020-04-04T19:29:45.7276214Z Task         : Get sources
2020-04-04T19:29:45.7276601Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T19:29:45.7276960Z Version      : 1.0.0
2020-04-04T19:29:45.7277226Z Author       : Microsoft
2020-04-04T19:29:45.7277226Z Author       : Microsoft
2020-04-04T19:29:45.7277622Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T19:29:45.7278097Z ==============================================================================
2020-04-04T19:29:46.0627973Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T19:29:46.0677135Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70665/merge to s
2020-04-04T19:29:46.0763574Z Cleaning up task key
2020-04-04T19:29:46.0764881Z Start cleaning up orphan processes.
2020-04-04T19:29:46.2759213Z Terminate orphan process: pid (4262) (python)
2020-04-04T19:29:46.2815054Z ##[section]Finishing: Finalize Job
