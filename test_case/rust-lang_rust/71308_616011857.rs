plain
2020-04-19T01:02:01.5274176Z ========================== Starting Command Output ===========================
2020-04-19T01:02:01.5290002Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a79a38af-7446-4e2f-badf-19fe207abd86.sh
2020-04-19T01:02:01.5492652Z 
2020-04-19T01:02:01.5556609Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T01:02:01.5576934Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71308/merge to s
2020-04-19T01:02:01.5581949Z Task         : Get sources
2020-04-19T01:02:01.5582344Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T01:02:01.5582755Z Version      : 1.0.0
2020-04-19T01:02:01.5582964Z Author       : Microsoft
---
2020-04-19T01:02:02.4036158Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T01:02:02.4154879Z ##[command]git config gc.auto 0
2020-04-19T01:02:02.4209980Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T01:02:02.4223187Z ##[command]git config --get-all http.proxy
2020-04-19T01:02:02.4308485Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71308/merge:refs/remotes/pull/71308/merge
---
2020-04-19T01:05:02.8701121Z  ---> 78ad2f4d4aca
2020-04-19T01:05:02.8703050Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-19T01:05:02.8703623Z  ---> Using cache
2020-04-19T01:05:02.8703935Z  ---> 4d2dc61c4d00
2020-04-19T01:05:02.8705453Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-19T01:05:02.8706785Z  ---> 776b6266a8b7
2020-04-19T01:05:02.8736385Z Successfully built 776b6266a8b7
2020-04-19T01:05:02.8781595Z Successfully tagged rust-ci:latest
2020-04-19T01:05:03.0484990Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-19T01:05:03.0484990Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-19T01:05:03.0500603Z Looks like docker image is the same as before, not uploading
2020-04-19T01:05:11.1198566Z [CI_JOB_NAME=mingw-check]
2020-04-19T01:05:11.1648959Z [CI_JOB_NAME=mingw-check]
2020-04-19T01:05:11.1673097Z == clock drift check ==
2020-04-19T01:05:11.1685320Z   local time: Sun Apr 19 01:05:11 UTC 2020
2020-04-19T01:05:11.4732226Z   network time: Sun, 19 Apr 2020 01:05:11 GMT
2020-04-19T01:05:11.4762213Z Starting sccache server...
2020-04-19T01:05:11.5840673Z configure: processing command line
2020-04-19T01:05:11.5841011Z configure: 
2020-04-19T01:05:11.5841873Z configure: rust.parallel-compiler := True
---
2020-04-19T01:08:59.1829465Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T01:08:59.1982379Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T01:08:59.3980750Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T01:08:59.6007120Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T01:09:00.0506890Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T01:09:03.0976492Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T01:09:03.5794931Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T01:09:05.7231039Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T01:09:06.1577951Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T01:10:54.0765640Z configure: rust.channel         := nightly
2020-04-19T01:10:54.0766053Z configure: rust.debug-assertions := True
2020-04-19T01:10:54.0766743Z configure: build.cargo-native-static := True
2020-04-19T01:10:54.0767209Z configure: build.locked-deps    := True
2020-04-19T01:10:54.0767724Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-19T01:10:54.0768208Z configure: writing `config.toml` in current directory
2020-04-19T01:10:54.0768410Z configure: 
2020-04-19T01:10:54.0768895Z configure: run `python /checkout/x.py --help`
2020-04-19T01:10:54.0769243Z configure: 
---
2020-04-19T01:12:27.9384378Z Hugepagesize:       2048 kB
2020-04-19T01:12:27.9384581Z DirectMap4k:      122816 kB
2020-04-19T01:12:27.9384767Z DirectMap2M:     5120000 kB
2020-04-19T01:12:27.9384953Z DirectMap1G:     4194304 kB
2020-04-19T01:12:27.9449842Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-19T01:12:29.2711146Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-19T01:12:29.2711146Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-19T01:12:29.2713684Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-19T01:12:29.4925105Z    Compiling unicode-xid v0.2.0
2020-04-19T01:12:29.6193049Z    Compiling syn v1.0.11
2020-04-19T01:12:30.4723003Z    Compiling linked-hash-map v0.5.2
2020-04-19T01:12:30.5012916Z    Compiling lazy_static v1.4.0
2020-04-19T01:12:30.5012916Z    Compiling lazy_static v1.4.0
2020-04-19T01:12:30.6968467Z    Compiling yaml-rust v0.4.3
2020-04-19T01:12:35.0305718Z    Compiling quote v1.0.2
2020-04-19T01:12:49.6848046Z    Compiling thiserror-impl v1.0.5
2020-04-19T01:12:54.4420398Z    Compiling thiserror v1.0.5
2020-04-19T01:12:54.5015521Z    Compiling yaml-merge-keys v0.4.0
2020-04-19T01:12:55.7210401Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-19T01:12:58.0748494Z Build completed successfully in 0:00:30
2020-04-19T01:12:58.0849453Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-19T01:12:58.3443717Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-19T01:12:59.4129452Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-19T01:15:07.0357295Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T01:15:07.0892245Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T01:15:07.2927443Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T01:15:07.4601549Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T01:15:07.9471730Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T01:15:10.3211427Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T01:15:10.8310074Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T01:15:12.9594233Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T01:15:13.4157382Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T01:19:17.2123404Z Diff in /checkout/src/libstd/path.rs at line 847:
2020-04-19T01:19:17.2125272Z          self.inner.next().map(Component::as_os_str)
2020-04-19T01:19:17.2125652Z      }
2020-04-19T01:19:17.2125897Z  
2020-04-19T01:19:17.2127230Z -    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
2020-04-19T01:19:17.2131405Z +    fn size_hint(&self) -> (usize, Option<usize>) {
2020-04-19T01:19:17.2132008Z +        self.inner.size_hint()
2020-04-19T01:19:17.2132516Z  }
2020-04-19T01:19:17.2132740Z  
2020-04-19T01:19:17.2133008Z  #[stable(feature = "rust1", since = "1.0.0")]
2020-04-19T01:19:17.2133008Z  #[stable(feature = "rust1", since = "1.0.0")]
2020-04-19T01:19:17.2149860Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/path.rs"` failed.
2020-04-19T01:19:17.2150937Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-19T01:19:17.2164654Z Build completed unsuccessfully in 0:00:36
2020-04-19T01:19:17.2267606Z == clock drift check ==
2020-04-19T01:19:17.2284852Z   local time: Sun Apr 19 01:19:17 UTC 2020
2020-04-19T01:19:17.2284852Z   local time: Sun Apr 19 01:19:17 UTC 2020
2020-04-19T01:19:17.4949425Z   network time: Sun, 19 Apr 2020 01:19:17 GMT
2020-04-19T01:19:19.2283094Z 
2020-04-19T01:19:19.2283094Z 
2020-04-19T01:19:19.2353873Z ##[error]Bash exited with code '1'.
2020-04-19T01:19:19.2368715Z ##[section]Finishing: Run build
2020-04-19T01:19:19.2416656Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71308/merge to s
2020-04-19T01:19:19.2421405Z Task         : Get sources
2020-04-19T01:19:19.2421836Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T01:19:19.2422137Z Version      : 1.0.0
2020-04-19T01:19:19.2422347Z Author       : Microsoft
2020-04-19T01:19:19.2422347Z Author       : Microsoft
2020-04-19T01:19:19.2422671Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T01:19:19.2423053Z ==============================================================================
2020-04-19T01:19:19.6010634Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T01:19:19.6060321Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71308/merge to s
2020-04-19T01:19:19.6159514Z Cleaning up task key
2020-04-19T01:19:19.6161091Z Start cleaning up orphan processes.
2020-04-19T01:19:19.6369502Z Terminate orphan process: pid (4562) (python)
2020-04-19T01:19:19.6665408Z ##[section]Finishing: Finalize Job
