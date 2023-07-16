plain
2020-04-27T07:38:49.7666488Z ========================== Starting Command Output ===========================
2020-04-27T07:38:49.7669269Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a0b7b02e-9124-43a1-8431-8eb1c54abbab.sh
2020-04-27T07:38:49.7669553Z 
2020-04-27T07:38:49.7674265Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T07:38:49.7695814Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-27T07:38:49.7699397Z Task         : Get sources
2020-04-27T07:38:49.7699743Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T07:38:49.7700064Z Version      : 1.0.0
2020-04-27T07:38:49.7700282Z Author       : Microsoft
---
2020-04-27T07:38:50.7805583Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T07:38:50.7810458Z ##[command]git config gc.auto 0
2020-04-27T07:38:50.7813974Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T07:38:50.7817190Z ##[command]git config --get-all http.proxy
2020-04-27T07:38:50.7823051Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71199/merge:refs/remotes/pull/71199/merge
---
2020-04-27T07:41:04.7580840Z  ---> f7353ccad5b1
2020-04-27T07:41:04.7581242Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-27T07:41:04.7586268Z  ---> Using cache
2020-04-27T07:41:04.7586947Z  ---> ed38efbaa060
2020-04-27T07:41:04.7588275Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-27T07:41:04.7596891Z  ---> c5008ef7ae8e
2020-04-27T07:41:04.7659692Z Successfully built c5008ef7ae8e
2020-04-27T07:41:04.7698118Z Successfully tagged rust-ci:latest
2020-04-27T07:41:04.7972505Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T07:41:04.7972505Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T07:41:04.7987263Z Looks like docker image is the same as before, not uploading
2020-04-27T07:41:12.7582922Z [CI_JOB_NAME=mingw-check]
2020-04-27T07:41:12.7791690Z [CI_JOB_NAME=mingw-check]
2020-04-27T07:41:12.7815703Z == clock drift check ==
2020-04-27T07:41:12.7825294Z   local time: Mon Apr 27 07:41:12 UTC 2020
2020-04-27T07:41:13.0656145Z   network time: Mon, 27 Apr 2020 07:41:13 GMT
2020-04-27T07:41:13.0677250Z Starting sccache server...
2020-04-27T07:41:13.1734287Z configure: processing command line
2020-04-27T07:41:13.1734929Z configure: 
2020-04-27T07:41:13.1735892Z configure: rust.parallel-compiler := True
---
2020-04-27T07:43:13.8200824Z     Checking unicode-width v0.1.6
2020-04-27T07:43:13.8931038Z     Checking getopts v0.2.21
2020-04-27T07:43:15.8222361Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-27T07:43:16.4462990Z     Finished release [optimized] target(s) in 27.64s
2020-04-27T07:43:16.4469729Z {"reason":"build-finished","success":true}
2020-04-27T07:43:16.9900831Z     Checking cfg-if v0.1.10
2020-04-27T07:43:16.9912300Z    Compiling libc v0.2.69
2020-04-27T07:43:17.0301061Z    Compiling semver-parser v0.7.0
2020-04-27T07:43:17.8055454Z     Checking lazy_static v1.4.0
---
2020-04-27T07:44:43.9075922Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T07:44:44.0949719Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T07:44:44.2764698Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T07:44:44.2779081Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T07:44:44.8371273Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T07:44:46.9293048Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T07:44:47.3781668Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T07:44:49.1910859Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T07:44:49.5909006Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T07:45:19.6868441Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-04-27T07:45:25.1203350Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T07:45:25.2740761Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T07:45:32.6145062Z     Checking rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T07:45:32.6956675Z  {"reason":"build-finished","success":true}
2020-04-27T07:45:32.7446193Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T07:45:32.9640501Z     Checking cfg-if v0.1.10
2020-04-27T07:45:32.9644884Z    Compiling libc v0.2.69
2020-04-27T07:45:32.9972506Z    Compiling semver-parser v0.7.0
---
2020-04-27T07:45:56.6990042Z    Compiling serde_derive v1.0.81
2020-04-27T07:46:19.7534612Z     Checking serde_json v1.0.40
2020-04-27T07:46:20.8734888Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-04-27T07:46:25.0724229Z     Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2020-04-27T07:46:25.1696988Z {"reason":"build-finished","success":true}
2020-04-27T07:46:25.1852726Z Build completed successfully in 0:05:11
2020-04-27T07:46:25.4290297Z configure: processing command line
2020-04-27T07:46:25.4290605Z configure: 
2020-04-27T07:46:25.4291508Z configure: dist.missing-tools   := True
---
2020-04-27T07:46:25.4297081Z configure: build.cargo-native-static := True
2020-04-27T07:46:25.4297555Z configure: rust.channel         := nightly
2020-04-27T07:46:25.4298014Z configure: llvm.ccache          := sccache
2020-04-27T07:46:25.4298634Z configure: build.locked-deps    := True
2020-04-27T07:46:25.4299409Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-27T07:46:25.4300259Z configure: writing `config.toml` in current directory
2020-04-27T07:46:25.4300654Z configure: 
2020-04-27T07:46:25.4301222Z configure: run `python /checkout/x.py --help`
2020-04-27T07:46:25.4301613Z configure: 
---
2020-04-27T07:47:50.4741190Z Hugepagesize:       2048 kB
2020-04-27T07:47:50.4741391Z DirectMap4k:      149440 kB
2020-04-27T07:47:50.4741703Z DirectMap2M:     2996224 kB
2020-04-27T07:47:50.4741899Z DirectMap1G:     6291456 kB
2020-04-27T07:47:50.4805761Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-27T07:47:51.8573021Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-27T07:47:51.8573021Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-27T07:47:51.8580793Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-27T07:47:52.0909092Z    Compiling unicode-xid v0.2.0
2020-04-27T07:47:52.2087679Z    Compiling syn v1.0.11
2020-04-27T07:47:52.9622797Z    Compiling linked-hash-map v0.5.2
2020-04-27T07:47:53.0081368Z    Compiling lazy_static v1.4.0
2020-04-27T07:47:53.0081368Z    Compiling lazy_static v1.4.0
2020-04-27T07:47:53.1642507Z    Compiling yaml-rust v0.4.3
2020-04-27T07:47:57.1013120Z    Compiling quote v1.0.2
2020-04-27T07:48:09.5875280Z    Compiling thiserror-impl v1.0.5
2020-04-27T07:48:14.1331905Z    Compiling thiserror v1.0.5
2020-04-27T07:48:14.1882536Z    Compiling yaml-merge-keys v0.4.0
2020-04-27T07:48:15.2438094Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-27T07:48:16.7798408Z     Finished release [optimized] target(s) in 24.91s
2020-04-27T07:48:16.7799116Z {"reason":"build-finished","success":true}
2020-04-27T07:48:17.6203889Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-27T07:48:17.8902191Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-27T07:48:18.9510781Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
2020-04-27T07:48:18.9529469Z Checking std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-27T07:48:40.3980955Z     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
2020-04-27T07:48:40.6292630Z     Checking unicode-width v0.1.6
2020-04-27T07:48:40.7020399Z     Checking getopts v0.2.21
2020-04-27T07:48:42.5842086Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-27T07:48:43.1910777Z {"reason":"build-finished","success":true}
2020-04-27T07:48:43.2052198Z Checking compiler artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
2020-04-27T07:48:43.7517800Z     Checking cfg-if v0.1.10
2020-04-27T07:48:43.7518470Z    Compiling winapi-i686-pc-windows-gnu v0.4.0
2020-04-27T07:48:43.7867979Z    Compiling semver-parser v0.7.0
---
2020-04-27T07:50:13.0121476Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T07:50:13.1526094Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T07:50:13.3450459Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T07:50:13.4011735Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T07:50:13.9272376Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T07:50:15.9781709Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T07:50:16.4463121Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T07:50:18.3486313Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T07:50:18.7386735Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T07:50:53.8259694Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T07:50:55.5196032Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T07:50:58.2972831Z     Checking rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T07:50:58.3776768Z     Finished release [optimized] target(s) in 2m 15s
2020-04-27T07:50:58.3783682Z {"reason":"build-finished","success":true}
2020-04-27T07:50:58.6740530Z     Checking cfg-if v0.1.10
2020-04-27T07:50:58.6751355Z    Compiling semver-parser v0.7.0
2020-04-27T07:50:58.7105107Z    Compiling proc-macro2 v0.4.30
2020-04-27T07:50:59.5686807Z    Compiling unicode-xid v0.1.0
---
2020-04-27T07:51:47.6911620Z     Checking serde_json v1.0.40
2020-04-27T07:51:48.8103866Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-04-27T07:51:52.9626710Z     Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2020-04-27T07:51:53.0572333Z     Finished release [optimized] target(s) in 54.62s
2020-04-27T07:51:53.0573730Z {"reason":"build-finished","success":true}
2020-04-27T07:51:53.0859029Z + python3 ../x.py build --stage 0 src/tools/build-manifest
2020-04-27T07:51:53.3543313Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-27T07:51:54.4086913Z Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
2020-04-27T07:51:54.5845897Z    Compiling proc-macro2 v0.4.30
---
2020-04-27T07:52:32.5889288Z    Compiling toml v0.5.3
2020-04-27T07:52:33.1299498Z    Compiling serde_json v1.0.40
2020-04-27T07:52:39.4541743Z    Compiling build-manifest v0.1.0 (/checkout/src/tools/build-manifest)
2020-04-27T07:52:43.4190031Z     Finished release [optimized] target(s) in 49.01s
2020-04-27T07:52:43.4196892Z {"reason":"build-finished","success":true}
2020-04-27T07:52:43.4369909Z + python3 ../x.py test --stage 0 src/tools/compiletest
2020-04-27T07:52:43.7135026Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-27T07:52:44.9707378Z    Compiling log v0.4.8
2020-04-27T07:52:44.9707973Z    Compiling memchr v2.3.2
---
2020-04-27T07:53:28.9438907Z    Compiling semver v0.9.0
2020-04-27T07:53:30.3951814Z    Compiling cargo_metadata v0.9.1
2020-04-27T07:53:33.6528894Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-27T07:53:43.4536638Z     Finished release [optimized] target(s) in 22.03s
2020-04-27T07:53:43.4537208Z {"reason":"build-finished","success":true}
2020-04-27T07:53:50.7811973Z * 596 error codes
2020-04-27T07:53:50.7812781Z * highest error code: E0753
2020-04-27T07:53:51.1241199Z * 283 features
2020-04-27T07:53:53.7498175Z Checking which error codes lack tests...
---
2020-04-27T07:53:56.3182191Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-27T07:53:56.3187059Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-27T07:53:56.3190374Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-27T07:53:56.3193338Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-27T07:53:58.6750236Z Diff in /checkout/src/librustc_mir/transform/mod.rs at line 123:
2020-04-27T07:53:58.6751448Z  /// type `T`.
2020-04-27T07:53:58.6752948Z  pub fn default_name<T: ?Sized>() -> Cow<'static, str> {
2020-04-27T07:53:58.6753979Z      let name = ::std::any::type_name::<T>();
2020-04-27T07:53:58.6755472Z -    if let Some(tail) = name.rfind(':') {
2020-04-27T07:53:58.6756202Z -        Cow::from(&name[tail + 1..])
2020-04-27T07:53:58.6757092Z -        Cow::from(name)
2020-04-27T07:53:58.6757559Z -    }
2020-04-27T07:53:58.6757559Z -    }
2020-04-27T07:53:58.6758185Z +    if let Some(tail) = name.rfind(':') { Cow::from(&name[tail + 1..]) } else { Cow::from(name) }
2020-04-27T07:53:58.6759490Z  
2020-04-27T07:53:58.6760913Z  /// A streamlined trait that you can implement to create a pass; the
2020-04-27T07:53:58.6760913Z  /// A streamlined trait that you can implement to create a pass; the
2020-04-27T07:53:58.6762069Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/transform/mod.rs"` failed.
2020-04-27T07:53:58.6763054Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-27T07:53:58.6774490Z Build completed unsuccessfully in 0:00:38
2020-04-27T07:53:58.6870451Z == clock drift check ==
2020-04-27T07:53:58.6882705Z   local time: Mon Apr 27 07:53:58 UTC 2020
2020-04-27T07:53:58.6882705Z   local time: Mon Apr 27 07:53:58 UTC 2020
2020-04-27T07:53:58.9707402Z   network time: Mon, 27 Apr 2020 07:53:58 GMT
2020-04-27T07:54:00.4899387Z 
2020-04-27T07:54:00.4899387Z 
2020-04-27T07:54:00.4960823Z ##[error]Bash exited with code '1'.
2020-04-27T07:54:00.4972933Z ##[section]Finishing: Run build
2020-04-27T07:54:00.5028902Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-27T07:54:00.5033646Z Task         : Get sources
2020-04-27T07:54:00.5033979Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T07:54:00.5034266Z Version      : 1.0.0
2020-04-27T07:54:00.5034476Z Author       : Microsoft
2020-04-27T07:54:00.5034476Z Author       : Microsoft
2020-04-27T07:54:00.5034815Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T07:54:00.5035180Z ==============================================================================
2020-04-27T07:54:00.8063679Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T07:54:00.8106095Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-27T07:54:00.8190892Z Cleaning up task key
2020-04-27T07:54:00.8192055Z Start cleaning up orphan processes.
2020-04-27T07:54:00.8353686Z Terminate orphan process: pid (4122) (python)
2020-04-27T07:54:00.8583843Z ##[section]Finishing: Finalize Job
