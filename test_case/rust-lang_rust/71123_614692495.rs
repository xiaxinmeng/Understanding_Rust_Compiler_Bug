plain
2020-04-16T12:38:40.9451529Z ========================== Starting Command Output ===========================
2020-04-16T12:38:40.9454878Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2e5b5b0a-09f6-4948-993c-df2df752d1f0.sh
2020-04-16T12:38:40.9455285Z 
2020-04-16T12:38:40.9460051Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-16T12:38:40.9476126Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71123/merge to s
2020-04-16T12:38:40.9478779Z Task         : Get sources
2020-04-16T12:38:40.9478999Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T12:38:40.9479212Z Version      : 1.0.0
2020-04-16T12:38:40.9479394Z Author       : Microsoft
---
2020-04-16T12:38:42.3095229Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-16T12:38:42.3100801Z ##[command]git config gc.auto 0
2020-04-16T12:38:42.3103880Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-16T12:38:42.3106664Z ##[command]git config --get-all http.proxy
2020-04-16T12:38:42.3112708Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71123/merge:refs/remotes/pull/71123/merge
---
2020-04-16T12:40:52.8106939Z  ---> 78ad2f4d4aca
2020-04-16T12:40:52.8107307Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-16T12:40:52.8107736Z  ---> Using cache
2020-04-16T12:40:52.8108168Z  ---> 4d2dc61c4d00
2020-04-16T12:40:52.8109274Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-16T12:40:52.8112406Z  ---> 776b6266a8b7
2020-04-16T12:40:52.8140663Z Successfully built 776b6266a8b7
2020-04-16T12:40:52.8177455Z Successfully tagged rust-ci:latest
2020-04-16T12:40:52.8417459Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-16T12:40:52.8417459Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-16T12:40:52.8440016Z Looks like docker image is the same as before, not uploading
2020-04-16T12:41:01.1408872Z [CI_JOB_NAME=mingw-check]
2020-04-16T12:41:01.1637421Z [CI_JOB_NAME=mingw-check]
2020-04-16T12:41:01.1657971Z == clock drift check ==
2020-04-16T12:41:01.1660784Z   local time: Thu Apr 16 12:41:01 UTC 2020
2020-04-16T12:41:01.5503825Z   network time: Thu, 16 Apr 2020 12:41:01 GMT
2020-04-16T12:41:01.5519165Z Starting sccache server...
2020-04-16T12:41:01.6377552Z configure: processing command line
2020-04-16T12:41:01.6381561Z configure: 
2020-04-16T12:41:01.6382342Z configure: rust.parallel-compiler := True
---
2020-04-16T12:44:07.2179151Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-16T12:44:10.8510905Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-16T12:44:11.8649234Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T12:44:11.8926532Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T12:44:12.0489290Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T12:44:12.6762687Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T12:44:12.7351728Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-16T12:44:13.9831616Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T12:44:14.3693056Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-16T12:45:44.6234770Z configure: rust.channel         := nightly
2020-04-16T12:45:44.6235354Z configure: build.cargo-native-static := True
2020-04-16T12:45:44.6235961Z configure: rust.debug-assertions := True
2020-04-16T12:45:44.6236647Z configure: build.locked-deps    := True
2020-04-16T12:45:44.6237356Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-16T12:45:44.6238154Z configure: writing `config.toml` in current directory
2020-04-16T12:45:44.6238485Z configure: 
2020-04-16T12:45:44.6239010Z configure: run `python /checkout/x.py --help`
2020-04-16T12:45:44.6239391Z configure: 
---
2020-04-16T12:47:00.3302447Z Hugepagesize:       2048 kB
2020-04-16T12:47:00.3302614Z DirectMap4k:      135104 kB
2020-04-16T12:47:00.3302769Z DirectMap2M:     3010560 kB
2020-04-16T12:47:00.3302938Z DirectMap1G:     6291456 kB
2020-04-16T12:47:00.3303464Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-16T12:47:01.4508686Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-16T12:47:01.4508686Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-16T12:47:01.4538045Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-16T12:47:01.6374216Z    Compiling unicode-xid v0.2.0
2020-04-16T12:47:01.7405883Z    Compiling syn v1.0.11
2020-04-16T12:47:02.4501857Z    Compiling linked-hash-map v0.5.2
2020-04-16T12:47:02.4961837Z    Compiling lazy_static v1.4.0
2020-04-16T12:47:02.4961837Z    Compiling lazy_static v1.4.0
2020-04-16T12:47:02.6439700Z    Compiling yaml-rust v0.4.3
2020-04-16T12:47:06.3390920Z    Compiling quote v1.0.2
2020-04-16T12:47:18.3918812Z    Compiling thiserror-impl v1.0.5
2020-04-16T12:47:22.2995356Z    Compiling thiserror v1.0.5
2020-04-16T12:47:22.3502859Z    Compiling yaml-merge-keys v0.4.0
2020-04-16T12:47:23.3286136Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-16T12:47:24.7398783Z Build completed successfully in 0:00:24
2020-04-16T12:47:24.7473547Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-16T12:47:24.9548648Z     Finished dev [unoptimized] target(s) in 0.13s
2020-04-16T12:47:25.8211615Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-16T12:49:10.1022197Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T12:49:10.1997664Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T12:49:10.3713210Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T12:49:10.4547682Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T12:49:10.8894221Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T12:49:12.7824943Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T12:49:13.1735169Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T12:49:14.8642469Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T12:49:15.2483300Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T12:52:40.4260002Z Diff in /checkout/src/libstd/sys/unix/alloc.rs at line 52:
2020-04-16T12:52:40.4264318Z      }
2020-04-16T12:52:40.4268235Z  }
2020-04-16T12:52:40.4272361Z  
2020-04-16T12:52:40.4278773Z -#[cfg(any(
2020-04-16T12:52:40.4282942Z -    target_os = "illumos",
2020-04-16T12:52:40.4287074Z -    target_os = "redox",
2020-04-16T12:52:40.4291179Z -    target_os = "solaris"
2020-04-16T12:52:40.4295218Z -))]
2020-04-16T12:52:40.4313120Z +#[cfg(any(target_os = "illumos", target_os = "redox", target_os = "solaris"))]
2020-04-16T12:52:40.4313374Z  #[inline]
2020-04-16T12:52:40.4313910Z  unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
2020-04-16T12:52:40.4314178Z      // On platforms where `posix_memalign` is unavailable, we use `memalign`.
2020-04-16T12:52:40.4314463Z Diff in /checkout/src/libstd/sys/unix/alloc.rs at line 83:
2020-04-16T12:52:40.4314708Z      libc::memalign(layout.align(), layout.size()) as *mut u8
2020-04-16T12:52:40.4314988Z  
2020-04-16T12:52:40.4314988Z  
2020-04-16T12:52:40.4315247Z -#[cfg(not(any(
2020-04-16T12:52:40.4315517Z -    target_os = "illumos",
2020-04-16T12:52:40.4316015Z -    target_os = "redox",
2020-04-16T12:52:40.4316294Z -    target_os = "solaris"
2020-04-16T12:52:40.4316531Z -)))]
2020-04-16T12:52:40.4316870Z +#[cfg(not(any(target_os = "illumos", target_os = "redox", target_os = "solaris")))]
2020-04-16T12:52:40.4317079Z  #[inline]
2020-04-16T12:52:40.4317439Z  unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
2020-04-16T12:52:40.4317665Z      let mut out = ptr::null_mut();
2020-04-16T12:52:40.4318344Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/sys/unix/alloc.rs"` failed.
2020-04-16T12:52:40.4319023Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-16T12:52:40.4324713Z Build completed unsuccessfully in 0:00:36
2020-04-16T12:52:40.4413444Z == clock drift check ==
2020-04-16T12:52:40.4413444Z == clock drift check ==
2020-04-16T12:52:40.4428905Z   local time: Thu Apr 16 12:52:40 UTC 2020
2020-04-16T12:52:40.6523126Z   network time: Thu, 16 Apr 2020 12:52:40 GMT
2020-04-16T12:52:42.1500789Z 
2020-04-16T12:52:42.1500789Z 
2020-04-16T12:52:42.1560942Z ##[error]Bash exited with code '1'.
2020-04-16T12:52:42.1572115Z ##[section]Finishing: Run build
2020-04-16T12:52:42.1608284Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71123/merge to s
2020-04-16T12:52:42.1612465Z Task         : Get sources
2020-04-16T12:52:42.1612710Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T12:52:42.1612944Z Version      : 1.0.0
2020-04-16T12:52:42.1613099Z Author       : Microsoft
2020-04-16T12:52:42.1613099Z Author       : Microsoft
2020-04-16T12:52:42.1613341Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-16T12:52:42.1613648Z ==============================================================================
2020-04-16T12:52:42.4327103Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-16T12:52:42.4360066Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71123/merge to s
2020-04-16T12:52:42.4436787Z Cleaning up task key
2020-04-16T12:52:42.4437883Z Start cleaning up orphan processes.
2020-04-16T12:52:42.4604862Z Terminate orphan process: pid (3704) (python)
2020-04-16T12:52:42.4783708Z ##[section]Finishing: Finalize Job
