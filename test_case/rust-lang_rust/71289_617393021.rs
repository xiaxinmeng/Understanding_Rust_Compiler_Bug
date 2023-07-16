plain
2020-04-21T18:31:19.5464867Z ========================== Starting Command Output ===========================
2020-04-21T18:31:19.5486278Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f82118e6-146d-4e97-b131-21053afe8613.sh
2020-04-21T18:31:19.5721522Z 
2020-04-21T18:31:19.5830794Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-21T18:31:19.5850487Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-21T18:31:19.5854026Z Task         : Get sources
2020-04-21T18:31:19.5854303Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T18:31:19.5854644Z Version      : 1.0.0
2020-04-21T18:31:19.5854830Z Author       : Microsoft
---
2020-04-21T18:31:20.7509723Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-21T18:31:20.7518358Z ##[command]git config gc.auto 0
2020-04-21T18:31:20.7525019Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-21T18:31:20.7531194Z ##[command]git config --get-all http.proxy
2020-04-21T18:31:20.7542286Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71289/merge:refs/remotes/pull/71289/merge
---
2020-04-21T18:34:32.9593812Z  ---> 78ad2f4d4aca
2020-04-21T18:34:32.9594050Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-21T18:34:32.9599067Z  ---> Using cache
2020-04-21T18:34:32.9600359Z  ---> 4d2dc61c4d00
2020-04-21T18:34:32.9601667Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-21T18:34:32.9612077Z  ---> 776b6266a8b7
2020-04-21T18:34:32.9645421Z Successfully built 776b6266a8b7
2020-04-21T18:34:32.9685194Z Successfully tagged rust-ci:latest
2020-04-21T18:34:33.0894924Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-21T18:34:33.0894924Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-21T18:34:33.0919010Z Looks like docker image is the same as before, not uploading
2020-04-21T18:34:38.7261485Z [CI_JOB_NAME=mingw-check]
2020-04-21T18:34:38.7601130Z [CI_JOB_NAME=mingw-check]
2020-04-21T18:34:38.7627688Z == clock drift check ==
2020-04-21T18:34:38.7637431Z   local time: Tue Apr 21 18:34:38 UTC 2020
2020-04-21T18:34:38.8355790Z   network time: Tue, 21 Apr 2020 18:34:38 GMT
2020-04-21T18:34:38.8381366Z Starting sccache server...
2020-04-21T18:34:38.9602769Z configure: processing command line
2020-04-21T18:34:38.9603539Z configure: 
2020-04-21T18:34:38.9604744Z configure: rust.parallel-compiler := True
---
2020-04-21T18:38:46.2195284Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-21T18:38:51.3180743Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-21T18:38:52.6799003Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T18:38:52.6800304Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T18:38:52.8975584Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T18:38:53.8225859Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T18:38:53.8620659Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-21T18:38:55.6543618Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T18:38:56.1908523Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-21T18:41:02.6634228Z configure: build.cargo-native-static := True
2020-04-21T18:41:02.6634683Z configure: rust.codegen-units-std := 1
2020-04-21T18:41:02.6635299Z configure: rust.verify-llvm-ir  := True
2020-04-21T18:41:02.6635958Z configure: rust.debug-assertions := True
2020-04-21T18:41:02.6636534Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-21T18:41:02.6637072Z configure: writing `config.toml` in current directory
2020-04-21T18:41:02.6637298Z configure: 
2020-04-21T18:41:02.6637687Z configure: run `python /checkout/x.py --help`
2020-04-21T18:41:02.6638107Z configure: 
---
2020-04-21T18:42:46.6056141Z Hugepagesize:       2048 kB
2020-04-21T18:42:46.6056367Z DirectMap4k:      112576 kB
2020-04-21T18:42:46.6056574Z DirectMap2M:     3033088 kB
2020-04-21T18:42:46.6056782Z DirectMap1G:     6291456 kB
2020-04-21T18:42:46.6075478Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-21T18:42:48.1225108Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-21T18:42:48.1225108Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-21T18:42:48.1231756Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-21T18:42:48.3994996Z    Compiling unicode-xid v0.2.0
2020-04-21T18:42:48.5424144Z    Compiling syn v1.0.11
2020-04-21T18:42:49.4789622Z    Compiling linked-hash-map v0.5.2
2020-04-21T18:42:49.4935127Z    Compiling lazy_static v1.4.0
2020-04-21T18:42:49.4935127Z    Compiling lazy_static v1.4.0
2020-04-21T18:42:49.7331264Z    Compiling yaml-rust v0.4.3
2020-04-21T18:42:54.4650472Z    Compiling quote v1.0.2
2020-04-21T18:43:10.8055899Z    Compiling thiserror-impl v1.0.5
2020-04-21T18:43:16.0717502Z    Compiling thiserror v1.0.5
2020-04-21T18:43:16.1360986Z    Compiling yaml-merge-keys v0.4.0
2020-04-21T18:43:17.4516452Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-21T18:43:19.2877463Z Build completed successfully in 0:00:32
2020-04-21T18:43:19.2982964Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-21T18:43:19.6118901Z     Finished dev [unoptimized] target(s) in 0.21s
2020-04-21T18:43:20.8734002Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-21T18:45:42.2266443Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T18:45:42.3185616Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T18:45:42.5413693Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T18:45:42.7034224Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T18:45:43.2311733Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T18:45:45.7864971Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T18:45:46.3254269Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T18:45:48.8096053Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T18:45:49.3185602Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T18:49:54.7124891Z    Compiling cargo_metadata v0.9.1
2020-04-21T18:49:59.7232111Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-21T18:50:11.3003168Z     Finished release [optimized] target(s) in 27.62s
2020-04-21T18:50:11.3108181Z tidy check
2020-04-21T18:50:11.9933228Z tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links.rs:486: TODO is deprecated; use FIXME
2020-04-21T18:50:11.9933816Z tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links.rs:492: TODO is deprecated; use FIXME
2020-04-21T18:50:11.9977168Z tidy error: /checkout/src/librustdoc/clean/types.rs:420: TODO is deprecated; use FIXME
2020-04-21T18:50:12.6098674Z tidy error: /checkout/src/test/rustdoc/intra-link-self.rs:3: TODO is deprecated; use FIXME
2020-04-21T18:50:12.6099472Z tidy error: /checkout/src/test/rustdoc/intra-link-self.rs:33: TODO is deprecated; use FIXME
2020-04-21T18:50:12.6100140Z tidy error: /checkout/src/test/rustdoc/intra-link-self.rs:44: TODO is deprecated; use FIXME
2020-04-21T18:50:15.1094364Z some tidy checks failed
2020-04-21T18:50:15.1098927Z Found 491 error codes
2020-04-21T18:50:15.1099467Z Found 0 error codes with no tests
2020-04-21T18:50:15.1099787Z Done!
2020-04-21T18:50:15.1099787Z Done!
2020-04-21T18:50:15.1100108Z 
2020-04-21T18:50:15.1100343Z 
2020-04-21T18:50:15.1102004Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-21T18:50:15.1103049Z 
2020-04-21T18:50:15.1103257Z 
2020-04-21T18:50:15.1105952Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-21T18:50:15.1106488Z Build completed unsuccessfully in 0:00:32
2020-04-21T18:50:15.1106488Z Build completed unsuccessfully in 0:00:32
2020-04-21T18:50:15.1216337Z == clock drift check ==
2020-04-21T18:50:15.1246088Z   local time: Tue Apr 21 18:50:15 UTC 2020
2020-04-21T18:50:15.3881210Z   network time: Tue, 21 Apr 2020 18:50:15 GMT
2020-04-21T18:50:16.9135013Z 
2020-04-21T18:50:16.9135013Z 
2020-04-21T18:50:16.9262779Z ##[error]Bash exited with code '1'.
2020-04-21T18:50:16.9282725Z ##[section]Finishing: Run build
2020-04-21T18:50:16.9338278Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-21T18:50:16.9343528Z Task         : Get sources
2020-04-21T18:50:16.9343870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T18:50:16.9344172Z Version      : 1.0.0
2020-04-21T18:50:16.9344382Z Author       : Microsoft
2020-04-21T18:50:16.9344382Z Author       : Microsoft
2020-04-21T18:50:16.9344732Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T18:50:16.9345116Z ==============================================================================
2020-04-21T18:50:17.2971704Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T18:50:17.3020386Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-21T18:50:17.3144884Z Cleaning up task key
2020-04-21T18:50:17.3146406Z Start cleaning up orphan processes.
2020-04-21T18:50:17.3487355Z Terminate orphan process: pid (4971) (python)
2020-04-21T18:50:17.3531818Z ##[section]Finishing: Finalize Job
