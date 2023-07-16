plain
2020-03-25T17:12:03.3122776Z ========================== Starting Command Output ===========================
2020-03-25T17:12:03.3129083Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/13938642-d188-4fc7-897a-d8bd9fab6337.sh
2020-03-25T17:12:03.3129701Z 
2020-03-25T17:12:03.3134528Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-25T17:12:03.3156062Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70397/merge to s
2020-03-25T17:12:03.3159671Z Task         : Get sources
2020-03-25T17:12:03.3159942Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T17:12:03.3160200Z Version      : 1.0.0
2020-03-25T17:12:03.3160366Z Author       : Microsoft
---
2020-03-25T17:12:05.5753131Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-25T17:12:05.5925065Z ##[command]git config gc.auto 0
2020-03-25T17:12:05.5981885Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-25T17:12:05.6014215Z ##[command]git config --get-all http.proxy
2020-03-25T17:12:05.6101198Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70397/merge:refs/remotes/pull/70397/merge
---
2020-03-25T17:15:57.7560790Z  ---> 3fc1b512c57b
2020-03-25T17:15:57.7561002Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-25T17:15:57.7565663Z  ---> Using cache
2020-03-25T17:15:57.7566018Z  ---> 5ee4295733f4
2020-03-25T17:15:57.7567702Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-25T17:15:57.7575214Z  ---> 3d07a0fa42fe
2020-03-25T17:15:58.6213013Z Successfully built 3d07a0fa42fe
2020-03-25T17:15:58.6214681Z Successfully tagged rust-ci:latest
2020-03-25T17:15:58.6215294Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-25T17:19:52.4767869Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-25T17:19:52.9587313Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-25T17:19:54.8261193Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-25T17:19:55.4751167Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-25T17:19:55.5208597Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-25T17:19:57.1698883Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-25T17:20:18.2715822Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-25T17:20:21.4233820Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-25T17:20:22.2852006Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-25T17:21:47.0581333Z configure: build.locked-deps    := True
2020-03-25T17:21:47.0581723Z configure: llvm.ccache          := sccache
2020-03-25T17:21:47.0582243Z configure: build.cargo-native-static := True
2020-03-25T17:21:47.0583979Z configure: dist.missing-tools   := True
2020-03-25T17:21:47.0584655Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-25T17:21:47.0585371Z configure: writing `config.toml` in current directory
2020-03-25T17:21:47.0585844Z configure: 
2020-03-25T17:21:47.0586327Z configure: run `python /checkout/x.py --help`
2020-03-25T17:21:47.0586682Z configure: 
---
2020-03-25T17:23:12.5588084Z Hugepagesize:       2048 kB
2020-03-25T17:23:12.5588285Z DirectMap4k:      118720 kB
2020-03-25T17:23:12.5588504Z DirectMap2M:     4075520 kB
2020-03-25T17:23:12.5588706Z DirectMap1G:     5242880 kB
2020-03-25T17:23:12.5653884Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-25T17:23:13.9910391Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-25T17:23:13.9910391Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-25T17:23:13.9971573Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-25T17:23:14.2407790Z    Compiling unicode-xid v0.2.0
2020-03-25T17:23:14.3751738Z    Compiling syn v1.0.11
2020-03-25T17:23:15.3161287Z    Compiling linked-hash-map v0.5.2
2020-03-25T17:23:15.3161958Z    Compiling lazy_static v1.4.0
2020-03-25T17:23:15.3161958Z    Compiling lazy_static v1.4.0
2020-03-25T17:23:15.5675848Z    Compiling yaml-rust v0.4.3
2020-03-25T17:23:20.2645903Z    Compiling quote v1.0.2
2020-03-25T17:23:35.6761266Z    Compiling thiserror-impl v1.0.5
2020-03-25T17:23:40.4106222Z    Compiling thiserror v1.0.5
2020-03-25T17:23:40.4762451Z    Compiling yaml-merge-keys v0.4.0
2020-03-25T17:23:42.2886266Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-25T17:23:44.1196455Z Build completed successfully in 0:00:31
2020-03-25T17:23:44.1201800Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-25T17:23:44.3321982Z     Finished dev [unoptimized] target(s) in 0.15s
2020-03-25T17:23:45.3088428Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-25T17:25:53.1592744Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-25T17:25:53.6382632Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-25T17:25:55.6368705Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-25T17:25:56.3389582Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-25T17:25:56.3859553Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-25T17:25:58.0978894Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-25T17:26:20.0393827Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-25T17:26:23.2311641Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-25T17:26:23.2948669Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-25T17:30:07.8545554Z -use std::ptr;
2020-03-25T17:30:07.8546826Z  use std::convert::TryFrom;
2020-03-25T17:30:07.8547890Z +use std::ptr;
2020-03-25T17:30:07.8548328Z  
2020-03-25T17:30:07.8549272Z  use rustc::ty::layout::{Align, HasDataLayout, Size, TargetDataLayout};
2020-03-25T17:30:07.8549879Z  use rustc::ty::{self, query::TyCtxtAt, Instance, ParamEnv};
2020-03-25T17:30:07.8558965Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/interpret/memory.rs"` failed.
2020-03-25T17:30:07.8560739Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-25T17:30:07.8571901Z Build completed unsuccessfully in 0:00:41
2020-03-25T17:30:07.8621988Z == clock drift check ==
2020-03-25T17:30:07.8641655Z   local time: Wed Mar 25 17:30:07 UTC 2020
2020-03-25T17:30:08.1302098Z   network time: Wed, 25 Mar 2020 17:30:08 GMT
2020-03-25T17:30:08.1302098Z   network time: Wed, 25 Mar 2020 17:30:08 GMT
2020-03-25T17:30:08.1303273Z == end clock drift check ==
2020-03-25T17:30:09.7059214Z 
2020-03-25T17:30:09.7135229Z ##[error]Bash exited with code '1'.
2020-03-25T17:30:09.7149884Z ##[section]Finishing: Run build
2020-03-25T17:30:09.7202272Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70397/merge to s
2020-03-25T17:30:09.7206869Z Task         : Get sources
2020-03-25T17:30:09.7207229Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T17:30:09.7207542Z Version      : 1.0.0
2020-03-25T17:30:09.7207763Z Author       : Microsoft
2020-03-25T17:30:09.7207763Z Author       : Microsoft
2020-03-25T17:30:09.7208139Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-25T17:30:09.7208538Z ==============================================================================
2020-03-25T17:30:10.0665545Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-25T17:30:10.0706698Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70397/merge to s
2020-03-25T17:30:10.0795927Z Cleaning up task key
2020-03-25T17:30:10.0797235Z Start cleaning up orphan processes.
2020-03-25T17:30:10.0994384Z Terminate orphan process: pid (4755) (python)
2020-03-25T17:30:10.1312866Z ##[section]Finishing: Finalize Job
