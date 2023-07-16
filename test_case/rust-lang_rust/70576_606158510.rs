plain
2020-03-30T17:05:45.7363242Z ========================== Starting Command Output ===========================
2020-03-30T17:05:45.7367108Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9c7dcd20-c25b-4848-8ea3-e18f91f838bd.sh
2020-03-30T17:05:45.7367389Z 
2020-03-30T17:05:45.7371367Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-30T17:05:45.7390575Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70576/merge to s
2020-03-30T17:05:45.7393827Z Task         : Get sources
2020-03-30T17:05:45.7394151Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T17:05:45.7394446Z Version      : 1.0.0
2020-03-30T17:05:45.7394646Z Author       : Microsoft
---
2020-03-30T17:05:47.0056739Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-30T17:05:47.0062578Z ##[command]git config gc.auto 0
2020-03-30T17:05:47.0066303Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-30T17:05:47.0069730Z ##[command]git config --get-all http.proxy
2020-03-30T17:05:47.0077409Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70576/merge:refs/remotes/pull/70576/merge
---
2020-03-30T17:08:38.1763113Z  ---> 3fc1b512c57b
2020-03-30T17:08:38.1763376Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-30T17:08:38.1766591Z  ---> Using cache
2020-03-30T17:08:38.1766961Z  ---> 5ee4295733f4
2020-03-30T17:08:38.1768346Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-30T17:08:38.1772660Z  ---> 3d07a0fa42fe
2020-03-30T17:08:38.1807337Z Successfully built 3d07a0fa42fe
2020-03-30T17:08:38.1831277Z Successfully tagged rust-ci:latest
2020-03-30T17:08:38.2287192Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-30T17:12:18.2448717Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T17:12:18.3461211Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T17:12:18.5270171Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T17:12:18.6936618Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T17:12:19.1556660Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T17:12:21.3175200Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T17:12:21.7612435Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T17:12:23.6561682Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T17:12:24.0682098Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T17:14:07.1359985Z configure: build.locked-deps    := True
2020-03-30T17:14:07.1360415Z configure: llvm.ccache          := sccache
2020-03-30T17:14:07.1361016Z configure: build.cargo-native-static := True
2020-03-30T17:14:07.1361629Z configure: dist.missing-tools   := True
2020-03-30T17:14:07.1362345Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-30T17:14:07.1363635Z configure: writing `config.toml` in current directory
2020-03-30T17:14:07.1363975Z configure: 
2020-03-30T17:14:07.1364526Z configure: run `python /checkout/x.py --help`
2020-03-30T17:14:07.1364892Z configure: 
---
2020-03-30T17:15:25.6229399Z Hugepagesize:       2048 kB
2020-03-30T17:15:25.6229630Z DirectMap4k:      128960 kB
2020-03-30T17:15:25.6229846Z DirectMap2M:     4065280 kB
2020-03-30T17:15:25.6230060Z DirectMap1G:     5242880 kB
2020-03-30T17:15:25.6269130Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-30T17:15:26.9361396Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-30T17:15:26.9361396Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-30T17:15:26.9373045Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-30T17:15:27.1692235Z    Compiling unicode-xid v0.2.0
2020-03-30T17:15:27.2871137Z    Compiling syn v1.0.11
2020-03-30T17:15:28.0853471Z    Compiling linked-hash-map v0.5.2
2020-03-30T17:15:28.1372567Z    Compiling lazy_static v1.4.0
2020-03-30T17:15:28.1372567Z    Compiling lazy_static v1.4.0
2020-03-30T17:15:28.3045702Z    Compiling yaml-rust v0.4.3
2020-03-30T17:15:32.4755355Z    Compiling quote v1.0.2
2020-03-30T17:15:46.0773171Z    Compiling thiserror-impl v1.0.5
2020-03-30T17:15:50.6048808Z    Compiling thiserror v1.0.5
2020-03-30T17:15:50.6626872Z    Compiling yaml-merge-keys v0.4.0
2020-03-30T17:15:51.7719912Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-30T17:15:53.4896495Z Build completed successfully in 0:00:27
2020-03-30T17:15:53.4907558Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-30T17:15:53.7274474Z     Finished dev [unoptimized] target(s) in 0.17s
2020-03-30T17:15:54.8087983Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-30T17:17:54.3278859Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T17:17:54.4165653Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T17:17:54.6041714Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T17:17:54.7121136Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T17:17:55.1855400Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T17:17:57.3782014Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T17:17:57.8343942Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T17:17:59.8353673Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T17:18:00.2721569Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T17:21:52.4639249Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-30T17:21:54.4026691Z Diff in /checkout/src/librustc_metadata/native_libs.rs at line 162:
2020-03-30T17:21:54.4027012Z              }
2020-03-30T17:21:54.4027176Z          }
2020-03-30T17:21:54.4027665Z          if lib.cfg.is_some() && !self.tcx.features().link_cfg {
2020-03-30T17:21:54.4029488Z -            feature_err(&self.tcx.sess.parse_sess, sym::link_cfg, span.unwrap(), "is unstable, see https://github.com/rust-lang/rfcs/pull/1721")
2020-03-30T17:21:54.4030185Z -                .emit();
2020-03-30T17:21:54.4030628Z +                &self.tcx.sess.parse_sess,
2020-03-30T17:21:54.4030894Z +                sym::link_cfg,
2020-03-30T17:21:54.4031128Z +                span.unwrap(),
2020-03-30T17:21:54.4031670Z +                "is unstable, see https://github.com/rust-lang/rfcs/pull/1721",
2020-03-30T17:21:54.4031670Z +                "is unstable, see https://github.com/rust-lang/rfcs/pull/1721",
2020-03-30T17:21:54.4031964Z +            )
2020-03-30T17:21:54.4032159Z +            .emit();
2020-03-30T17:21:54.4032326Z          }
2020-03-30T17:21:54.4032636Z          if lib.kind == cstore::NativeStaticNobundle && !self.tcx.features().static_nobundle {
2020-03-30T17:21:54.4032992Z              feature_err(
2020-03-30T17:21:54.4033907Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_metadata/native_libs.rs"` failed.
2020-03-30T17:21:54.4034872Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-30T17:21:54.4043517Z Build completed unsuccessfully in 0:00:36
2020-03-30T17:21:54.4096481Z == clock drift check ==
2020-03-30T17:21:54.4111252Z   local time: Mon Mar 30 17:21:54 UTC 2020
2020-03-30T17:21:54.5078833Z   network time: Mon, 30 Mar 2020 17:21:54 GMT
2020-03-30T17:21:54.5078833Z   network time: Mon, 30 Mar 2020 17:21:54 GMT
2020-03-30T17:21:54.5084345Z == end clock drift check ==
2020-03-30T17:21:56.0249534Z 
2020-03-30T17:21:56.0316797Z ##[error]Bash exited with code '1'.
2020-03-30T17:21:56.0332157Z ##[section]Finishing: Run build
2020-03-30T17:21:56.0377536Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70576/merge to s
2020-03-30T17:21:56.0382433Z Task         : Get sources
2020-03-30T17:21:56.0382788Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T17:21:56.0383106Z Version      : 1.0.0
2020-03-30T17:21:56.0383327Z Author       : Microsoft
2020-03-30T17:21:56.0383327Z Author       : Microsoft
2020-03-30T17:21:56.0383693Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-30T17:21:56.0384090Z ==============================================================================
2020-03-30T17:21:56.3650855Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-30T17:21:56.3697429Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70576/merge to s
2020-03-30T17:21:56.3783852Z Cleaning up task key
2020-03-30T17:21:56.3785121Z Start cleaning up orphan processes.
2020-03-30T17:21:56.3958681Z Terminate orphan process: pid (3450) (python)
2020-03-30T17:21:56.4094563Z ##[section]Finishing: Finalize Job
