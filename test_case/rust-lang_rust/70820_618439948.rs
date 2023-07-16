plain
2020-04-23T12:55:01.1598580Z ========================== Starting Command Output ===========================
2020-04-23T12:55:01.1603893Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/829418e2-32c3-4d00-9519-bb4d3b2ac9e4.sh
2020-04-23T12:55:01.1604344Z 
2020-04-23T12:55:01.1608905Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T12:55:01.1627117Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-23T12:55:01.1630290Z Task         : Get sources
2020-04-23T12:55:01.1630575Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T12:55:01.1630836Z Version      : 1.0.0
2020-04-23T12:55:01.1631017Z Author       : Microsoft
---
2020-04-23T12:55:02.3284546Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T12:55:02.3293531Z ##[command]git config gc.auto 0
2020-04-23T12:55:02.3300006Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T12:55:02.3305684Z ##[command]git config --get-all http.proxy
2020-04-23T12:55:02.3316127Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-23T12:57:36.4221200Z  ---> 78ad2f4d4aca
2020-04-23T12:57:36.4221493Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-23T12:57:36.4223253Z  ---> Using cache
2020-04-23T12:57:36.4223987Z  ---> 4d2dc61c4d00
2020-04-23T12:57:36.4227081Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-23T12:57:36.4238743Z  ---> 776b6266a8b7
2020-04-23T12:57:36.4277254Z Successfully built 776b6266a8b7
2020-04-23T12:57:36.4318511Z Successfully tagged rust-ci:latest
2020-04-23T12:57:36.4902417Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-23T12:57:36.4902417Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-23T12:57:36.4928511Z Looks like docker image is the same as before, not uploading
2020-04-23T12:57:44.5047025Z [CI_JOB_NAME=mingw-check]
2020-04-23T12:57:44.5382617Z [CI_JOB_NAME=mingw-check]
2020-04-23T12:57:44.5415612Z == clock drift check ==
2020-04-23T12:57:44.5424054Z   local time: Thu Apr 23 12:57:44 UTC 2020
2020-04-23T12:57:44.5912144Z   network time: Thu, 23 Apr 2020 12:57:44 GMT
2020-04-23T12:57:44.5937427Z Starting sccache server...
2020-04-23T12:57:44.7164590Z configure: processing command line
2020-04-23T12:57:44.7165382Z configure: 
2020-04-23T12:57:44.7166599Z configure: rust.parallel-compiler := True
---
2020-04-23T13:01:49.7349455Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T13:01:49.9880590Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T13:01:50.1926700Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T13:01:50.2066627Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T13:01:50.8859836Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T13:01:53.4777434Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T13:01:53.9974247Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T13:01:56.2244552Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T13:01:56.6909024Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T13:03:55.5633226Z configure: build.cargo-native-static := True
2020-04-23T13:03:55.5633549Z configure: llvm.assertions      := True
2020-04-23T13:03:55.5634021Z configure: build.submodules     := False
2020-04-23T13:03:55.5634318Z configure: llvm.ccache          := sccache
2020-04-23T13:03:55.5635000Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-23T13:03:55.5635558Z configure: writing `config.toml` in current directory
2020-04-23T13:03:55.5635816Z configure: 
2020-04-23T13:03:55.5636247Z configure: run `python /checkout/x.py --help`
2020-04-23T13:03:55.5636479Z configure: 
---
2020-04-23T13:05:35.3095106Z Hugepagesize:       2048 kB
2020-04-23T13:05:35.3095334Z DirectMap4k:      137152 kB
2020-04-23T13:05:35.3095576Z DirectMap2M:     5105664 kB
2020-04-23T13:05:35.3095799Z DirectMap1G:     4194304 kB
2020-04-23T13:05:35.3122028Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-23T13:05:36.7860786Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-23T13:05:36.7860786Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-23T13:05:36.7867941Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-23T13:05:37.0422761Z    Compiling unicode-xid v0.2.0
2020-04-23T13:05:37.1737251Z    Compiling syn v1.0.11
2020-04-23T13:05:38.1071409Z    Compiling linked-hash-map v0.5.2
2020-04-23T13:05:38.1361133Z    Compiling lazy_static v1.4.0
2020-04-23T13:05:38.1361133Z    Compiling lazy_static v1.4.0
2020-04-23T13:05:38.3576180Z    Compiling yaml-rust v0.4.3
2020-04-23T13:05:42.9727248Z    Compiling quote v1.0.2
2020-04-23T13:05:59.1251784Z    Compiling thiserror-impl v1.0.5
2020-04-23T13:06:04.3018072Z    Compiling thiserror v1.0.5
2020-04-23T13:06:04.3643288Z    Compiling yaml-merge-keys v0.4.0
2020-04-23T13:06:05.6861325Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-23T13:06:07.5157487Z Build completed successfully in 0:00:32
2020-04-23T13:06:07.5246836Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-23T13:06:07.8061664Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-23T13:06:08.9804812Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-23T13:08:20.3319510Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-23T13:08:25.4550096Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-23T13:08:26.7986460Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T13:08:26.7987175Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T13:08:27.0078280Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T13:08:27.8050480Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T13:08:27.9592699Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-23T13:08:29.7018616Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T13:08:30.2455347Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-23T13:13:10.4358891Z + /scripts/validate-toolstate.sh
2020-04-23T13:13:10.4404011Z Cloning into 'rust-toolstate'...
2020-04-23T13:15:19.7032142Z fatal: unable to access 'https://github.com/rust-lang-nursery/rust-toolstate.git/': Failed to connect to github.com port 443: Connection timed out
2020-04-23T13:15:19.7044106Z == clock drift check ==
2020-04-23T13:15:19.7079855Z   local time: Thu Apr 23 13:15:19 UTC 2020
2020-04-23T13:15:19.8661348Z   network time: Thu, 23 Apr 2020 13:15:19 GMT
2020-04-23T13:15:20.1917731Z 
2020-04-23T13:15:20.1917731Z 
2020-04-23T13:15:20.2024512Z ##[error]Bash exited with code '128'.
2020-04-23T13:15:20.2039689Z ##[section]Finishing: Run build
2020-04-23T13:15:20.2093789Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-23T13:15:20.2099100Z Task         : Get sources
2020-04-23T13:15:20.2099465Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T13:15:20.2099802Z Version      : 1.0.0
2020-04-23T13:15:20.2100048Z Author       : Microsoft
2020-04-23T13:15:20.2100048Z Author       : Microsoft
2020-04-23T13:15:20.2100411Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T13:15:20.2101100Z ==============================================================================
2020-04-23T13:15:20.6115349Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T13:15:20.6164714Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-23T13:15:20.6274874Z Cleaning up task key
2020-04-23T13:15:20.6276225Z Start cleaning up orphan processes.
2020-04-23T13:15:20.6528986Z Terminate orphan process: pid (4067) (python)
2020-04-23T13:15:20.6713179Z ##[section]Finishing: Finalize Job
