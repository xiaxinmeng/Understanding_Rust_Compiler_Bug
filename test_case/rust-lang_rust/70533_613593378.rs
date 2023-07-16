plain
2020-04-14T16:09:58.7521845Z ========================== Starting Command Output ===========================
2020-04-14T16:09:58.7541545Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a4ba7a8d-efc0-4669-b63f-4e2178b390e0.sh
2020-04-14T16:09:58.7716774Z 
2020-04-14T16:09:58.7779112Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T16:09:58.7798364Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70533/merge to s
2020-04-14T16:09:58.7802974Z Task         : Get sources
2020-04-14T16:09:58.7803251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T16:09:58.7803574Z Version      : 1.0.0
2020-04-14T16:09:58.7803758Z Author       : Microsoft
---
2020-04-14T16:09:59.5529682Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T16:09:59.5589750Z ##[command]git config gc.auto 0
2020-04-14T16:09:59.5623493Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T16:09:59.5649828Z ##[command]git config --get-all http.proxy
2020-04-14T16:09:59.5755775Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70533/merge:refs/remotes/pull/70533/merge
---
2020-04-14T16:12:45.1933279Z  ---> 78ad2f4d4aca
2020-04-14T16:12:45.1933696Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-14T16:12:45.1934264Z  ---> Using cache
2020-04-14T16:12:45.1934800Z  ---> 4d2dc61c4d00
2020-04-14T16:12:45.1940902Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-14T16:12:45.1970706Z  ---> 776b6266a8b7
2020-04-14T16:12:45.1970951Z Successfully built 776b6266a8b7
2020-04-14T16:12:45.1971394Z Successfully tagged rust-ci:latest
2020-04-14T16:12:45.2592300Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T16:12:45.2592300Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T16:12:45.2605881Z Looks like docker image is the same as before, not uploading
2020-04-14T16:12:52.4729028Z [CI_JOB_NAME=mingw-check]
2020-04-14T16:12:52.4946617Z [CI_JOB_NAME=mingw-check]
2020-04-14T16:12:52.4975985Z == clock drift check ==
2020-04-14T16:12:52.4987683Z   local time: Tue Apr 14 16:12:52 UTC 2020
2020-04-14T16:12:52.7632116Z   network time: Tue, 14 Apr 2020 16:12:52 GMT
2020-04-14T16:12:52.7657064Z Starting sccache server...
2020-04-14T16:12:52.8857927Z configure: processing command line
2020-04-14T16:12:52.8858730Z configure: 
2020-04-14T16:12:52.8859931Z configure: rust.parallel-compiler := True
---
2020-04-14T16:16:51.0371661Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T16:16:51.1370214Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T16:16:51.3830349Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T16:16:51.4800793Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T16:16:52.0449884Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T16:16:54.5363843Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T16:16:55.0404285Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T16:16:57.3604150Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T16:16:57.8413802Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T16:18:50.3196092Z configure: rust.dist-src        := False
2020-04-14T16:18:50.3196491Z configure: rust.codegen-units-std := 1
2020-04-14T16:18:50.3196902Z configure: rust.verify-llvm-ir  := True
2020-04-14T16:18:50.3197294Z configure: rust.debug-assertions := True
2020-04-14T16:18:50.3197797Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-14T16:18:50.3198425Z configure: writing `config.toml` in current directory
2020-04-14T16:18:50.3198637Z configure: 
2020-04-14T16:18:50.3199041Z configure: run `python /checkout/x.py --help`
2020-04-14T16:18:50.3199233Z configure: 
---
2020-04-14T16:20:19.0670898Z Hugepagesize:       2048 kB
2020-04-14T16:20:19.0671105Z DirectMap4k:      118720 kB
2020-04-14T16:20:19.0671290Z DirectMap2M:     4075520 kB
2020-04-14T16:20:19.0671474Z DirectMap1G:     5242880 kB
2020-04-14T16:20:19.0672101Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-14T16:20:20.3985381Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-14T16:20:20.3985381Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-14T16:20:20.3995246Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-14T16:20:20.6243332Z    Compiling unicode-xid v0.2.0
2020-04-14T16:20:20.7582902Z    Compiling syn v1.0.11
2020-04-14T16:20:21.5705782Z    Compiling linked-hash-map v0.5.2
2020-04-14T16:20:21.6158790Z    Compiling lazy_static v1.4.0
2020-04-14T16:20:21.6158790Z    Compiling lazy_static v1.4.0
2020-04-14T16:20:21.8268926Z    Compiling yaml-rust v0.4.3
2020-04-14T16:20:26.2211320Z    Compiling quote v1.0.2
2020-04-14T16:20:40.7126972Z    Compiling thiserror-impl v1.0.5
2020-04-14T16:20:45.6208262Z    Compiling thiserror v1.0.5
2020-04-14T16:20:45.6764628Z    Compiling yaml-merge-keys v0.4.0
2020-04-14T16:20:46.9460957Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-14T16:20:49.3846035Z Build completed successfully in 0:00:30
2020-04-14T16:20:49.3945429Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-14T16:20:49.6913583Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-14T16:20:50.8332284Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-14T16:23:01.9728780Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T16:23:01.9932929Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T16:23:02.2013553Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T16:23:02.4127508Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T16:23:02.8455984Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T16:23:05.2426897Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T16:23:05.7546880Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T16:23:07.9381446Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T16:23:08.3926914Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T16:27:28.5314453Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-14T16:27:28.5317848Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-14T16:27:29.6339082Z Diff in /checkout/src/bootstrap/test.rs at line 744:
2020-04-14T16:27:29.6339465Z              );
2020-04-14T16:27:29.6339753Z              // Second step: install npm dependencies.
2020-04-14T16:27:29.6340052Z              let mut cmd = Command::new("npm");
2020-04-14T16:27:29.6340958Z -            cmd.arg("install")
2020-04-14T16:27:29.6341548Z -                .current_dir(builder.out.join("browser-UI-test").to_str().unwrap());
2020-04-14T16:27:29.6342184Z +            cmd.arg("install").current_dir(builder.out.join("browser-UI-test").to_str().unwrap());
2020-04-14T16:27:29.6342519Z              try_run(builder, &mut cmd);
2020-04-14T16:27:29.6343013Z              // Third step: building documentation with lastest rustdoc version.
2020-04-14T16:27:29.6343369Z Diff in /checkout/src/bootstrap/test.rs at line 767:
2020-04-14T16:27:29.6343621Z              command
2020-04-14T16:27:29.6343621Z              command
2020-04-14T16:27:29.6344049Z                  .arg("../browser-UI-test/src/index.js")
2020-04-14T16:27:29.6344477Z                  .arg("--no-sandbox")
2020-04-14T16:27:29.6344938Z -                .arg("--test-folder").arg("ui-tests")
2020-04-14T16:27:29.6345415Z -                .arg("--failure-folder").arg("failures")
2020-04-14T16:27:29.6345955Z -                .arg("--variable").arg("DOC_PATH").arg("test-docs/doc/test_docs")
2020-04-14T16:27:29.6346445Z +                .arg("--test-folder")
2020-04-14T16:27:29.6348034Z +                .arg("ui-tests")
2020-04-14T16:27:29.6348451Z +                .arg("--failure-folder")
2020-04-14T16:27:29.6348705Z +                .arg("failures")
2020-04-14T16:27:29.6349093Z +                .arg("--variable")
2020-04-14T16:27:29.6349323Z +                .arg("DOC_PATH")
2020-04-14T16:27:29.6349763Z +                .arg("test-docs/doc/test_docs")
2020-04-14T16:27:29.6350175Z                  .arg("--show-text")
2020-04-14T16:27:29.6350579Z                  .arg("--generate-images")
2020-04-14T16:27:29.6351061Z                  .current_dir(builder.out.join("test-rust-docs-ui"));
2020-04-14T16:27:29.6369838Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/test.rs"` failed.
2020-04-14T16:27:29.6370812Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-14T16:27:29.6378525Z Build completed unsuccessfully in 0:00:41
2020-04-14T16:27:29.6483567Z == clock drift check ==
2020-04-14T16:27:29.6483567Z == clock drift check ==
2020-04-14T16:27:29.6493957Z   local time: Tue Apr 14 16:27:29 UTC 2020
2020-04-14T16:27:29.9476541Z   network time: Tue, 14 Apr 2020 16:27:29 GMT
2020-04-14T16:27:31.7565066Z 
2020-04-14T16:27:31.7565066Z 
2020-04-14T16:27:31.7667904Z ##[error]Bash exited with code '1'.
2020-04-14T16:27:31.7707757Z ##[section]Finishing: Run build
2020-04-14T16:27:31.7757192Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70533/merge to s
2020-04-14T16:27:31.7762263Z Task         : Get sources
2020-04-14T16:27:31.7762589Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T16:27:31.7762899Z Version      : 1.0.0
2020-04-14T16:27:31.7763106Z Author       : Microsoft
2020-04-14T16:27:31.7763106Z Author       : Microsoft
2020-04-14T16:27:31.7763443Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T16:27:31.7763838Z ==============================================================================
2020-04-14T16:27:32.1361456Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T16:27:32.1408903Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70533/merge to s
2020-04-14T16:27:32.1505214Z Cleaning up task key
2020-04-14T16:27:32.1506624Z Start cleaning up orphan processes.
2020-04-14T16:27:32.1692777Z Terminate orphan process: pid (4299) (python)
2020-04-14T16:27:32.2015128Z ##[section]Finishing: Finalize Job
