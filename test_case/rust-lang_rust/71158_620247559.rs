plain
2020-04-27T20:23:15.2533744Z ========================== Starting Command Output ===========================
2020-04-27T20:23:15.2537251Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3562ae0b-fb6b-40e6-9b03-1e9f2a60a8da.sh
2020-04-27T20:23:15.2537544Z 
2020-04-27T20:23:15.2542573Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T20:23:15.2560207Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T20:23:15.2563266Z Task         : Get sources
2020-04-27T20:23:15.2563546Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T20:23:15.2563837Z Version      : 1.0.0
2020-04-27T20:23:15.2564027Z Author       : Microsoft
---
2020-04-27T20:23:16.2433087Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T20:23:16.2439886Z ##[command]git config gc.auto 0
2020-04-27T20:23:16.2444044Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T20:23:16.2447895Z ##[command]git config --get-all http.proxy
2020-04-27T20:23:16.2455029Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71158/merge:refs/remotes/pull/71158/merge
---
2020-04-27T20:26:16.6226931Z  ---> f7353ccad5b1
2020-04-27T20:26:16.6227192Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-27T20:26:16.6231028Z  ---> Using cache
2020-04-27T20:26:16.6231454Z  ---> ed38efbaa060
2020-04-27T20:26:16.6232913Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-27T20:26:16.6236561Z  ---> c5008ef7ae8e
2020-04-27T20:26:16.6299799Z Successfully built c5008ef7ae8e
2020-04-27T20:26:16.6327782Z Successfully tagged rust-ci:latest
2020-04-27T20:26:16.6796223Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T20:26:16.6796223Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T20:26:16.6821356Z Looks like docker image is the same as before, not uploading
2020-04-27T20:26:24.7822284Z [CI_JOB_NAME=mingw-check]
2020-04-27T20:26:24.8102531Z [CI_JOB_NAME=mingw-check]
2020-04-27T20:26:24.8129047Z == clock drift check ==
2020-04-27T20:26:24.8129310Z   local time: Mon Apr 27 20:26:24 UTC 2020
2020-04-27T20:26:24.9728504Z   network time: Mon, 27 Apr 2020 20:26:24 GMT
2020-04-27T20:26:24.9774316Z Starting sccache server...
2020-04-27T20:26:25.0936241Z configure: processing command line
2020-04-27T20:26:25.0936474Z configure: 
2020-04-27T20:26:25.0937582Z configure: rust.parallel-compiler := True
---
2020-04-27T20:28:33.9290855Z     Checking term v0.0.0 (/checkout/src/libterm)
2020-04-27T20:28:34.2098259Z     Checking unicode-width v0.1.6
2020-04-27T20:28:34.3084085Z     Checking getopts v0.2.21
2020-04-27T20:28:36.2435961Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-27T20:28:36.9343537Z {"reason":"build-finished","success":true}
2020-04-27T20:28:36.9579166Z Checking compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T20:28:37.5367640Z     Checking cfg-if v0.1.10
2020-04-27T20:28:37.5368159Z    Compiling libc v0.2.69
2020-04-27T20:28:37.5682475Z    Compiling semver-parser v0.7.0
---
2020-04-27T20:30:11.6247415Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T20:30:11.7295561Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T20:30:11.9151467Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T20:30:12.0453000Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T20:30:12.5138801Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T20:30:14.8277970Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T20:30:15.2948363Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T20:30:17.2952284Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T20:30:17.7174000Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T20:30:56.9386315Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T20:31:00.5159810Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T20:31:03.5663860Z     Checking rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T20:31:03.6559968Z     Finished release [optimized] target(s) in 2m 26s
2020-04-27T20:31:03.6562346Z {"reason":"build-finished","success":true}
2020-04-27T20:31:03.9610386Z     Checking cfg-if v0.1.10
2020-04-27T20:31:03.9610941Z    Compiling libc v0.2.69
2020-04-27T20:31:04.0006647Z    Compiling semver-parser v0.7.0
2020-04-27T20:31:04.8330801Z    Compiling proc-macro2 v0.4.30
---
2020-04-27T20:31:56.6668153Z     Checking serde_json v1.0.40
2020-04-27T20:31:57.9286180Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-04-27T20:32:02.7223220Z     Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2020-04-27T20:32:02.8228673Z     Finished release [optimized] target(s) in 59.10s
2020-04-27T20:32:02.8235940Z {"reason":"build-finished","success":true}
2020-04-27T20:32:03.0952642Z configure: processing command line
2020-04-27T20:32:03.0953397Z configure: 
2020-04-27T20:32:03.0953831Z configure: build.submodules     := False
2020-04-27T20:32:03.0954938Z configure: rust.dist-src        := False
---
2020-04-27T20:32:03.0958232Z configure: rust.debug-assertions := True
2020-04-27T20:32:03.0958881Z configure: dist.missing-tools   := True
2020-04-27T20:32:03.0959524Z configure: rust.codegen-units-std := 1
2020-04-27T20:32:03.0960990Z configure: rust.verify-llvm-ir  := True
2020-04-27T20:32:03.0961777Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-27T20:32:03.0962777Z configure: writing `config.toml` in current directory
2020-04-27T20:32:03.0963099Z configure: 
2020-04-27T20:32:03.0963645Z configure: run `python /checkout/x.py --help`
2020-04-27T20:32:03.0963976Z configure: 
---
2020-04-27T20:33:38.4459386Z Hugepagesize:       2048 kB
2020-04-27T20:33:38.4459583Z DirectMap4k:      143296 kB
2020-04-27T20:33:38.4459794Z DirectMap2M:     5099520 kB
2020-04-27T20:33:38.4460822Z DirectMap1G:     4194304 kB
2020-04-27T20:33:38.4485923Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-27T20:33:39.9139966Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-27T20:33:39.9139966Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-27T20:33:39.9219868Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-27T20:33:40.1761250Z    Compiling unicode-xid v0.2.0
2020-04-27T20:33:40.3020682Z    Compiling syn v1.0.11
2020-04-27T20:33:41.1458670Z    Compiling linked-hash-map v0.5.2
2020-04-27T20:33:41.1802823Z    Compiling lazy_static v1.4.0
2020-04-27T20:33:41.1802823Z    Compiling lazy_static v1.4.0
2020-04-27T20:33:41.3731734Z    Compiling yaml-rust v0.4.3
2020-04-27T20:33:45.5346145Z    Compiling quote v1.0.2
2020-04-27T20:33:59.3878552Z    Compiling thiserror-impl v1.0.5
2020-04-27T20:34:04.0253829Z    Compiling thiserror v1.0.5
2020-04-27T20:34:04.0823749Z    Compiling yaml-merge-keys v0.4.0
2020-04-27T20:34:05.2716044Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-27T20:34:07.3119873Z    {"reason":"build-finished","success":true}
2020-04-27T20:34:07.3355441Z Build completed successfully in 0:00:28
2020-04-27T20:34:07.3456470Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-27T20:34:07.6483202Z     Finished dev [unoptimized] target(s) in 0.21s
2020-04-27T20:34:08.8180027Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-27T20:34:32.3026391Z     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
2020-04-27T20:34:32.5936812Z     Checking unicode-width v0.1.6
2020-04-27T20:34:32.6717475Z     Checking getopts v0.2.21
2020-04-27T20:34:34.8130025Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-27T20:34:35.5018136Z {"reason":"build-finished","success":true}
2020-04-27T20:34:35.5159197Z Checking compiler artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
2020-04-27T20:34:36.1475073Z     Checking cfg-if v0.1.10
2020-04-27T20:34:36.1483710Z    Compiling semver-parser v0.7.0
2020-04-27T20:34:36.1918774Z    Compiling winapi-i686-pc-windows-gnu v0.4.0
---
2020-04-27T20:36:14.8766263Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T20:36:15.0076247Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T20:36:15.1966178Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T20:36:15.2722354Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T20:36:15.8005911Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T20:36:18.0761342Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T20:36:18.5548475Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T20:36:20.5863251Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T20:36:21.0367855Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T20:37:02.0758536Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T20:37:02.2494547Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T20:37:06.4405497Z     Checking rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T20:37:06.5300576Z     Finished release [optimized] target(s) in 2m 31s
2020-04-27T20:37:06.5305091Z {"reason":"build-finished","success":true}
2020-04-27T20:37:06.8479094Z     Checking cfg-if v0.1.10
2020-04-27T20:37:06.8484401Z    Compiling semver-parser v0.7.0
2020-04-27T20:37:06.8858353Z    Compiling proc-macro2 v0.4.30
2020-04-27T20:37:07.8312019Z     Checking lazy_static v1.4.0
---
2020-04-27T20:37:35.2723907Z    Compiling serde_derive v1.0.81
2020-04-27T20:38:01.1481262Z     Checking serde_json v1.0.40
2020-04-27T20:38:02.3629304Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-04-27T20:38:07.2386321Z     Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2020-04-27T20:38:07.3415750Z     Finished release [optimized] target(s) in 1m 00s{"reason":"build-finished","success":true}
2020-04-27T20:38:07.3611619Z Build completed successfully in 0:03:59
2020-04-27T20:38:07.3729229Z + python3 ../x.py build --stage 0 src/tools/build-manifest
2020-04-27T20:38:07.6700821Z     Finished dev [unoptimized] target(s) in 0.20s
2020-04-27T20:38:08.8557104Z Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
---
2020-04-27T20:38:51.3468074Z    Compiling serde_json v1.0.40
2020-04-27T20:38:51.9529232Z    Compiling toml v0.5.3
2020-04-27T20:38:58.9910934Z    Compiling build-manifest v0.1.0 (/checkout/src/tools/build-manifest)
2020-04-27T20:39:03.3969330Z     Finished release [optimized] target(s) in 54.53s
2020-04-27T20:39:03.3972189Z {"reason":"build-finished","success":true}
2020-04-27T20:39:03.4163806Z + python3 ../x.py test --stage 0 src/tools/compiletest
2020-04-27T20:39:03.7196775Z     Finished dev [unoptimized] target(s) in 0.21s
2020-04-27T20:39:05.1183370Z    Compiling log v0.4.8
2020-04-27T20:39:05.1183953Z    Compiling memchr v2.3.2
---
2020-04-27T20:39:54.4562067Z    Compiling semver v0.9.0
2020-04-27T20:39:56.0669574Z    Compiling cargo_metadata v0.9.1
2020-04-27T20:39:59.6585592Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-27T20:40:10.5194751Z     Finished release [optimized] target(s) in 24.60s
2020-04-27T20:40:10.5200739Z {"reason":"build-finished","success":true}
2020-04-27T20:40:17.7457820Z * 596 error codes
2020-04-27T20:40:17.7458627Z * highest error code: E0753
2020-04-27T20:40:18.1263579Z * 283 features
2020-04-27T20:40:20.4393692Z Checking which error codes lack tests...
---
2020-04-27T20:40:22.6273224Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-27T20:40:22.6273656Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-27T20:40:22.6274232Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-27T20:40:22.6279602Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-27T20:40:23.8360769Z Diff in /checkout/src/libfmt_macros/lib.rs at line 322:
2020-04-27T20:40:23.8361188Z                  let pos = self.to_span_index(pos);
2020-04-27T20:40:23.8362254Z                  let description = format!("expected `'}}'`, found `{:?}`", maybe);
2020-04-27T20:40:23.8362602Z                  let label = "expected `}`".to_owned();
2020-04-27T20:40:23.8363058Z -                let note = Some(
2020-04-27T20:40:23.8363603Z -                        "if you intended to print `{`, you can escape it using `{{`".to_owned(),
2020-04-27T20:40:23.8364397Z -                let secondary_label =
2020-04-27T20:40:23.8364830Z -                    self.last_opening_brace
2020-04-27T20:40:23.8364830Z -                    self.last_opening_brace
2020-04-27T20:40:23.8365343Z -                        .map(|sp| ("because of this opening brace".to_owned(), sp));
2020-04-27T20:40:23.8365619Z +                let note =
2020-04-27T20:40:23.8365930Z +                    Some("if you intended to print `{`, you can escape it using `{{`".to_owned());
2020-04-27T20:40:23.8366250Z +                let secondary_label = self
2020-04-27T20:40:23.8366482Z +                    .last_opening_brace
2020-04-27T20:40:23.8366786Z +                    .map(|sp| ("because of this opening brace".to_owned(), sp));
2020-04-27T20:40:23.8367080Z                  self.errors.push(ParseError {
2020-04-27T20:40:23.8367519Z                      note,
2020-04-27T20:40:23.8367519Z                      note,
2020-04-27T20:40:23.8376205Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libfmt_macros/lib.rs"` failed.
2020-04-27T20:40:23.8377550Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-27T20:40:23.8391266Z Build completed unsuccessfully in 0:00:39
2020-04-27T20:40:23.8500767Z == clock drift check ==
2020-04-27T20:40:23.8521923Z   local time: Mon Apr 27 20:40:23 UTC 2020
2020-04-27T20:40:23.8521923Z   local time: Mon Apr 27 20:40:23 UTC 2020
2020-04-27T20:40:24.0118928Z   network time: Mon, 27 Apr 2020 20:40:24 GMT
2020-04-27T20:40:25.4988647Z 
2020-04-27T20:40:25.4988647Z 
2020-04-27T20:40:25.5063500Z ##[error]Bash exited with code '1'.
2020-04-27T20:40:25.5079556Z ##[section]Finishing: Run build
2020-04-27T20:40:25.5129830Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T20:40:25.5135369Z Task         : Get sources
2020-04-27T20:40:25.5135693Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T20:40:25.5135976Z Version      : 1.0.0
2020-04-27T20:40:25.5136181Z Author       : Microsoft
2020-04-27T20:40:25.5136181Z Author       : Microsoft
2020-04-27T20:40:25.5136519Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T20:40:25.5137353Z ==============================================================================
2020-04-27T20:40:25.8575807Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T20:40:25.8621475Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T20:40:25.8708457Z Cleaning up task key
2020-04-27T20:40:25.8709691Z Start cleaning up orphan processes.
2020-04-27T20:40:25.8911788Z Terminate orphan process: pid (3651) (python)
2020-04-27T20:40:25.9066845Z ##[section]Finishing: Finalize Job
