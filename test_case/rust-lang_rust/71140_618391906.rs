plain
2020-04-23T11:25:24.9690714Z ========================== Starting Command Output ===========================
2020-04-23T11:25:24.9695015Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d0b7202d-c9f6-4854-b10d-c4cc692f6a27.sh
2020-04-23T11:25:24.9695418Z 
2020-04-23T11:25:24.9699601Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T11:25:24.9715460Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71140/merge to s
2020-04-23T11:25:24.9718520Z Task         : Get sources
2020-04-23T11:25:24.9718776Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T11:25:24.9719004Z Version      : 1.0.0
2020-04-23T11:25:24.9719161Z Author       : Microsoft
---
2020-04-23T11:25:26.5383926Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T11:25:26.5394360Z ##[command]git config gc.auto 0
2020-04-23T11:25:26.5415964Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T11:25:26.5424770Z ##[command]git config --get-all http.proxy
2020-04-23T11:25:27.5350191Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71140/merge:refs/remotes/pull/71140/merge
---
2020-04-23T11:27:52.3963384Z  ---> 78ad2f4d4aca
2020-04-23T11:27:52.3963687Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-23T11:27:52.3969239Z  ---> Using cache
2020-04-23T11:27:52.3969764Z  ---> 4d2dc61c4d00
2020-04-23T11:27:52.3971266Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-23T11:27:52.3976665Z  ---> 776b6266a8b7
2020-04-23T11:27:52.4058088Z Successfully built 776b6266a8b7
2020-04-23T11:27:52.4093826Z Successfully tagged rust-ci:latest
2020-04-23T11:27:52.4555544Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-23T11:27:52.4555544Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-23T11:27:52.4574891Z Looks like docker image is the same as before, not uploading
2020-04-23T11:28:00.4846842Z [CI_JOB_NAME=mingw-check]
2020-04-23T11:28:00.5109099Z [CI_JOB_NAME=mingw-check]
2020-04-23T11:28:00.5143764Z == clock drift check ==
2020-04-23T11:28:00.5152563Z   local time: Thu Apr 23 11:28:00 UTC 2020
2020-04-23T11:28:00.8044283Z   network time: Thu, 23 Apr 2020 11:28:00 GMT
2020-04-23T11:28:00.8072630Z Starting sccache server...
2020-04-23T11:28:00.9246781Z configure: processing command line
2020-04-23T11:28:00.9247067Z configure: 
2020-04-23T11:28:00.9248128Z configure: rust.parallel-compiler := True
---
2020-04-23T11:32:01.2971465Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T11:32:01.5739959Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T11:32:01.7184701Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T11:32:01.7736536Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T11:32:02.3924796Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T11:32:04.8096561Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T11:32:05.3150045Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T11:32:07.4243630Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T11:32:07.8531027Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T11:33:59.3122154Z configure: rust.debug-assertions := True
2020-04-23T11:33:59.3122554Z configure: rust.codegen-units-std := 1
2020-04-23T11:33:59.3122968Z configure: rust.verify-llvm-ir  := True
2020-04-23T11:33:59.3123395Z configure: build.cargo-native-static := True
2020-04-23T11:33:59.3123897Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-23T11:33:59.3124373Z configure: writing `config.toml` in current directory
2020-04-23T11:33:59.3124563Z configure: 
2020-04-23T11:33:59.3124917Z configure: run `python /checkout/x.py --help`
2020-04-23T11:33:59.3125120Z configure: 
---
2020-04-23T11:35:34.0664510Z Hugepagesize:       2048 kB
2020-04-23T11:35:34.0664758Z DirectMap4k:      128960 kB
2020-04-23T11:35:34.0664988Z DirectMap2M:     4065280 kB
2020-04-23T11:35:34.0665219Z DirectMap1G:     5242880 kB
2020-04-23T11:35:34.0674356Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-23T11:35:35.4678883Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-23T11:35:35.4678883Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-23T11:35:35.4691491Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-23T11:35:35.7247931Z    Compiling unicode-xid v0.2.0
2020-04-23T11:35:35.8702591Z    Compiling syn v1.0.11
2020-04-23T11:35:36.7348738Z    Compiling linked-hash-map v0.5.2
2020-04-23T11:35:36.7867436Z    Compiling lazy_static v1.4.0
2020-04-23T11:35:36.7867436Z    Compiling lazy_static v1.4.0
2020-04-23T11:35:36.9703117Z    Compiling yaml-rust v0.4.3
2020-04-23T11:35:41.4213440Z    Compiling quote v1.0.2
2020-04-23T11:35:56.5947123Z    Compiling thiserror-impl v1.0.5
2020-04-23T11:36:01.5656568Z    Compiling thiserror v1.0.5
2020-04-23T11:36:01.6291758Z    Compiling yaml-merge-keys v0.4.0
2020-04-23T11:36:02.8580834Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-23T11:36:05.6829908Z Build completed successfully in 0:00:31
2020-04-23T11:36:05.6929604Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-23T11:36:05.9819831Z     Finished dev [unoptimized] target(s) in 0.20s
2020-04-23T11:36:07.1481828Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-23T11:38:16.7800072Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T11:38:16.8746804Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T11:38:17.0949107Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T11:38:17.2259887Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T11:38:17.7939315Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T11:38:20.0432175Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T11:38:20.5155170Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T11:38:22.7017557Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T11:38:23.1907181Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T11:42:33.7714122Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-23T11:42:33.7714540Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-23T11:42:33.7720313Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-23T11:42:33.7721564Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-23T11:42:36.5446799Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/interpret/memory.rs"` failed.
2020-04-23T11:42:36.5447909Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-23T11:42:36.5448338Z Diff in /checkout/src/librustc_mir/interpret/memory.rs at line 409:
2020-04-23T11:42:36.5448916Z                      self.get_raw(ptr.alloc_id)?;
2020-04-23T11:42:36.5449140Z                      None
2020-04-23T11:42:36.5449554Z -                } else { Some(ptr) }
2020-04-23T11:42:36.5449787Z +                } else {
---
2020-04-23T11:42:36.5450638Z      }
2020-04-23T11:42:36.5457580Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-23T11:42:36.5457942Z Build completed unsuccessfully in 0:00:40
2020-04-23T11:42:36.5559553Z == clock drift check ==
2020-04-23T11:42:36.5605710Z   local time: Thu Apr 23 11:42:36 UTC 2020
2020-04-23T11:42:36.8498806Z   network time: Thu, 23 Apr 2020 11:42:36 GMT
2020-04-23T11:42:38.3235325Z 
2020-04-23T11:42:38.3235325Z 
2020-04-23T11:42:38.3318122Z ##[error]Bash exited with code '1'.
2020-04-23T11:42:38.3334260Z ##[section]Finishing: Run build
2020-04-23T11:42:38.3398658Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71140/merge to s
2020-04-23T11:42:38.3404517Z Task         : Get sources
2020-04-23T11:42:38.3404872Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T11:42:38.3405221Z Version      : 1.0.0
2020-04-23T11:42:38.3405475Z Author       : Microsoft
2020-04-23T11:42:38.3405475Z Author       : Microsoft
2020-04-23T11:42:38.3405845Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T11:42:38.3406282Z ==============================================================================
2020-04-23T11:42:38.6988020Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T11:42:38.7033131Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71140/merge to s
2020-04-23T11:42:38.7149220Z Cleaning up task key
2020-04-23T11:42:38.7150550Z Start cleaning up orphan processes.
2020-04-23T11:42:38.7377068Z Terminate orphan process: pid (3341) (python)
2020-04-23T11:42:38.7523929Z ##[section]Finishing: Finalize Job
