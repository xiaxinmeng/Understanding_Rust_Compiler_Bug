plain
2020-04-27T19:14:34.0065083Z ========================== Starting Command Output ===========================
2020-04-27T19:14:34.0070414Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4106671c-54ef-4e5f-8273-80d978e1b90b.sh
2020-04-27T19:14:34.0070706Z 
2020-04-27T19:14:34.0076705Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T19:14:34.0093925Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71617/merge to s
2020-04-27T19:14:34.0097069Z Task         : Get sources
2020-04-27T19:14:34.0097332Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T19:14:34.0097563Z Version      : 1.0.0
2020-04-27T19:14:34.0097723Z Author       : Microsoft
---
2020-04-27T19:14:34.9941926Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T19:14:34.9948081Z ##[command]git config gc.auto 0
2020-04-27T19:14:34.9952594Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T19:14:34.9956726Z ##[command]git config --get-all http.proxy
2020-04-27T19:14:34.9964366Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71617/merge:refs/remotes/pull/71617/merge
---
2020-04-27T19:16:48.7567530Z  ---> f7353ccad5b1
2020-04-27T19:16:48.7567775Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-27T19:16:48.7568205Z  ---> Using cache
2020-04-27T19:16:48.7568613Z  ---> ed38efbaa060
2020-04-27T19:16:48.7570027Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-27T19:16:48.7620155Z  ---> c5008ef7ae8e
2020-04-27T19:16:48.7652168Z Successfully built c5008ef7ae8e
2020-04-27T19:16:48.7695721Z Successfully tagged rust-ci:latest
2020-04-27T19:16:48.8087933Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T19:16:48.8087933Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T19:16:48.8103599Z Looks like docker image is the same as before, not uploading
2020-04-27T19:16:56.7801804Z [CI_JOB_NAME=mingw-check]
2020-04-27T19:16:56.8093971Z [CI_JOB_NAME=mingw-check]
2020-04-27T19:16:56.8108220Z == clock drift check ==
2020-04-27T19:16:56.8114876Z   local time: Mon Apr 27 19:16:56 UTC 2020
2020-04-27T19:16:56.9085892Z   network time: Mon, 27 Apr 2020 19:16:56 GMT
2020-04-27T19:16:56.9115290Z Starting sccache server...
2020-04-27T19:16:57.0244845Z configure: processing command line
2020-04-27T19:16:57.0246930Z configure: 
2020-04-27T19:16:57.0247997Z configure: rust.parallel-compiler := True
---
2020-04-27T19:19:10.4855043Z     Checking unicode-width v0.1.6
2020-04-27T19:19:10.5759944Z     Checking getopts v0.2.21
2020-04-27T19:19:12.7774340Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-27T19:19:13.4839688Z     Finished release [optimized] target(s) in 30.66s
2020-04-27T19:19:13.4840713Z {"reason":"build-finished","success":true}
2020-04-27T19:19:14.0388499Z     Checking cfg-if v0.1.10
2020-04-27T19:19:14.0389097Z    Compiling libc v0.2.69
2020-04-27T19:19:14.0737880Z    Compiling semver-parser v0.7.0
2020-04-27T19:19:14.9920530Z     Checking scopeguard v1.0.0
---
2020-04-27T19:20:55.0941642Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T19:20:55.2132597Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T19:20:55.4111723Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T19:20:55.5230727Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T19:20:56.0058425Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T19:20:58.3943076Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T19:20:58.8579638Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T19:21:00.9218500Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T19:21:01.3616963Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T19:21:40.4414742Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T19:21:40.5977951Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T19:21:52.7921604Z     Checking rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T19:21:52.8791793Z     Finished release [optimized] target(s) in 2m 39s
2020-04-27T19:21:52.8796286Z {"reason":"build-finished","success":true}
2020-04-27T19:21:53.1839762Z     Checking cfg-if v0.1.10
2020-04-27T19:21:53.1840308Z    Compiling libc v0.2.69
2020-04-27T19:21:53.2207631Z    Compiling semver-parser v0.7.0
2020-04-27T19:21:54.0819679Z    Compiling proc-macro2 v0.4.30
---
2020-04-27T19:22:47.8819846Z     Checking serde_json v1.0.40
2020-04-27T19:22:49.2190321Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-04-27T19:22:53.9083301Z     Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2020-04-27T19:22:54.0126900Z     Finished release [optimized] target(s) in 1m 01s
2020-04-27T19:22:54.0128172Z {"reason":"build-finished","success":true}
2020-04-27T19:22:54.2657000Z configure: processing command line
2020-04-27T19:22:54.2657675Z configure: 
2020-04-27T19:22:54.2658658Z configure: rust.codegen-units-std := 1
2020-04-27T19:22:54.2659491Z configure: rust.verify-llvm-ir  := True
---
2020-04-27T19:22:54.2662378Z configure: rust.dist-src        := False
2020-04-27T19:22:54.2662685Z configure: llvm.ccache          := sccache
2020-04-27T19:22:54.2663196Z configure: dist.missing-tools   := True
2020-04-27T19:22:54.2663512Z configure: llvm.assertions      := True
2020-04-27T19:22:54.2664154Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-27T19:22:54.2664735Z configure: writing `config.toml` in current directory
2020-04-27T19:22:54.2664978Z configure: 
2020-04-27T19:22:54.2665433Z configure: run `python /checkout/x.py --help`
2020-04-27T19:22:54.2665691Z configure: 
---
2020-04-27T19:24:27.6853250Z Hugepagesize:       2048 kB
2020-04-27T19:24:27.6853605Z DirectMap4k:      151488 kB
2020-04-27T19:24:27.6853937Z DirectMap2M:     4042752 kB
2020-04-27T19:24:27.6854286Z DirectMap1G:     5242880 kB
2020-04-27T19:24:27.6915202Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-27T19:24:29.1767145Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-27T19:24:29.1767145Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-27T19:24:29.1780186Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-27T19:24:29.4579728Z    Compiling unicode-xid v0.2.0
2020-04-27T19:24:29.5819564Z    Compiling syn v1.0.11
2020-04-27T19:24:30.4574569Z    Compiling linked-hash-map v0.5.2
2020-04-27T19:24:30.4756377Z    Compiling lazy_static v1.4.0
2020-04-27T19:24:30.4756377Z    Compiling lazy_static v1.4.0
2020-04-27T19:24:30.6868347Z    Compiling yaml-rust v0.4.3
2020-04-27T19:24:35.1995842Z    Compiling quote v1.0.2
2020-04-27T19:24:50.0223535Z    Compiling thiserror-impl v1.0.5
2020-04-27T19:24:55.0381966Z    Compiling thiserror v1.0.5
2020-04-27T19:24:55.0941794Z    Compiling yaml-merge-keys v0.4.0
2020-04-27T19:24:56.3503732Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-27T19:24:58.0613308Z     Finished release [optimized] target(s) in 28.87s
2020-04-27T19:24:58.0615829Z {"reason":"build-finished","success":true}
2020-04-27T19:24:59.4482057Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-27T19:24:59.7879792Z     Finished dev [unoptimized] target(s) in 0.23s
2020-04-27T19:25:01.6816439Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
2020-04-27T19:25:01.6817574Z Checking std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-27T19:25:25.9110052Z     Checking unicode-width v0.1.6
2020-04-27T19:25:25.9900990Z     Checking getopts v0.2.21
2020-04-27T19:25:28.2209193Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-27T19:25:29.0208676Z     Finished release [optimized] target(s) in 27.99s
2020-04-27T19:25:29.0209262Z {"reason":"build-finished","success":true}
2020-04-27T19:25:29.7205651Z     Checking cfg-if v0.1.10
2020-04-27T19:25:29.7206359Z    Compiling semver-parser v0.7.0
2020-04-27T19:25:29.7629950Z    Compiling winapi-i686-pc-windows-gnu v0.4.0
2020-04-27T19:25:30.1284257Z    Compiling winapi v0.3.8
---
2020-04-27T19:27:08.1789784Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-27T19:27:13.0200222Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-27T19:27:14.2792233Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T19:27:14.3263674Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T19:27:14.5375800Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T19:27:15.3256643Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T19:27:15.3757247Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-27T19:27:16.9003410Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T19:27:17.7148578Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-27T19:28:01.6109527Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T19:28:01.7670604Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T19:28:11.6084904Z     Checking rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T19:28:11.6963756Z     Finished release [optimized] target(s) in 2m 42s
2020-04-27T19:28:11.6967022Z {"reason":"build-finished","success":true}
2020-04-27T19:28:12.0184008Z     Checking cfg-if v0.1.10
2020-04-27T19:28:12.0185498Z    Compiling semver-parser v0.7.0
2020-04-27T19:28:12.0594024Z    Compiling proc-macro2 v0.4.30
2020-04-27T19:28:13.1733767Z    Compiling getrandom v0.1.14
---
2020-04-27T19:28:42.1764983Z    Compiling serde_derive v1.0.81
2020-04-27T19:29:09.3323128Z     Checking serde_json v1.0.40
2020-04-27T19:29:10.5678225Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-04-27T19:29:15.5263705Z     Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2020-04-27T19:29:15.6393321Z     Finished {"reason":"build-finished","success":true}
2020-04-27T19:29:15.6550812Z Build completed successfully in 0:04:16
2020-04-27T19:29:15.6654656Z + python3 ../x.py build --stage 0 src/tools/build-manifest
2020-04-27T19:29:15.9781795Z     Finished dev [unoptimized] target(s) in 0.21s
2020-04-27T19:29:17.1601515Z Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
---
2020-04-27T19:30:01.6379815Z    Compiling toml v0.5.3
2020-04-27T19:30:02.3312460Z    Compiling serde_json v1.0.40
2020-04-27T19:30:09.8915630Z    Compiling build-manifest v0.1.0 (/checkout/src/tools/build-manifest)
2020-04-27T19:30:14.4326468Z     Finished release [optimized] target(s) in 57.27s
2020-04-27T19:30:14.4327954Z {"reason":"build-finished","success":true}
2020-04-27T19:30:14.4524706Z + python3 ../x.py test --stage 0 src/tools/compiletest
2020-04-27T19:30:14.7530945Z     Finished dev [unoptimized] target(s) in 0.21s
2020-04-27T19:30:16.1393966Z    Compiling memchr v2.3.2
2020-04-27T19:30:16.1400878Z    Compiling log v0.4.8
---
2020-04-27T19:31:08.4354248Z    Compiling semver v0.9.0
2020-04-27T19:31:10.1622389Z    Compiling cargo_metadata v0.9.1
2020-04-27T19:31:13.9222775Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-27T19:31:25.4220524Z     Finished release [optimized] target(s) in 26.39s
2020-04-27T19:31:25.4221480Z {"reason":"build-finished","success":true}
2020-04-27T19:31:32.2890970Z * 596 error codes
2020-04-27T19:31:32.2891209Z * highest error code: E0753
2020-04-27T19:31:32.6397028Z * 283 features
2020-04-27T19:31:34.6888503Z Checking which error codes lack tests...
---
2020-04-27T19:31:37.0380938Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-27T19:31:37.0381303Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-27T19:31:37.0381921Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-27T19:31:37.0382463Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-27T19:31:40.6544589Z Diff in /checkout/src/librustc_typeck/check/demand.rs at line 771:
2020-04-27T19:31:40.6549482Z                      let is_fallible = match (exp.bit_width(), found.bit_width()) {
2020-04-27T19:31:40.6553892Z                          (Some(exp), Some(found)) if found < exp => false,
2020-04-27T19:31:40.6558192Z                          (None, Some(found)) if found <= 16 => false,
2020-04-27T19:31:40.6568964Z +                        _ => true,
2020-04-27T19:31:40.6573618Z                      };
2020-04-27T19:31:40.6573618Z                      };
2020-04-27T19:31:40.6577855Z                      suggest_to_change_suffix_or_into(err, is_fallible);
2020-04-27T19:31:40.6594492Z Diff in /checkout/src/librustc_typeck/check/demand.rs at line 778:
2020-04-27T19:31:40.6595115Z -                },
2020-04-27T19:31:40.6595294Z +                }
2020-04-27T19:31:40.6595532Z                  (&ty::Uint(_), &ty::Int(_)) => {
2020-04-27T19:31:40.6595532Z                  (&ty::Uint(_), &ty::Int(_)) => {
2020-04-27T19:31:40.6595855Z                      suggest_to_change_suffix_or_into(err, true);
2020-04-27T19:31:40.6596102Z                      true
2020-04-27T19:31:40.6614885Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_typeck/check/demand.rs"` failed.
2020-04-27T19:31:40.6615857Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-27T19:31:40.6616669Z Build completed unsuccessfully in 0:00:43
2020-04-27T19:31:40.6745613Z == clock drift check ==
2020-04-27T19:31:40.6766234Z   local time: Mon Apr 27 19:31:40 UTC 2020
2020-04-27T19:31:40.6766234Z   local time: Mon Apr 27 19:31:40 UTC 2020
2020-04-27T19:31:40.8371645Z   network time: Mon, 27 Apr 2020 19:31:40 GMT
2020-04-27T19:31:42.3147728Z 
2020-04-27T19:31:42.3147728Z 
2020-04-27T19:31:42.3236629Z ##[error]Bash exited with code '1'.
2020-04-27T19:31:42.3251057Z ##[section]Finishing: Run build
2020-04-27T19:31:42.3359073Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71617/merge to s
2020-04-27T19:31:42.3364388Z Task         : Get sources
2020-04-27T19:31:42.3364753Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T19:31:42.3365093Z Version      : 1.0.0
2020-04-27T19:31:42.3365513Z Author       : Microsoft
2020-04-27T19:31:42.3365513Z Author       : Microsoft
2020-04-27T19:31:42.3365881Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T19:31:42.3366313Z ==============================================================================
2020-04-27T19:31:42.7078223Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T19:31:42.7121431Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71617/merge to s
2020-04-27T19:31:42.7238843Z Cleaning up task key
2020-04-27T19:31:42.7240031Z Start cleaning up orphan processes.
2020-04-27T19:31:42.7432925Z Terminate orphan process: pid (3634) (python)
2020-04-27T19:31:42.7698587Z ##[section]Finishing: Finalize Job
