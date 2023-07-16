plain
2020-04-12T22:02:19.3255192Z ========================== Starting Command Output ===========================
2020-04-12T22:02:19.3260142Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2fcc4ba6-074a-40fe-982d-a9c9efd51921.sh
2020-04-12T22:02:19.3260691Z 
2020-04-12T22:02:19.3265382Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T22:02:19.3282251Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-12T22:02:19.3285148Z Task         : Get sources
2020-04-12T22:02:19.3285404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T22:02:19.3285649Z Version      : 1.0.0
2020-04-12T22:02:19.3285818Z Author       : Microsoft
---
2020-04-12T22:02:20.4848034Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T22:02:20.4854825Z ##[command]git config gc.auto 0
2020-04-12T22:02:20.4859091Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T22:02:20.4862971Z ##[command]git config --get-all http.proxy
2020-04-12T22:02:20.4870860Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71003/merge:refs/remotes/pull/71003/merge
---
2020-04-12T22:05:32.7755127Z  ---> 78ad2f4d4aca
2020-04-12T22:05:32.7755377Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-12T22:05:32.7760887Z  ---> Using cache
2020-04-12T22:05:32.7761225Z  ---> 4d2dc61c4d00
2020-04-12T22:05:32.7762430Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-12T22:05:32.7776624Z  ---> 776b6266a8b7
2020-04-12T22:05:32.7822381Z Successfully built 776b6266a8b7
2020-04-12T22:05:32.7875739Z Successfully tagged rust-ci:latest
2020-04-12T22:05:32.8383472Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-12T22:05:32.8383472Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-12T22:05:32.8398237Z Looks like docker image is the same as before, not uploading
2020-04-12T22:05:40.2273295Z [CI_JOB_NAME=mingw-check]
2020-04-12T22:05:40.2487408Z [CI_JOB_NAME=mingw-check]
2020-04-12T22:05:40.2516873Z == clock drift check ==
2020-04-12T22:05:40.2527118Z   local time: Sun Apr 12 22:05:40 UTC 2020
2020-04-12T22:05:40.5573665Z   network time: Sun, 12 Apr 2020 22:05:40 GMT
2020-04-12T22:05:40.5598359Z Starting sccache server...
2020-04-12T22:05:40.6682782Z configure: processing command line
2020-04-12T22:05:40.6683083Z configure: 
2020-04-12T22:05:40.6684003Z configure: rust.parallel-compiler := True
---
2020-04-12T22:09:50.6386537Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T22:09:50.8711348Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T22:09:51.0774728Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T22:09:51.0909852Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T22:09:51.6871249Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T22:09:54.1029892Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T22:09:54.5847323Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T22:09:56.6005519Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T22:09:57.0527961Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T22:11:48.7682481Z configure: rust.codegen-units-std := 1
2020-04-12T22:11:48.7682991Z configure: rust.verify-llvm-ir  := True
2020-04-12T22:11:48.7683390Z configure: build.cargo-native-static := True
2020-04-12T22:11:48.7683658Z configure: llvm.assertions      := True
2020-04-12T22:11:48.7684151Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-12T22:11:48.7684636Z configure: writing `config.toml` in current directory
2020-04-12T22:11:48.7684841Z configure: 
2020-04-12T22:11:48.7685279Z configure: run `python /checkout/x.py --help`
2020-04-12T22:11:48.7685499Z configure: 
---
2020-04-12T22:13:29.6327029Z Hugepagesize:       2048 kB
2020-04-12T22:13:29.6327239Z DirectMap4k:      135104 kB
2020-04-12T22:13:29.6327430Z DirectMap2M:     4059136 kB
2020-04-12T22:13:29.6327621Z DirectMap1G:     5242880 kB
2020-04-12T22:13:29.6355775Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-12T22:13:31.0789506Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-12T22:13:31.0789506Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-12T22:13:31.0799842Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-12T22:13:31.3499350Z    Compiling unicode-xid v0.2.0
2020-04-12T22:13:31.4672704Z    Compiling syn v1.0.11
2020-04-12T22:13:32.3645498Z    Compiling linked-hash-map v0.5.2
2020-04-12T22:13:32.4323296Z    Compiling lazy_static v1.4.0
2020-04-12T22:13:32.4323296Z    Compiling lazy_static v1.4.0
2020-04-12T22:13:32.5973337Z    Compiling yaml-rust v0.4.3
2020-04-12T22:13:37.1132705Z    Compiling quote v1.0.2
2020-04-12T22:13:52.1957812Z    Compiling thiserror-impl v1.0.5
2020-04-12T22:13:57.3438095Z    Compiling thiserror v1.0.5
2020-04-12T22:13:57.4009314Z    Compiling yaml-merge-keys v0.4.0
2020-04-12T22:13:58.6121349Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-12T22:14:00.2594246Z Build completed successfully in 0:00:30
2020-04-12T22:14:00.2681404Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-12T22:14:00.5684159Z     Finished dev [unoptimized] target(s) in 0.20s
2020-04-12T22:14:01.7208603Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-12T22:16:12.2177248Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T22:16:12.2750686Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T22:16:12.4686781Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T22:16:12.6486135Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T22:16:13.0861052Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T22:16:15.4267581Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T22:16:15.9299937Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T22:16:18.1333982Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T22:16:18.5876012Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T22:20:04.3698045Z    Compiling cargo_metadata v0.9.1
2020-04-12T22:20:08.8960653Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-12T22:20:19.9594219Z     Finished release [optimized] target(s) in 25.51s
2020-04-12T22:20:19.9684928Z tidy check
2020-04-12T22:20:26.0163670Z tidy error: /checkout/src/librustc_mir/transform/dag_nrvo.rs:34: TODO is deprecated; use FIXME
2020-04-12T22:20:28.8869517Z Found 490 error codes
2020-04-12T22:20:28.8869860Z Found 0 error codes with no tests
2020-04-12T22:20:28.8870193Z Done!
2020-04-12T22:20:28.8870382Z some tidy checks failed
2020-04-12T22:20:28.8870382Z some tidy checks failed
2020-04-12T22:20:28.8871392Z 
2020-04-12T22:20:28.8872913Z 
2020-04-12T22:20:28.8874586Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-12T22:20:28.8875416Z 
2020-04-12T22:20:28.8875545Z 
2020-04-12T22:20:28.8876324Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-12T22:20:28.8876731Z Build completed unsuccessfully in 0:00:35
2020-04-12T22:20:28.8876731Z Build completed unsuccessfully in 0:00:35
2020-04-12T22:20:28.8988335Z == clock drift check ==
2020-04-12T22:20:28.9002559Z   local time: Sun Apr 12 22:20:28 UTC 2020
2020-04-12T22:20:29.0206317Z   network time: Sun, 12 Apr 2020 22:20:29 GMT
2020-04-12T22:20:30.5751194Z 
2020-04-12T22:20:30.5751194Z 
2020-04-12T22:20:30.5824232Z ##[error]Bash exited with code '1'.
2020-04-12T22:20:30.5839261Z ##[section]Finishing: Run build
2020-04-12T22:20:30.5881785Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-12T22:20:30.5886080Z Task         : Get sources
2020-04-12T22:20:30.5886383Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T22:20:30.5886675Z Version      : 1.0.0
2020-04-12T22:20:30.5886873Z Author       : Microsoft
2020-04-12T22:20:30.5886873Z Author       : Microsoft
2020-04-12T22:20:30.5887201Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T22:20:30.5887573Z ==============================================================================
2020-04-12T22:20:30.9510690Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T22:20:30.9557704Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-12T22:20:30.9656259Z Cleaning up task key
2020-04-12T22:20:30.9657637Z Start cleaning up orphan processes.
2020-04-12T22:20:30.9891591Z Terminate orphan process: pid (3480) (python)
2020-04-12T22:20:31.0093384Z ##[section]Finishing: Finalize Job
