plain
2020-04-18T06:56:45.4367914Z ========================== Starting Command Output ===========================
2020-04-18T06:56:45.4372176Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/39c18008-e123-4505-b4a0-f32cdf4783a5.sh
2020-04-18T06:56:45.4372608Z 
2020-04-18T06:56:45.4379315Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T06:56:45.4410142Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71272/merge to s
2020-04-18T06:56:45.4415299Z Task         : Get sources
2020-04-18T06:56:45.4415821Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T06:56:45.4416311Z Version      : 1.0.0
2020-04-18T06:56:45.4416652Z Author       : Microsoft
---
2020-04-18T06:56:46.4479943Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T06:56:46.4484154Z ##[command]git config gc.auto 0
2020-04-18T06:56:46.4486932Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T06:56:46.4489483Z ##[command]git config --get-all http.proxy
2020-04-18T06:56:46.4494805Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71272/merge:refs/remotes/pull/71272/merge
---
2020-04-18T06:59:23.0984905Z  ---> 78ad2f4d4aca
2020-04-18T06:59:23.0985122Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-18T06:59:23.0985470Z  ---> Using cache
2020-04-18T06:59:23.0985791Z  ---> 4d2dc61c4d00
2020-04-18T06:59:23.0987018Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-18T06:59:23.0988214Z  ---> 776b6266a8b7
2020-04-18T06:59:23.0988404Z Successfully built 776b6266a8b7
2020-04-18T06:59:23.1033727Z Successfully tagged rust-ci:latest
2020-04-18T06:59:23.1327714Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-18T06:59:23.1327714Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-18T06:59:23.1342067Z Looks like docker image is the same as before, not uploading
2020-04-18T06:59:23.8278081Z [CI_JOB_NAME=mingw-check]
2020-04-18T06:59:23.8471540Z [CI_JOB_NAME=mingw-check]
2020-04-18T06:59:23.8510171Z == clock drift check ==
2020-04-18T06:59:23.8519390Z   local time: Sat Apr 18 06:59:23 UTC 2020
2020-04-18T06:59:24.0299077Z   network time: Sat, 18 Apr 2020 06:59:24 GMT
2020-04-18T06:59:24.0326120Z Starting sccache server...
2020-04-18T06:59:24.1339072Z configure: processing command line
2020-04-18T06:59:24.1340436Z configure: 
2020-04-18T06:59:24.1341354Z configure: rust.parallel-compiler := True
---
2020-04-18T07:02:36.5725732Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T07:02:36.6041449Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T07:02:36.7624008Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T07:02:36.9094498Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T07:02:37.3463785Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T07:02:39.3017365Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T07:02:39.6833707Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T07:02:41.3752156Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T07:02:41.7372924Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T07:04:10.6104043Z configure: rust.debug-assertions := True
2020-04-18T07:04:10.6104415Z configure: build.cargo-native-static := True
2020-04-18T07:04:10.6104639Z configure: rust.channel         := nightly
2020-04-18T07:04:10.6104853Z configure: build.submodules     := False
2020-04-18T07:04:10.6105305Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-18T07:04:10.6105704Z configure: writing `config.toml` in current directory
2020-04-18T07:04:10.6105893Z configure: 
2020-04-18T07:04:10.6106195Z configure: run `python /checkout/x.py --help`
2020-04-18T07:04:10.6106360Z configure: 
---
2020-04-18T07:05:34.0608077Z Hugepagesize:       2048 kB
2020-04-18T07:05:34.0608239Z DirectMap4k:      131008 kB
2020-04-18T07:05:34.0608413Z DirectMap2M:     4063232 kB
2020-04-18T07:05:34.0608574Z DirectMap1G:     5242880 kB
2020-04-18T07:05:34.0650519Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-18T07:05:35.2103354Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-18T07:05:35.2103354Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-18T07:05:35.2139472Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-18T07:05:35.4127403Z    Compiling unicode-xid v0.2.0
2020-04-18T07:05:35.5302873Z    Compiling syn v1.0.11
2020-04-18T07:05:36.2752363Z    Compiling linked-hash-map v0.5.2
2020-04-18T07:05:36.2971884Z    Compiling lazy_static v1.4.0
2020-04-18T07:05:36.2971884Z    Compiling lazy_static v1.4.0
2020-04-18T07:05:36.4739564Z    Compiling yaml-rust v0.4.3
2020-04-18T07:05:40.0220548Z    Compiling quote v1.0.2
2020-04-18T07:05:51.8362975Z    Compiling thiserror-impl v1.0.5
2020-04-18T07:05:55.6781724Z    Compiling thiserror v1.0.5
2020-04-18T07:05:55.7352002Z    Compiling yaml-merge-keys v0.4.0
2020-04-18T07:05:56.7021699Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-18T07:05:58.0755267Z     Finished release [optimized] target(s) in 22.86s
2020-04-18T07:05:58.0900984Z error: .github/workflows/ci.yml is not up to date
2020-04-18T07:05:58.0902486Z caused by: src/ci/github-actions/ci.yml and .github/workflows/ci.yml are different
2020-04-18T07:05:58.0903061Z 
2020-04-18T07:05:58.0903061Z 
2020-04-18T07:05:58.0904161Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/expand-yaml-anchors" "check" "/checkout"
2020-04-18T07:05:58.0905222Z 
2020-04-18T07:05:58.0905386Z 
2020-04-18T07:05:58.0905386Z 
2020-04-18T07:05:58.0911568Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/expand-yaml-anchors
2020-04-18T07:05:58.1010381Z == clock drift check ==
2020-04-18T07:05:58.1022959Z   local time: Sat Apr 18 07:05:58 UTC 2020
2020-04-18T07:05:58.1022959Z   local time: Sat Apr 18 07:05:58 UTC 2020
2020-04-18T07:05:58.4081758Z   network time: Sat, 18 Apr 2020 07:05:58 GMT
2020-04-18T07:06:03.9067103Z 
2020-04-18T07:06:03.9067103Z 
2020-04-18T07:06:03.9129276Z ##[error]Bash exited with code '1'.
2020-04-18T07:06:03.9143794Z ##[section]Finishing: Run build
2020-04-18T07:06:03.9189428Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71272/merge to s
2020-04-18T07:06:03.9193699Z Task         : Get sources
2020-04-18T07:06:03.9193988Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T07:06:03.9194252Z Version      : 1.0.0
2020-04-18T07:06:03.9194454Z Author       : Microsoft
2020-04-18T07:06:03.9194454Z Author       : Microsoft
2020-04-18T07:06:03.9194752Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T07:06:03.9195088Z ==============================================================================
2020-04-18T07:06:04.2190874Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T07:06:04.2235832Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71272/merge to s
2020-04-18T07:06:04.2319020Z Cleaning up task key
2020-04-18T07:06:04.2320158Z Start cleaning up orphan processes.
2020-04-18T07:06:04.2473834Z Terminate orphan process: pid (4448) (python)
2020-04-18T07:06:04.2745514Z ##[section]Finishing: Finalize Job
