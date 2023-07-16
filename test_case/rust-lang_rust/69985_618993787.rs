plain
2020-04-24T11:18:34.9441238Z ========================== Starting Command Output ===========================
2020-04-24T11:18:34.9443653Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dfc80c44-a538-41bc-90cb-18a2ae062008.sh
2020-04-24T11:18:34.9443887Z 
2020-04-24T11:18:34.9448153Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T11:18:34.9465928Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-04-24T11:18:34.9469030Z Task         : Get sources
2020-04-24T11:18:34.9469289Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T11:18:34.9469557Z Version      : 1.0.0
2020-04-24T11:18:34.9469733Z Author       : Microsoft
---
2020-04-24T11:18:35.9411191Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T11:18:35.9417609Z ##[command]git config gc.auto 0
2020-04-24T11:18:35.9421717Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T11:18:35.9425521Z ##[command]git config --get-all http.proxy
2020-04-24T11:18:35.9432758Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
---
2020-04-24T11:21:50.6829999Z  ---> f7353ccad5b1
2020-04-24T11:21:50.6830220Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T11:21:50.6833334Z  ---> Using cache
2020-04-24T11:21:50.6833702Z  ---> ed38efbaa060
2020-04-24T11:21:50.6834957Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T11:21:50.6841058Z  ---> c5008ef7ae8e
2020-04-24T11:21:50.6872540Z Successfully built c5008ef7ae8e
2020-04-24T11:21:50.6896181Z Successfully tagged rust-ci:latest
2020-04-24T11:21:50.7330106Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T11:21:50.7330106Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T11:21:50.7345220Z Looks like docker image is the same as before, not uploading
2020-04-24T11:21:52.1236640Z [CI_JOB_NAME=mingw-check]
2020-04-24T11:21:52.1432488Z [CI_JOB_NAME=mingw-check]
2020-04-24T11:21:52.1460162Z == clock drift check ==
2020-04-24T11:21:52.1470032Z   local time: Fri Apr 24 11:21:52 UTC 2020
2020-04-24T11:21:52.1852774Z   network time: Fri, 24 Apr 2020 11:21:52 GMT
2020-04-24T11:21:52.1889734Z Starting sccache server...
2020-04-24T11:21:52.2879724Z configure: processing command line
2020-04-24T11:21:52.2880021Z configure: 
2020-04-24T11:21:52.2880862Z configure: rust.parallel-compiler := True
---
2020-04-24T11:25:11.4597632Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T11:25:11.6464561Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T11:25:11.7962387Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T11:25:11.8132755Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T11:25:12.3021577Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T11:25:14.1523817Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T11:25:14.5375821Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T11:25:16.2483743Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T11:25:16.6141960Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T11:26:47.1709655Z configure: rust.debug-assertions := True
2020-04-24T11:26:47.1710014Z configure: build.submodules     := False
2020-04-24T11:26:47.1710378Z configure: llvm.assertions      := True
2020-04-24T11:26:47.1710905Z configure: rust.dist-src        := False
2020-04-24T11:26:47.1711543Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-24T11:26:47.1712241Z configure: writing `config.toml` in current directory
2020-04-24T11:26:47.1712565Z configure: 
2020-04-24T11:26:47.1713055Z configure: run `python /checkout/x.py --help`
2020-04-24T11:26:47.1713363Z configure: 
---
2020-04-24T11:28:04.1394751Z Hugepagesize:       2048 kB
2020-04-24T11:28:04.1395002Z DirectMap4k:      141248 kB
2020-04-24T11:28:04.1395317Z DirectMap2M:     4052992 kB
2020-04-24T11:28:04.1395556Z DirectMap1G:     5242880 kB
2020-04-24T11:28:04.1396521Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-24T11:28:04.5167229Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T11:28:04.5167229Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T11:28:04.5175901Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-24T11:28:04.7152006Z    Compiling unicode-xid v0.2.0
2020-04-24T11:28:04.8235576Z    Compiling syn v1.0.11
2020-04-24T11:28:05.5294713Z    Compiling linked-hash-map v0.5.2
2020-04-24T11:28:05.5672011Z    Compiling lazy_static v1.4.0
2020-04-24T11:28:05.5672011Z    Compiling lazy_static v1.4.0
2020-04-24T11:28:05.7279038Z    Compiling yaml-rust v0.4.3
2020-04-24T11:28:09.2868668Z    Compiling quote v1.0.2
2020-04-24T11:28:20.9473915Z    Compiling thiserror-impl v1.0.5
2020-04-24T11:28:24.7382984Z    Compiling thiserror v1.0.5
2020-04-24T11:28:24.7867587Z    Compiling yaml-merge-keys v0.4.0
2020-04-24T11:28:25.7050958Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-24T11:28:27.0354994Z Build completed successfully in 0:00:23
2020-04-24T11:28:27.0430405Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-24T11:28:27.2567766Z     Finished dev [unoptimized] target(s) in 0.14s
2020-04-24T11:28:28.1610905Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-24T11:30:09.6327581Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T11:30:09.7637224Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T11:30:09.9307659Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T11:30:09.9796208Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T11:30:10.4317411Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T11:30:12.2529171Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T11:30:12.6418366Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T11:30:14.2715135Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T11:30:14.6318400Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T11:33:24.4009350Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-24T11:33:24.4009958Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-24T11:33:24.4010479Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-24T11:33:24.4015111Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-24T11:33:28.0937669Z Diff in /checkout/src/libcore/array/mod.rs at line 221:
2020-04-24T11:33:28.0941683Z  {
2020-04-24T11:33:28.0945501Z      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
2020-04-24T11:33:28.0949158Z          fmt::Display::fmt(
2020-04-24T11:33:28.0952661Z -            &format_args!(
2020-04-24T11:33:28.0956095Z -                "The iterator only returned {} items, but {} were needed",
2020-04-24T11:33:28.0959189Z -                self.len(),
2020-04-24T11:33:28.0965158Z -            ),
2020-04-24T11:33:28.0965158Z -            ),
2020-04-24T11:33:28.0968832Z +            &format_args!("The iterator only returned {} items, but {} were needed", self.len(), N),
2020-04-24T11:33:28.0972351Z              f,
2020-04-24T11:33:28.0981460Z      }
2020-04-24T11:33:28.0981460Z      }
2020-04-24T11:33:28.0994731Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libcore/array/mod.rs"` failed.
2020-04-24T11:33:28.0998716Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-24T11:33:28.0999372Z Build completed unsuccessfully in 0:00:31
2020-04-24T11:33:28.1116774Z == clock drift check ==
2020-04-24T11:33:28.1116774Z == clock drift check ==
2020-04-24T11:33:28.1130216Z   local time: Fri Apr 24 11:33:28 UTC 2020
2020-04-24T11:33:28.4000226Z   network time: Fri, 24 Apr 2020 11:33:28 GMT
2020-04-24T11:33:30.0826950Z 
2020-04-24T11:33:30.0826950Z 
2020-04-24T11:33:30.0865432Z ##[error]Bash exited with code '1'.
2020-04-24T11:33:30.0877755Z ##[section]Finishing: Run build
2020-04-24T11:33:30.0913161Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-04-24T11:33:30.0920359Z Task         : Get sources
2020-04-24T11:33:30.0920669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T11:33:30.0920953Z Version      : 1.0.0
2020-04-24T11:33:30.0921176Z Author       : Microsoft
2020-04-24T11:33:30.0921176Z Author       : Microsoft
2020-04-24T11:33:30.0921493Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T11:33:30.0921863Z ==============================================================================
2020-04-24T11:33:30.3528807Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T11:33:30.3566215Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-04-24T11:33:30.3646258Z Cleaning up task key
2020-04-24T11:33:30.3647194Z Start cleaning up orphan processes.
2020-04-24T11:33:30.3780970Z Terminate orphan process: pid (3667) (python)
2020-04-24T11:33:30.4087420Z ##[section]Finishing: Finalize Job
