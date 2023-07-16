plain
2020-04-19T16:22:42.1960283Z ========================== Starting Command Output ===========================
2020-04-19T16:22:42.1965524Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fc5d1123-f478-42f5-a255-9b3dc0d0032d.sh
2020-04-19T16:22:42.1966050Z 
2020-04-19T16:22:42.1970846Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T16:22:42.1988752Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71308/merge to s
2020-04-19T16:22:42.1992140Z Task         : Get sources
2020-04-19T16:22:42.1992422Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T16:22:42.1992851Z Version      : 1.0.0
2020-04-19T16:22:42.1993036Z Author       : Microsoft
---
2020-04-19T16:22:43.8818499Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T16:22:43.8828374Z ##[command]git config gc.auto 0
2020-04-19T16:22:43.8833269Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T16:22:43.8837359Z ##[command]git config --get-all http.proxy
2020-04-19T16:22:43.8847688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71308/merge:refs/remotes/pull/71308/merge
---
2020-04-19T16:26:02.9723380Z  ---> 78ad2f4d4aca
2020-04-19T16:26:02.9723632Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-19T16:26:02.9724034Z  ---> Using cache
2020-04-19T16:26:02.9724390Z  ---> 4d2dc61c4d00
2020-04-19T16:26:02.9726038Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-19T16:26:02.9727350Z  ---> 776b6266a8b7
2020-04-19T16:26:02.9771843Z Successfully built 776b6266a8b7
2020-04-19T16:26:02.9803525Z Successfully tagged rust-ci:latest
2020-04-19T16:26:03.1256298Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-19T16:26:03.1256298Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-19T16:26:03.1273322Z Looks like docker image is the same as before, not uploading
2020-04-19T16:26:10.3096623Z [CI_JOB_NAME=mingw-check]
2020-04-19T16:26:10.3375696Z [CI_JOB_NAME=mingw-check]
2020-04-19T16:26:10.3407000Z == clock drift check ==
2020-04-19T16:26:10.3415474Z   local time: Sun Apr 19 16:26:10 UTC 2020
2020-04-19T16:26:10.6553680Z   network time: Sun, 19 Apr 2020 16:26:10 GMT
2020-04-19T16:26:10.6581034Z Starting sccache server...
2020-04-19T16:26:10.7733449Z configure: processing command line
2020-04-19T16:26:10.7737516Z configure: 
2020-04-19T16:26:10.7738509Z configure: rust.parallel-compiler := True
---
2020-04-19T16:30:18.9968830Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T16:30:18.9970484Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T16:30:18.9971307Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T16:30:18.9972143Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-19T16:30:20.6476854Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T16:30:22.1720804Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T16:30:24.4151095Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T16:30:24.9063262Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-19T16:30:26.2124702Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-04-19T16:32:19.4333702Z configure: build.submodules     := False
2020-04-19T16:32:19.4333925Z configure: llvm.ccache          := sccache
2020-04-19T16:32:19.4334323Z configure: rust.dist-src        := False
2020-04-19T16:32:19.4334713Z configure: build.cargo-native-static := True
2020-04-19T16:32:19.4335190Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-19T16:32:19.4335631Z configure: writing `config.toml` in current directory
2020-04-19T16:32:19.4335819Z configure: 
2020-04-19T16:32:19.4336169Z configure: run `python /checkout/x.py --help`
2020-04-19T16:32:19.4336347Z configure: 
---
2020-04-19T16:34:01.7665372Z Hugepagesize:       2048 kB
2020-04-19T16:34:01.7665598Z DirectMap4k:      139200 kB
2020-04-19T16:34:01.7665841Z DirectMap2M:     3006464 kB
2020-04-19T16:34:01.7666066Z DirectMap1G:     6291456 kB
2020-04-19T16:34:01.7691992Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-19T16:34:03.2002480Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-19T16:34:03.2002480Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-19T16:34:03.2010134Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-19T16:34:03.4608984Z    Compiling unicode-xid v0.2.0
2020-04-19T16:34:03.6050923Z    Compiling syn v1.0.11
2020-04-19T16:34:04.4732512Z    Compiling linked-hash-map v0.5.2
2020-04-19T16:34:04.4789705Z    Compiling lazy_static v1.4.0
2020-04-19T16:34:04.4789705Z    Compiling lazy_static v1.4.0
2020-04-19T16:34:04.7392236Z    Compiling yaml-rust v0.4.3
2020-04-19T16:34:09.3426356Z    Compiling quote v1.0.2
2020-04-19T16:34:24.7756185Z    Compiling thiserror-impl v1.0.5
2020-04-19T16:34:29.8032243Z    Compiling thiserror v1.0.5
2020-04-19T16:34:29.8594610Z    Compiling yaml-merge-keys v0.4.0
2020-04-19T16:34:33.7093548Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-19T16:34:35.3618569Z Build completed successfully in 0:00:33
2020-04-19T16:34:35.3700260Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-19T16:34:35.6210309Z     Finished dev [unoptimized] target(s) in 0.16s
2020-04-19T16:34:36.7940366Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-19T16:36:51.6124167Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T16:36:51.6392790Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T16:36:51.8504106Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T16:36:52.0584832Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T16:36:52.4978993Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T16:36:54.9003143Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T16:36:55.4320589Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T16:36:57.8235540Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T16:36:58.3321835Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T16:41:16.8626626Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-19T16:41:16.8630435Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-19T16:41:16.8631212Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-19T16:41:16.8635210Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-19T16:41:24.3645123Z Diff in /checkout/src/libstd/sys/cloudabi/shims/fs.rs at line 140:
2020-04-19T16:41:24.3648560Z      fn next(&mut self) -> Option<io::Result<DirEntry>> {
2020-04-19T16:41:24.3648858Z          match self.0 {}
2020-04-19T16:41:24.3649363Z -
2020-04-19T16:41:24.3649494Z  }
2020-04-19T16:41:24.3649616Z  
2020-04-19T16:41:24.3649795Z  impl DirEntry {
2020-04-19T16:41:24.3649795Z  impl DirEntry {
2020-04-19T16:41:24.3650772Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/sys/cloudabi/shims/fs.rs"` failed.
2020-04-19T16:41:24.3651765Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-19T16:41:24.3677811Z Build completed unsuccessfully in 0:00:44
2020-04-19T16:41:24.3777994Z == clock drift check ==
2020-04-19T16:41:24.3841415Z   local time: Sun Apr 19 16:41:24 UTC 2020
2020-04-19T16:41:24.3841415Z   local time: Sun Apr 19 16:41:24 UTC 2020
2020-04-19T16:41:24.9745145Z   network time: Sun, 19 Apr 2020 16:41:24 GMT
2020-04-19T16:41:26.6094464Z 
2020-04-19T16:41:26.6094464Z 
2020-04-19T16:41:26.6192145Z ##[error]Bash exited with code '1'.
2020-04-19T16:41:26.6218927Z ##[section]Finishing: Run build
2020-04-19T16:41:26.6271351Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71308/merge to s
2020-04-19T16:41:26.6277086Z Task         : Get sources
2020-04-19T16:41:26.6277440Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T16:41:26.6277766Z Version      : 1.0.0
2020-04-19T16:41:26.6278021Z Author       : Microsoft
2020-04-19T16:41:26.6278021Z Author       : Microsoft
2020-04-19T16:41:26.6278388Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T16:41:26.6279600Z ==============================================================================
2020-04-19T16:41:26.9959804Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T16:41:27.0006557Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71308/merge to s
2020-04-19T16:41:27.0106425Z Cleaning up task key
2020-04-19T16:41:27.0107931Z Start cleaning up orphan processes.
2020-04-19T16:41:27.0338618Z Terminate orphan process: pid (3642) (python)
2020-04-19T16:41:27.0515124Z ##[section]Finishing: Finalize Job
