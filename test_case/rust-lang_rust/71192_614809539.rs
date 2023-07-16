plain
2020-04-16T16:15:18.0997674Z ========================== Starting Command Output ===========================
2020-04-16T16:15:18.1000163Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/897e3bc2-a0a4-415e-beca-78bc53f36d79.sh
2020-04-16T16:15:18.1000475Z 
2020-04-16T16:15:18.1005074Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-16T16:15:18.1024067Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71192/merge to s
2020-04-16T16:15:18.1027427Z Task         : Get sources
2020-04-16T16:15:18.1027780Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T16:15:18.1028053Z Version      : 1.0.0
2020-04-16T16:15:18.1028235Z Author       : Microsoft
---
2020-04-16T16:15:19.2843525Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-16T16:15:19.2853881Z ##[command]git config gc.auto 0
2020-04-16T16:15:19.2860162Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-16T16:15:19.2865632Z ##[command]git config --get-all http.proxy
2020-04-16T16:15:20.2790508Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71192/merge:refs/remotes/pull/71192/merge
---
2020-04-16T16:17:41.6941524Z  ---> 78ad2f4d4aca
2020-04-16T16:17:41.6941747Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-16T16:17:41.6942120Z  ---> Using cache
2020-04-16T16:17:41.6942462Z  ---> 4d2dc61c4d00
2020-04-16T16:17:41.6949608Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-16T16:17:41.6950924Z  ---> 776b6266a8b7
2020-04-16T16:17:41.7064905Z Successfully built 776b6266a8b7
2020-04-16T16:17:41.7094942Z Successfully tagged rust-ci:latest
2020-04-16T16:17:41.7380616Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-16T16:17:41.7380616Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-16T16:17:41.7400201Z Looks like docker image is the same as before, not uploading
2020-04-16T16:17:48.6724694Z [CI_JOB_NAME=mingw-check]
2020-04-16T16:17:48.6981633Z [CI_JOB_NAME=mingw-check]
2020-04-16T16:17:48.7017017Z == clock drift check ==
2020-04-16T16:17:48.7029190Z   local time: Thu Apr 16 16:17:48 UTC 2020
2020-04-16T16:17:49.2952849Z   network time: Thu, 16 Apr 2020 16:17:49 GMT
2020-04-16T16:17:49.2981385Z Starting sccache server...
2020-04-16T16:17:50.3264257Z configure: processing command line
2020-04-16T16:17:50.3267000Z configure: 
2020-04-16T16:17:50.3268125Z configure: rust.parallel-compiler := True
---
2020-04-16T16:21:56.6914215Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-16T16:22:01.8813024Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-16T16:22:03.2544970Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T16:22:03.3156603Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T16:22:03.5360494Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T16:22:04.4160554Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T16:22:04.4968608Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-16T16:22:06.1863852Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T16:22:06.9405419Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-16T16:24:14.2158547Z configure: rust.verify-llvm-ir  := True
2020-04-16T16:24:14.2159308Z configure: build.locked-deps    := True
2020-04-16T16:24:14.2159841Z configure: llvm.ccache          := sccache
2020-04-16T16:24:14.2160562Z configure: rust.debug-assertions := True
2020-04-16T16:24:14.2161424Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-16T16:24:14.2162386Z configure: writing `config.toml` in current directory
2020-04-16T16:24:14.2162797Z configure: 
2020-04-16T16:24:14.2163441Z configure: run `python /checkout/x.py --help`
2020-04-16T16:24:14.2164183Z configure: 
---
2020-04-16T16:25:58.7879388Z Hugepagesize:       2048 kB
2020-04-16T16:25:58.7879594Z DirectMap4k:      143296 kB
2020-04-16T16:25:58.7879800Z DirectMap2M:     4050944 kB
2020-04-16T16:25:58.7880024Z DirectMap1G:     5242880 kB
2020-04-16T16:25:58.7915142Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-16T16:26:00.3415975Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-16T16:26:00.3415975Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-16T16:26:00.3423967Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-16T16:26:00.6169993Z    Compiling unicode-xid v0.2.0
2020-04-16T16:26:00.7543317Z    Compiling syn v1.0.11
2020-04-16T16:26:01.7149162Z    Compiling linked-hash-map v0.5.2
2020-04-16T16:26:01.7692062Z    Compiling lazy_static v1.4.0
2020-04-16T16:26:01.7692062Z    Compiling lazy_static v1.4.0
2020-04-16T16:26:01.9665360Z    Compiling yaml-rust v0.4.3
2020-04-16T16:26:06.7585088Z    Compiling quote v1.0.2
2020-04-16T16:26:23.1820864Z    Compiling thiserror-impl v1.0.5
2020-04-16T16:26:28.5676085Z    Compiling thiserror v1.0.5
2020-04-16T16:26:28.6334613Z    Compiling yaml-merge-keys v0.4.0
2020-04-16T16:26:29.9932865Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-16T16:26:32.3795614Z Build completed successfully in 0:00:33
2020-04-16T16:26:32.3797384Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-16T16:26:32.6759182Z     Finished dev [unoptimized] target(s) in 0.20s
2020-04-16T16:26:33.8873738Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-16T16:28:56.5764360Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T16:28:56.6607552Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T16:28:56.9033377Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T16:28:57.0467523Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T16:28:57.6470311Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T16:29:00.2120565Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T16:29:00.7784542Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T16:29:03.1618165Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T16:29:03.7331588Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T16:33:37.1140522Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-16T16:33:37.1140971Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-16T16:33:37.1142480Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-16T16:33:37.1143337Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-16T16:33:43.3623751Z Diff in /checkout/src/librustc_middle/mir/interpret/error.rs at line 454:
2020-04-16T16:33:43.3624128Z      ReadBytesAsPointer,
2020-04-16T16:33:43.3624283Z  
2020-04-16T16:33:43.3624704Z      // The variants below are only reachable from CTFE/const prop, miri will never emit them.
2020-04-16T16:33:43.3626145Z      /// Encountered a pointer where we needed raw bytes.
2020-04-16T16:33:43.3626646Z      ReadPointerAsBytes,
2020-04-16T16:33:43.3626878Z      /// Accessing thread local statics
2020-04-16T16:33:43.3626878Z      /// Accessing thread local statics
2020-04-16T16:33:43.3635490Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_middle/mir/interpret/error.rs"` failed.
2020-04-16T16:33:43.3640362Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-16T16:33:43.3654830Z Build completed unsuccessfully in 0:00:45
2020-04-16T16:33:43.3777484Z == clock drift check ==
2020-04-16T16:33:43.3777484Z == clock drift check ==
2020-04-16T16:33:43.3789217Z   local time: Thu Apr 16 16:33:43 UTC 2020
2020-04-16T16:33:43.6922571Z   network time: Thu, 16 Apr 2020 16:33:43 GMT
2020-04-16T16:33:44.9884039Z 
2020-04-16T16:33:44.9884039Z 
2020-04-16T16:33:44.9960714Z ##[error]Bash exited with code '1'.
2020-04-16T16:33:44.9991105Z ##[section]Finishing: Run build
2020-04-16T16:33:45.0037162Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71192/merge to s
2020-04-16T16:33:45.0042403Z Task         : Get sources
2020-04-16T16:33:45.0042726Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T16:33:45.0043021Z Version      : 1.0.0
2020-04-16T16:33:45.0043248Z Author       : Microsoft
2020-04-16T16:33:45.0043248Z Author       : Microsoft
2020-04-16T16:33:45.0043579Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-16T16:33:45.0044181Z ==============================================================================
2020-04-16T16:33:45.3887160Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-16T16:33:45.3963155Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71192/merge to s
2020-04-16T16:33:45.4062536Z Cleaning up task key
2020-04-16T16:33:45.4063811Z Start cleaning up orphan processes.
2020-04-16T16:33:45.4264092Z Terminate orphan process: pid (3558) (python)
2020-04-16T16:33:45.4471773Z ##[section]Finishing: Finalize Job
