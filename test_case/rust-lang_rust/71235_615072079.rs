plain
2020-04-17T04:34:35.7887071Z ========================== Starting Command Output ===========================
2020-04-17T04:34:35.7891549Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b943300d-fb06-46da-a808-639997361148.sh
2020-04-17T04:34:35.7891949Z 
2020-04-17T04:34:35.7896370Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T04:34:35.7914128Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71235/merge to s
2020-04-17T04:34:35.7917425Z Task         : Get sources
2020-04-17T04:34:35.7917800Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T04:34:35.7918042Z Version      : 1.0.0
2020-04-17T04:34:35.7918209Z Author       : Microsoft
---
2020-04-17T04:34:36.9839417Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T04:34:36.9845953Z ##[command]git config gc.auto 0
2020-04-17T04:34:36.9849594Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T04:34:36.9852884Z ##[command]git config --get-all http.proxy
2020-04-17T04:34:36.9859923Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71235/merge:refs/remotes/pull/71235/merge
---
2020-04-17T04:37:08.7843587Z  ---> 78ad2f4d4aca
2020-04-17T04:37:08.7844065Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-17T04:37:08.7849529Z  ---> Using cache
2020-04-17T04:37:08.7849923Z  ---> 4d2dc61c4d00
2020-04-17T04:37:08.7851136Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-17T04:37:08.7855483Z  ---> 776b6266a8b7
2020-04-17T04:37:08.7958423Z Successfully built 776b6266a8b7
2020-04-17T04:37:08.7971468Z Successfully tagged rust-ci:latest
2020-04-17T04:37:08.8257241Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-17T04:37:08.8257241Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-17T04:37:08.8271500Z Looks like docker image is the same as before, not uploading
2020-04-17T04:37:14.2130862Z [CI_JOB_NAME=mingw-check]
2020-04-17T04:37:14.2557940Z [CI_JOB_NAME=mingw-check]
2020-04-17T04:37:14.2598728Z == clock drift check ==
2020-04-17T04:37:14.2624480Z   local time: Fri Apr 17 04:37:14 UTC 2020
2020-04-17T04:37:14.6337132Z   network time: Fri, 17 Apr 2020 04:37:14 GMT
2020-04-17T04:37:14.6338046Z Starting sccache server...
2020-04-17T04:37:14.7369895Z configure: processing command line
2020-04-17T04:37:14.7370142Z configure: 
2020-04-17T04:37:14.7371002Z configure: rust.parallel-compiler := True
---
2020-04-17T04:41:04.4789726Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-17T04:41:09.2719874Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-17T04:41:10.5378372Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T04:41:10.5465130Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T04:41:10.7486081Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T04:41:11.6198663Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T04:41:11.6423224Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-17T04:41:13.2555142Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T04:41:13.7687951Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-17T04:43:14.0587602Z configure: rust.dist-src        := False
2020-04-17T04:43:14.0587995Z configure: llvm.assertions      := True
2020-04-17T04:43:14.0588535Z configure: build.locked-deps    := True
2020-04-17T04:43:14.0589157Z configure: build.cargo-native-static := True
2020-04-17T04:43:14.0589836Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-17T04:43:14.0590598Z configure: writing `config.toml` in current directory
2020-04-17T04:43:14.0590911Z configure: 
2020-04-17T04:43:14.0591425Z configure: run `python /checkout/x.py --help`
2020-04-17T04:43:14.0591753Z configure: 
---
2020-04-17T04:44:48.7277651Z Hugepagesize:       2048 kB
2020-04-17T04:44:48.7277856Z DirectMap4k:      106432 kB
2020-04-17T04:44:48.7278075Z DirectMap2M:     4087808 kB
2020-04-17T04:44:48.7278277Z DirectMap1G:     5242880 kB
2020-04-17T04:44:48.7344065Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-17T04:44:50.1465164Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-17T04:44:50.1465164Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-17T04:44:50.1474261Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-17T04:44:50.3887550Z    Compiling unicode-xid v0.2.0
2020-04-17T04:44:50.5154713Z    Compiling syn v1.0.11
2020-04-17T04:44:51.4042127Z    Compiling linked-hash-map v0.5.2
2020-04-17T04:44:51.4050041Z    Compiling lazy_static v1.4.0
2020-04-17T04:44:51.4050041Z    Compiling lazy_static v1.4.0
2020-04-17T04:44:51.6379890Z    Compiling yaml-rust v0.4.3
2020-04-17T04:44:56.1205367Z    Compiling quote v1.0.2
2020-04-17T04:45:11.6645013Z    Compiling thiserror-impl v1.0.5
2020-04-17T04:45:16.6251554Z    Compiling thiserror v1.0.5
2020-04-17T04:45:16.6897243Z    Compiling yaml-merge-keys v0.4.0
2020-04-17T04:45:18.2741853Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-17T04:45:20.0030550Z Build completed successfully in 0:00:31
2020-04-17T04:45:20.0126746Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-17T04:45:20.2899096Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-17T04:45:21.4105315Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-17T04:47:33.7131981Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T04:47:33.7979903Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T04:47:33.9995393Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T04:47:34.1514909Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T04:47:34.6986258Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T04:47:37.0505833Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T04:47:37.6351495Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T04:47:39.8152583Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T04:47:40.2804751Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T04:51:49.7676099Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-17T04:51:49.7685415Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-17T04:51:49.7686704Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-17T04:51:49.7691673Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-17T04:51:51.7907992Z Diff in /checkout/src/librustc_resolve/late/diagnostics.rs at line 1035:
2020-04-17T04:51:51.7926562Z          lifetime_names: &FxHashSet<ast::Ident>,
2020-04-17T04:51:51.7931198Z          params: &[ElisionFailureInfo],
2020-04-17T04:51:51.7932323Z -        err.span_label(span, &format!(
2020-04-17T04:51:51.7932323Z -        err.span_label(span, &format!(
2020-04-17T04:51:51.7932797Z -            "expected {} lifetime parameter{}",
2020-04-17T04:51:51.7933905Z -                "named".to_string()
2020-04-17T04:51:51.7934276Z -            } else {
2020-04-17T04:51:51.7934633Z -                count.to_string()
2020-04-17T04:51:51.7934956Z -            },
2020-04-17T04:51:51.7934956Z -            },
2020-04-17T04:51:51.7935405Z -            pluralize!(count)
2020-04-17T04:51:51.7935788Z -        ));
2020-04-17T04:51:51.7935959Z +        err.span_label(
2020-04-17T04:51:51.7936134Z +            span,
2020-04-17T04:51:51.7936322Z +            &format!(
2020-04-17T04:51:51.7936545Z +                "expected {} lifetime parameter{}",
2020-04-17T04:51:51.7936860Z +                if count == 1 { "named".to_string() } else { count.to_string() },
2020-04-17T04:51:51.7941883Z +                pluralize!(count)
2020-04-17T04:51:51.7952309Z +        );
2020-04-17T04:51:51.7956371Z  
2020-04-17T04:51:51.7956371Z  
2020-04-17T04:51:51.7956674Z          let snippet = self.tcx.sess.source_map().span_to_snippet(span).ok();
2020-04-17T04:51:51.7957450Z          let suggest_existing = |err: &mut DiagnosticBuilder<'_>, sugg| {
2020-04-17T04:51:51.7957857Z Diff in /checkout/src/librustc_resolve/late/diagnostics.rs at line 1089:
2020-04-17T04:51:51.7958303Z                  });
2020-04-17T04:51:51.7958525Z                  for param in params {
2020-04-17T04:51:51.7958525Z                  for param in params {
2020-04-17T04:51:51.7959196Z -                    if let Ok(snippet) = self.tcx.sess.source_map().span_to_snippet(param.span)
2020-04-17T04:51:51.7959622Z -                    {
2020-04-17T04:51:51.7959928Z +                    if let Ok(snippet) = self.tcx.sess.source_map().span_to_snippet(param.span) {
2020-04-17T04:51:51.7960503Z                          if snippet.starts_with('&') && !snippet.starts_with("&'") {
2020-04-17T04:51:51.7960809Z                              introduce_suggestion
2020-04-17T04:51:51.7961336Z                                  .push((param.span, format!("&'a {}", &snippet[1..])));
2020-04-17T04:51:51.7961696Z Diff in /checkout/src/librustc_resolve/late/diagnostics.rs at line 1101:
2020-04-17T04:51:51.7962141Z                  }
2020-04-17T04:51:51.7962141Z                  }
2020-04-17T04:51:51.7962376Z                  introduce_suggestion.push((span, sugg.to_string()));
2020-04-17T04:51:51.7962876Z -                err.multipart_suggestion(
2020-04-17T04:51:51.7963253Z -                    &msg,
2020-04-17T04:51:51.7963622Z -                    introduce_suggestion,
2020-04-17T04:51:51.7964414Z -                );
2020-04-17T04:51:51.7964414Z -                );
2020-04-17T04:51:51.7964756Z +                err.multipart_suggestion(&msg, introduce_suggestion, Applicability::MaybeIncorrect);
2020-04-17T04:51:51.7965098Z                  if should_break {
2020-04-17T04:51:51.7965482Z                  }
2020-04-17T04:51:51.7965482Z                  }
2020-04-17T04:51:51.7972928Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_resolve/late/diagnostics.rs"` failed.
2020-04-17T04:51:51.7973892Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-17T04:51:51.7985899Z Build completed unsuccessfully in 0:00:35
2020-04-17T04:51:51.8089325Z == clock drift check ==
2020-04-17T04:51:51.8089325Z == clock drift check ==
2020-04-17T04:51:51.8108988Z   local time: Fri Apr 17 04:51:51 UTC 2020
2020-04-17T04:51:51.8210539Z   network time: Diff in /checkout/src/librustc_resolve/late/lifetimes.rs at line 2387:
2020-04-17T04:51:51.8210970Z          let mut err = self.report_missing_lifetime_specifiers(span, lifetime_refs.len());
2020-04-17T04:51:51.8211410Z          if let Some(params) = error {
2020-04-17T04:51:51.8211410Z          if let Some(params) = error {
2020-04-17T04:51:51.8212428Z -            if self.report_elision_failure(&mut err, params) && lifetime_names.is_empty()
2020-04-17T04:51:51.8212998Z -            {
2020-04-17T04:51:51.8213500Z +            if self.report_elision_failure(&mut err, params) && lifetime_names.is_empty() {
2020-04-17T04:51:51.8214141Z                  lifetime_names.insert(ast::Ident::from_str("'static"));
2020-04-17T04:51:51.8214691Z          }
2020-04-17T04:51:51.8214691Z          }
2020-04-17T04:51:51.9089746Z Fri, 17 Apr 2020 04:51:51 GMT
2020-04-17T04:51:53.3907721Z 
2020-04-17T04:51:53.3907721Z 
2020-04-17T04:51:53.3979714Z ##[error]Bash exited with code '1'.
2020-04-17T04:51:53.3994026Z ##[section]Finishing: Run build
2020-04-17T04:51:53.4064346Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71235/merge to s
2020-04-17T04:51:53.4069524Z Task         : Get sources
2020-04-17T04:51:53.4069815Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T04:51:53.4070104Z Version      : 1.0.0
2020-04-17T04:51:53.4070311Z Author       : Microsoft
2020-04-17T04:51:53.4070311Z Author       : Microsoft
2020-04-17T04:51:53.4070620Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T04:51:53.4070979Z ==============================================================================
2020-04-17T04:51:53.7584937Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T04:51:53.7658685Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71235/merge to s
2020-04-17T04:51:53.7762849Z Cleaning up task key
2020-04-17T04:51:53.7764227Z Start cleaning up orphan processes.
2020-04-17T04:51:53.7966192Z Terminate orphan process: pid (3520) (python)
2020-04-17T04:51:53.8214600Z ##[section]Finishing: Finalize Job
