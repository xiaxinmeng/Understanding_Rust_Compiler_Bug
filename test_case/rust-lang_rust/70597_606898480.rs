plain
2020-03-31T20:00:59.1700188Z ========================== Starting Command Output ===========================
2020-03-31T20:00:59.1718684Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dfb719b5-cfbd-4796-a2d8-5777d6f09130.sh
2020-03-31T20:00:59.1967148Z 
2020-03-31T20:00:59.2026076Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T20:00:59.2046618Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70597/merge to s
2020-03-31T20:00:59.2050188Z Task         : Get sources
2020-03-31T20:00:59.2050498Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T20:00:59.2050827Z Version      : 1.0.0
2020-03-31T20:00:59.2051032Z Author       : Microsoft
---
2020-03-31T20:01:00.4351359Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T20:01:00.4359974Z ##[command]git config gc.auto 0
2020-03-31T20:01:00.4364595Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T20:01:00.4369341Z ##[command]git config --get-all http.proxy
2020-03-31T20:01:00.4377780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70597/merge:refs/remotes/pull/70597/merge
---
2020-03-31T20:03:17.9427690Z  ---> 3fc1b512c57b
2020-03-31T20:03:17.9428046Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-31T20:03:17.9466073Z  ---> Using cache
2020-03-31T20:03:17.9467742Z  ---> 5ee4295733f4
2020-03-31T20:03:17.9469390Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-31T20:03:17.9470824Z  ---> 3d07a0fa42fe
2020-03-31T20:03:17.9522498Z Successfully built 3d07a0fa42fe
2020-03-31T20:03:17.9560202Z Successfully tagged rust-ci:latest
2020-03-31T20:03:17.9823892Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-31T20:07:00.7678046Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T20:07:00.8039895Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T20:07:00.9885590Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T20:07:01.1551727Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T20:07:01.5690308Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T20:07:03.7683051Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T20:07:04.2206219Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T20:07:06.1773683Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T20:07:06.6066138Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T20:08:50.4546073Z configure: build.locked-deps    := True
2020-03-31T20:08:50.4546386Z configure: llvm.ccache          := sccache
2020-03-31T20:08:50.4546865Z configure: build.cargo-native-static := True
2020-03-31T20:08:50.4547335Z configure: dist.missing-tools   := True
2020-03-31T20:08:50.4547960Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-31T20:08:50.4548525Z configure: writing `config.toml` in current directory
2020-03-31T20:08:50.4548767Z configure: 
2020-03-31T20:08:50.4549183Z configure: run `python /checkout/x.py --help`
2020-03-31T20:08:50.4549411Z configure: 
---
2020-03-31T20:10:10.7188385Z Hugepagesize:       2048 kB
2020-03-31T20:10:10.7188611Z DirectMap4k:      135104 kB
2020-03-31T20:10:10.7188837Z DirectMap2M:     4059136 kB
2020-03-31T20:10:10.7189078Z DirectMap1G:     5242880 kB
2020-03-31T20:10:10.7189965Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-31T20:10:12.0598992Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-31T20:10:12.0598992Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-31T20:10:12.0600626Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-31T20:10:12.2956147Z    Compiling unicode-xid v0.2.0
2020-03-31T20:10:12.4199588Z    Compiling syn v1.0.11
2020-03-31T20:10:13.2243792Z    Compiling linked-hash-map v0.5.2
2020-03-31T20:10:13.3014915Z    Compiling lazy_static v1.4.0
2020-03-31T20:10:13.3014915Z    Compiling lazy_static v1.4.0
2020-03-31T20:10:13.4483036Z    Compiling yaml-rust v0.4.3
2020-03-31T20:10:17.6856072Z    Compiling quote v1.0.2
2020-03-31T20:10:31.5402343Z    Compiling thiserror-impl v1.0.5
2020-03-31T20:10:36.0403954Z    Compiling thiserror v1.0.5
2020-03-31T20:10:36.1019646Z    Compiling yaml-merge-keys v0.4.0
2020-03-31T20:10:37.2407655Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-31T20:10:40.3249429Z Build completed successfully in 0:00:29
2020-03-31T20:10:40.3286307Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-31T20:10:40.5712141Z     Finished dev [unoptimized] target(s) in 0.18s
2020-03-31T20:10:41.6630173Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-31T20:12:41.6211625Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T20:12:41.7385756Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T20:12:41.9329737Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T20:12:42.0628960Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T20:12:42.5341787Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T20:12:44.7312855Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T20:12:45.1915115Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T20:12:47.2231936Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T20:12:47.7323771Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T20:16:37.3777495Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-31T20:16:37.3780695Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-31T20:16:37.3785318Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-31T20:16:37.3788545Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-31T20:16:42.9206921Z Diff in /checkout/src/libstd/sys/hermit/thread.rs at line 5:
2020-03-31T20:16:42.9207322Z  use crate::io;
2020-03-31T20:16:42.9207516Z  use crate::mem;
2020-03-31T20:16:42.9216093Z  use crate::sys::hermit::abi;
2020-03-31T20:16:42.9234445Z -use crate::sys::{stack_overflow};
2020-03-31T20:16:42.9234725Z +use crate::sys::stack_overflow;
2020-03-31T20:16:42.9235432Z  use core::u32;
2020-03-31T20:16:42.9236402Z  
2020-03-31T20:16:42.9236402Z  
2020-03-31T20:16:42.9237807Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/sys/hermit/thread.rs"` failed.
2020-03-31T20:16:42.9238884Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-31T20:16:42.9267200Z Build completed unsuccessfully in 0:00:36
2020-03-31T20:16:42.9316561Z == clock drift check ==
2020-03-31T20:16:42.9331868Z   local time: Tue Mar 31 20:16:42 UTC 2020
2020-03-31T20:16:42.9544296Z   network time: Tue, 31 Mar 2020 20:16:42 GMT
2020-03-31T20:16:42.9544296Z   network time: Tue, 31 Mar 2020 20:16:42 GMT
2020-03-31T20:16:42.9547230Z == end clock drift check ==
2020-03-31T20:16:44.7238794Z 
2020-03-31T20:16:44.7303771Z ##[error]Bash exited with code '1'.
2020-03-31T20:16:44.7316754Z ##[section]Finishing: Run build
2020-03-31T20:16:44.7364408Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70597/merge to s
2020-03-31T20:16:44.7369192Z Task         : Get sources
2020-03-31T20:16:44.7369549Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T20:16:44.7369872Z Version      : 1.0.0
2020-03-31T20:16:44.7370116Z Author       : Microsoft
2020-03-31T20:16:44.7370116Z Author       : Microsoft
2020-03-31T20:16:44.7370483Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T20:16:44.7370912Z ==============================================================================
2020-03-31T20:16:45.0669388Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T20:16:45.0715157Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70597/merge to s
2020-03-31T20:16:45.0807660Z Cleaning up task key
2020-03-31T20:16:45.0808729Z Start cleaning up orphan processes.
2020-03-31T20:16:45.0984032Z Terminate orphan process: pid (8193) (python)
2020-03-31T20:16:45.1128702Z ##[section]Finishing: Finalize Job
