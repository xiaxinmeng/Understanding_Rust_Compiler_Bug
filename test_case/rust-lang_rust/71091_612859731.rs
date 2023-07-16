plain
2020-04-13T10:51:28.2124697Z ========================== Starting Command Output ===========================
2020-04-13T10:51:28.2127277Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aff11ae5-9937-4f23-a438-c1afa590af65.sh
2020-04-13T10:51:28.2127547Z 
2020-04-13T10:51:28.2130823Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T10:51:28.2150218Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-13T10:51:28.2153671Z Task         : Get sources
2020-04-13T10:51:28.2153956Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T10:51:28.2154232Z Version      : 1.0.0
2020-04-13T10:51:28.2154554Z Author       : Microsoft
---
2020-04-13T10:51:29.2148717Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T10:51:29.2153582Z ##[command]git config gc.auto 0
2020-04-13T10:51:29.2157028Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T10:51:29.2160928Z ##[command]git config --get-all http.proxy
2020-04-13T10:51:29.2167137Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71091/merge:refs/remotes/pull/71091/merge
---
2020-04-13T10:54:08.5988920Z  ---> 78ad2f4d4aca
2020-04-13T10:54:08.5989441Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-13T10:54:08.5996624Z  ---> Using cache
2020-04-13T10:54:08.5997695Z  ---> 4d2dc61c4d00
2020-04-13T10:54:08.6000575Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-13T10:54:08.6002752Z  ---> 776b6266a8b7
2020-04-13T10:54:08.6091886Z Successfully built 776b6266a8b7
2020-04-13T10:54:08.6121602Z Successfully tagged rust-ci:latest
2020-04-13T10:54:08.6434835Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T10:54:08.6434835Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T10:54:08.6452104Z Looks like docker image is the same as before, not uploading
2020-04-13T10:54:09.3079822Z [CI_JOB_NAME=mingw-check]
2020-04-13T10:54:09.3386407Z [CI_JOB_NAME=mingw-check]
2020-04-13T10:54:09.3421265Z == clock drift check ==
2020-04-13T10:54:09.3431800Z   local time: Mon Apr 13 10:54:09 UTC 2020
2020-04-13T10:54:09.6248802Z   network time: Mon, 13 Apr 2020 10:54:09 GMT
2020-04-13T10:54:09.6273837Z Starting sccache server...
2020-04-13T10:54:09.7491855Z configure: processing command line
2020-04-13T10:54:09.7492198Z configure: 
2020-04-13T10:54:09.7493214Z configure: rust.parallel-compiler := True
---
2020-04-13T10:58:07.8955041Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T10:58:08.0207689Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T10:58:08.2281785Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T10:58:08.3321516Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T10:58:08.9057325Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T10:58:11.4547159Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T10:58:11.9724018Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T10:58:14.2151306Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T10:58:14.6861179Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T11:00:08.4149673Z configure: rust.channel         := nightly
2020-04-13T11:00:08.4149998Z configure: llvm.assertions      := True
2020-04-13T11:00:08.4150524Z configure: dist.missing-tools   := True
2020-04-13T11:00:08.4151028Z configure: rust.dist-src        := False
2020-04-13T11:00:08.4151959Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-13T11:00:08.4152597Z configure: writing `config.toml` in current directory
2020-04-13T11:00:08.4152862Z configure: 
2020-04-13T11:00:08.4153324Z configure: run `python /checkout/x.py --help`
2020-04-13T11:00:08.4153576Z configure: 
---
2020-04-13T11:01:46.0539061Z Hugepagesize:       2048 kB
2020-04-13T11:01:46.0539309Z DirectMap4k:      131008 kB
2020-04-13T11:01:46.0539626Z DirectMap2M:     3014656 kB
2020-04-13T11:01:46.0539880Z DirectMap1G:     6291456 kB
2020-04-13T11:01:46.0558991Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-13T11:01:47.5130669Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-13T11:01:47.5130669Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-13T11:01:47.5138970Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-13T11:01:47.7744335Z    Compiling unicode-xid v0.2.0
2020-04-13T11:01:47.9116196Z    Compiling syn v1.0.11
2020-04-13T11:01:48.8004975Z    Compiling linked-hash-map v0.5.2
2020-04-13T11:01:48.8700258Z    Compiling lazy_static v1.4.0
2020-04-13T11:01:48.8700258Z    Compiling lazy_static v1.4.0
2020-04-13T11:01:49.0498171Z    Compiling yaml-rust v0.4.3
2020-04-13T11:01:53.6752718Z    Compiling quote v1.0.2
2020-04-13T11:02:09.4543427Z    Compiling thiserror-impl v1.0.5
2020-04-13T11:02:14.6364679Z    Compiling thiserror v1.0.5
2020-04-13T11:02:14.6967956Z    Compiling yaml-merge-keys v0.4.0
2020-04-13T11:02:16.0072536Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-13T11:02:18.3224805Z Build completed successfully in 0:00:32
2020-04-13T11:02:18.3347270Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-13T11:02:18.6399859Z     Finished dev [unoptimized] target(s) in 0.20s
2020-04-13T11:02:19.8980882Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-13T11:04:36.1676745Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T11:04:36.2882828Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T11:04:36.4933458Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T11:04:36.6266854Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T11:04:37.1541243Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T11:04:39.5495744Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T11:04:40.0544361Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T11:04:42.3057121Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T11:04:42.7961325Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T11:09:03.3728861Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-13T11:09:03.3736399Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-13T11:09:03.3738132Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-13T11:09:03.3742384Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-13T11:09:06.0707150Z Diff in /checkout/src/libstd/io/util.rs at line 4:
2020-04-13T11:09:06.0712680Z  use crate::io::{self, BufRead, Initializer, IoSlice, IoSliceMut, Read, Write};
2020-04-13T11:09:06.0722466Z  mod copy_specialization {
2020-04-13T11:09:06.0722466Z  mod copy_specialization {
2020-04-13T11:09:06.0727359Z +    use crate::io::{self, BufRead, ErrorKind, Read, Write};
2020-04-13T11:09:06.0760347Z -    use crate::io::{self, Read, BufRead, Write, ErrorKind};
2020-04-13T11:09:06.0760609Z  
2020-04-13T11:09:06.0760777Z      pub trait Copyable {
2020-04-13T11:09:06.0760777Z      pub trait Copyable {
2020-04-13T11:09:06.0761385Z          fn copy_to<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<u64>;
2020-04-13T11:09:06.0762529Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/io/util.rs"` failed.
2020-04-13T11:09:06.0766028Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-13T11:09:06.0767308Z Build completed unsuccessfully in 0:00:41
2020-04-13T11:09:06.0864009Z == clock drift check ==
2020-04-13T11:09:06.0879334Z   local time: Mon Apr 13 11:09:06 UTC 2020
2020-04-13T11:09:06.0879334Z   local time: Mon Apr 13 11:09:06 UTC 2020
2020-04-13T11:09:06.2740461Z   network time: Mon, 13 Apr 2020 11:09:06 GMT
2020-04-13T11:09:07.9883568Z 
2020-04-13T11:09:07.9883568Z 
2020-04-13T11:09:07.9998991Z ##[error]Bash exited with code '1'.
2020-04-13T11:09:08.0019026Z ##[section]Finishing: Run build
2020-04-13T11:09:08.0093241Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-13T11:09:08.0098416Z Task         : Get sources
2020-04-13T11:09:08.0098743Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T11:09:08.0099042Z Version      : 1.0.0
2020-04-13T11:09:08.0099267Z Author       : Microsoft
2020-04-13T11:09:08.0099267Z Author       : Microsoft
2020-04-13T11:09:08.0099761Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T11:09:08.0100146Z ==============================================================================
2020-04-13T11:09:08.3945180Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T11:09:08.4041152Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-13T11:09:08.4128914Z Cleaning up task key
2020-04-13T11:09:08.4130139Z Start cleaning up orphan processes.
2020-04-13T11:09:08.4348441Z Terminate orphan process: pid (3477) (python)
2020-04-13T11:09:08.4529810Z ##[section]Finishing: Finalize Job
