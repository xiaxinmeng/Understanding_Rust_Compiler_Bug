plain
2020-04-22T09:28:27.4778730Z ========================== Starting Command Output ===========================
2020-04-22T09:28:27.4781159Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bc73d881-976d-4062-9954-42a8b04f5444.sh
2020-04-22T09:28:27.4781405Z 
2020-04-22T09:28:27.4792614Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T09:28:27.4810693Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T09:28:27.4813788Z Task         : Get sources
2020-04-22T09:28:27.4814189Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T09:28:27.4814440Z Version      : 1.0.0
2020-04-22T09:28:27.4814608Z Author       : Microsoft
---
2020-04-22T09:28:28.7028445Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T09:28:28.7066178Z ##[command]git config gc.auto 0
2020-04-22T09:28:28.7102221Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T09:28:28.7124986Z ##[command]git config --get-all http.proxy
2020-04-22T09:28:28.7207404Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71420/merge:refs/remotes/pull/71420/merge
---
2020-04-22T09:30:46.9056551Z  ---> 78ad2f4d4aca
2020-04-22T09:30:46.9056808Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-22T09:30:46.9057172Z  ---> Using cache
2020-04-22T09:30:46.9057493Z  ---> 4d2dc61c4d00
2020-04-22T09:30:46.9058740Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-22T09:30:46.9059960Z  ---> 776b6266a8b7
2020-04-22T09:30:46.9060168Z Successfully built 776b6266a8b7
2020-04-22T09:30:46.9084367Z Successfully tagged rust-ci:latest
2020-04-22T09:30:46.9926549Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T09:30:46.9926549Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T09:30:46.9941434Z Looks like docker image is the same as before, not uploading
2020-04-22T09:30:55.0452748Z [CI_JOB_NAME=mingw-check]
2020-04-22T09:30:55.0682455Z [CI_JOB_NAME=mingw-check]
2020-04-22T09:30:55.0714665Z == clock drift check ==
2020-04-22T09:30:55.0725717Z   local time: Wed Apr 22 09:30:55 UTC 2020
2020-04-22T09:30:55.2348869Z   network time: Wed, 22 Apr 2020 09:30:55 GMT
2020-04-22T09:30:55.2375329Z Starting sccache server...
2020-04-22T09:30:55.3454762Z configure: processing command line
2020-04-22T09:30:55.3456163Z configure: 
2020-04-22T09:30:55.3457982Z configure: rust.parallel-compiler := True
---
2020-04-22T09:34:32.7603418Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T09:34:32.8911407Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T09:34:33.0769938Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T09:34:33.1492909Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T09:34:33.6789428Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T09:34:35.9251618Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T09:34:36.4003987Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T09:34:38.3880614Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T09:34:38.8267998Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T09:36:19.6786026Z configure: build.locked-deps    := True
2020-04-22T09:36:19.6786386Z configure: llvm.ccache          := sccache
2020-04-22T09:36:19.6786915Z configure: rust.codegen-units-std := 1
2020-04-22T09:36:19.6787459Z configure: rust.verify-llvm-ir  := True
2020-04-22T09:36:19.6788090Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-22T09:36:19.6788801Z configure: writing `config.toml` in current directory
2020-04-22T09:36:19.6789097Z configure: 
2020-04-22T09:36:19.6789563Z configure: run `python /checkout/x.py --help`
2020-04-22T09:36:19.6789902Z configure: 
---
2020-04-22T09:37:46.4801873Z Hugepagesize:       2048 kB
2020-04-22T09:37:46.4802073Z DirectMap4k:      139200 kB
2020-04-22T09:37:46.4802268Z DirectMap2M:     4055040 kB
2020-04-22T09:37:46.4802482Z DirectMap1G:     5242880 kB
2020-04-22T09:37:46.4840904Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-22T09:37:47.8061561Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-22T09:37:47.8061561Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-22T09:37:47.8072427Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-22T09:37:48.0311275Z    Compiling unicode-xid v0.2.0
2020-04-22T09:37:48.1549409Z    Compiling syn v1.0.11
2020-04-22T09:37:48.9474808Z    Compiling linked-hash-map v0.5.2
2020-04-22T09:37:48.9826262Z    Compiling lazy_static v1.4.0
2020-04-22T09:37:48.9826262Z    Compiling lazy_static v1.4.0
2020-04-22T09:37:49.1684635Z    Compiling yaml-rust v0.4.3
2020-04-22T09:37:53.2298292Z    Compiling quote v1.0.2
2020-04-22T09:38:06.7150314Z    Compiling thiserror-impl v1.0.5
2020-04-22T09:38:11.2037901Z    Compiling thiserror v1.0.5
2020-04-22T09:38:11.2715473Z    Compiling yaml-merge-keys v0.4.0
2020-04-22T09:38:12.3919701Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-22T09:38:13.9350043Z Build completed successfully in 0:00:27
2020-04-22T09:38:13.9441209Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-22T09:38:14.2100357Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-22T09:38:15.2733561Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-22T09:40:15.3379548Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T09:40:15.5153901Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T09:40:15.7222652Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T09:40:15.7496709Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T09:40:16.3210944Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T09:40:18.4368480Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T09:40:18.8945700Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T09:40:20.8671916Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T09:40:21.3236103Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T09:44:03.2364037Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-22T09:44:03.2364434Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-22T09:44:03.2370105Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-22T09:44:03.2374115Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-22T09:44:04.0831832Z Diff in /checkout/src/librustc_lint/builtin.rs at line 28:
2020-04-22T09:44:04.0836164Z  use rustc_ast_pretty::pprust::{self, expr_to_string};
2020-04-22T09:44:04.0839387Z  use rustc_data_structures::fx::FxHashSet;
2020-04-22T09:44:04.0842434Z  use rustc_errors::{Applicability, DiagnosticBuilder};
2020-04-22T09:44:04.0845946Z -use rustc_feature::{Stability, GateIssue};
2020-04-22T09:44:04.0849213Z  use rustc_feature::{deprecated_attributes, AttributeGate, AttributeTemplate, AttributeType};
2020-04-22T09:44:04.0852014Z +use rustc_feature::{GateIssue, Stability};
2020-04-22T09:44:04.0854765Z  use rustc_hir as hir;
2020-04-22T09:44:04.0888854Z  use rustc_hir::def_id::DefId;
2020-04-22T09:44:04.0888854Z  use rustc_hir::def_id::DefId;
2020-04-22T09:44:04.0900031Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_lint/builtin.rs"` failed.
2020-04-22T09:44:04.0901216Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-22T09:44:04.0915205Z Build completed unsuccessfully in 0:00:29
2020-04-22T09:44:04.1021291Z == clock drift check ==
2020-04-22T09:44:04.1038138Z   local time: Wed Apr 22 09:44:04 UTC 2020
2020-04-22T09:44:04.1038138Z   local time: Wed Apr 22 09:44:04 UTC 2020
2020-04-22T09:44:04.2632496Z   network time: Wed, 22 Apr 2020 09:44:04 GMT
2020-04-22T09:44:05.9065660Z 
2020-04-22T09:44:05.9065660Z 
2020-04-22T09:44:05.9147755Z ##[error]Bash exited with code '1'.
2020-04-22T09:44:05.9162314Z ##[section]Finishing: Run build
2020-04-22T09:44:05.9209581Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T09:44:05.9214287Z Task         : Get sources
2020-04-22T09:44:05.9214598Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T09:44:05.9214880Z Version      : 1.0.0
2020-04-22T09:44:05.9215101Z Author       : Microsoft
2020-04-22T09:44:05.9215101Z Author       : Microsoft
2020-04-22T09:44:05.9215425Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T09:44:05.9215788Z ==============================================================================
2020-04-22T09:44:06.2495413Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T09:44:06.2548668Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T09:44:06.2641376Z Cleaning up task key
2020-04-22T09:44:06.2642687Z Start cleaning up orphan processes.
2020-04-22T09:44:06.2823876Z Terminate orphan process: pid (3679) (python)
2020-04-22T09:44:06.2976659Z ##[section]Finishing: Finalize Job
