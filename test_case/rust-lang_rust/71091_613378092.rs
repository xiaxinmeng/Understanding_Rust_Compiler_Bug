plain
2020-04-14T10:44:28.8898791Z ========================== Starting Command Output ===========================
2020-04-14T10:44:28.8902261Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b3c9809d-8689-4265-9c65-ef8374930783.sh
2020-04-14T10:44:28.8902750Z 
2020-04-14T10:44:28.8910500Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T10:44:28.8927522Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-14T10:44:28.8930122Z Task         : Get sources
2020-04-14T10:44:28.8930331Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T10:44:28.8930535Z Version      : 1.0.0
2020-04-14T10:44:28.8930719Z Author       : Microsoft
---
2020-04-14T10:44:30.1287918Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T10:44:30.1296357Z ##[command]git config gc.auto 0
2020-04-14T10:44:30.1301718Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T10:44:30.1309466Z ##[command]git config --get-all http.proxy
2020-04-14T10:44:30.1320550Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71091/merge:refs/remotes/pull/71091/merge
---
2020-04-14T10:46:28.1798451Z  ---> 78ad2f4d4aca
2020-04-14T10:46:28.1798629Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-14T10:46:28.1803095Z  ---> Using cache
2020-04-14T10:46:28.1803583Z  ---> 4d2dc61c4d00
2020-04-14T10:46:28.1804558Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-14T10:46:28.1811186Z  ---> 776b6266a8b7
2020-04-14T10:46:28.1846306Z Successfully built 776b6266a8b7
2020-04-14T10:46:28.1885586Z Successfully tagged rust-ci:latest
2020-04-14T10:46:28.2121638Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T10:46:28.2121638Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T10:46:28.2140422Z Looks like docker image is the same as before, not uploading
2020-04-14T10:46:36.2182177Z [CI_JOB_NAME=mingw-check]
2020-04-14T10:46:36.2396950Z [CI_JOB_NAME=mingw-check]
2020-04-14T10:46:36.2430497Z == clock drift check ==
2020-04-14T10:46:36.2441926Z   local time: Tue Apr 14 10:46:36 UTC 2020
2020-04-14T10:46:36.5384002Z   network time: Tue, 14 Apr 2020 10:46:36 GMT
2020-04-14T10:46:36.5407218Z Starting sccache server...
2020-04-14T10:46:36.6426780Z configure: processing command line
2020-04-14T10:46:36.6427702Z configure: 
2020-04-14T10:46:36.6429585Z configure: rust.parallel-compiler := True
---
2020-04-14T10:50:07.5159778Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-14T10:50:11.7054547Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-14T10:50:12.8755044Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T10:50:12.9206781Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T10:50:13.1163144Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T10:50:13.8751224Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T10:50:13.9181797Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-14T10:50:15.3050096Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T10:50:15.7967100Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-14T10:52:03.3361000Z configure: build.submodules     := False
2020-04-14T10:52:03.3361226Z configure: llvm.ccache          := sccache
2020-04-14T10:52:03.3361476Z configure: rust.channel         := nightly
2020-04-14T10:52:03.3361842Z configure: build.locked-deps    := True
2020-04-14T10:52:03.3362301Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-14T10:52:03.3362744Z configure: writing `config.toml` in current directory
2020-04-14T10:52:03.3362928Z configure: 
2020-04-14T10:52:03.3363264Z configure: run `python /checkout/x.py --help`
2020-04-14T10:52:03.3363439Z configure: 
---
2020-04-14T10:53:31.6610120Z Hugepagesize:       2048 kB
2020-04-14T10:53:31.6610291Z DirectMap4k:      112576 kB
2020-04-14T10:53:31.6610477Z DirectMap2M:     4081664 kB
2020-04-14T10:53:31.6610638Z DirectMap1G:     5242880 kB
2020-04-14T10:53:31.6611327Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-14T10:53:32.9699904Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-14T10:53:32.9699904Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-14T10:53:32.9708565Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-14T10:53:33.2080750Z    Compiling unicode-xid v0.2.0
2020-04-14T10:53:33.3395030Z    Compiling syn v1.0.11
2020-04-14T10:53:34.1598701Z    Compiling linked-hash-map v0.5.2
2020-04-14T10:53:34.2004375Z    Compiling lazy_static v1.4.0
2020-04-14T10:53:34.2004375Z    Compiling lazy_static v1.4.0
2020-04-14T10:53:34.3889293Z    Compiling yaml-rust v0.4.3
2020-04-14T10:53:38.5604503Z    Compiling quote v1.0.2
2020-04-14T10:53:52.5411873Z    Compiling thiserror-impl v1.0.5
2020-04-14T10:53:57.1627433Z    Compiling thiserror v1.0.5
2020-04-14T10:53:57.2161480Z    Compiling yaml-merge-keys v0.4.0
2020-04-14T10:53:58.3115598Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-14T10:54:01.4416371Z Build completed successfully in 0:00:29
2020-04-14T10:54:01.4553876Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-14T10:54:01.7485048Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-14T10:54:02.7813311Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-14T10:56:02.5786909Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-14T10:56:07.1386784Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-14T10:56:08.3965772Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T10:56:08.4047175Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T10:56:08.5911209Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T10:56:09.3091517Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T10:56:09.4253236Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-14T10:56:10.9291017Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T10:56:11.4335313Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-14T11:00:15.1822374Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-14T11:00:15.1825925Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-14T11:00:15.1826915Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-14T11:00:15.1830467Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-14T11:00:19.5767187Z Diff in /checkout/src/libstd/io/util.rs at line 4:
2020-04-14T11:00:19.5772962Z  use crate::io::{self, BufRead, Initializer, IoSlice, IoSliceMut, Read, Write};
2020-04-14T11:00:19.5783913Z  mod copy_specialization {
2020-04-14T11:00:19.5797423Z -    use crate::mem::MaybeUninit;
2020-04-14T11:00:19.5797423Z -    use crate::mem::MaybeUninit;
2020-04-14T11:00:19.5810626Z      use crate::io::{self, BufRead, ErrorKind, Read, Write};
2020-04-14T11:00:19.5811818Z  
2020-04-14T11:00:19.5811979Z      pub trait Copyable {
2020-04-14T11:00:19.5811979Z      pub trait Copyable {
2020-04-14T11:00:19.5812745Z          fn copy_to<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<u64>;
2020-04-14T11:00:19.5831064Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/io/util.rs"` failed.
2020-04-14T11:00:19.5832010Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-14T11:00:19.5839480Z Build completed unsuccessfully in 0:00:40
2020-04-14T11:00:19.5954740Z == clock drift check ==
2020-04-14T11:00:19.5954740Z == clock drift check ==
2020-04-14T11:00:19.5973328Z   local time: Tue Apr 14 11:00:19 UTC 2020
2020-04-14T11:00:19.7045756Z   network time: Tue, 14 Apr 2020 11:00:19 GMT
2020-04-14T11:00:21.1966558Z 
2020-04-14T11:00:21.1966558Z 
2020-04-14T11:00:21.2041353Z ##[error]Bash exited with code '1'.
2020-04-14T11:00:21.2055441Z ##[section]Finishing: Run build
2020-04-14T11:00:21.2098462Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-14T11:00:21.2103183Z Task         : Get sources
2020-04-14T11:00:21.2103460Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T11:00:21.2103695Z Version      : 1.0.0
2020-04-14T11:00:21.2103863Z Author       : Microsoft
2020-04-14T11:00:21.2103863Z Author       : Microsoft
2020-04-14T11:00:21.2104146Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T11:00:21.2104447Z ==============================================================================
2020-04-14T11:00:21.5465527Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T11:00:21.5512407Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71091/merge to s
2020-04-14T11:00:21.5613824Z Cleaning up task key
2020-04-14T11:00:21.5615108Z Start cleaning up orphan processes.
2020-04-14T11:00:21.5808196Z Terminate orphan process: pid (5799) (python)
2020-04-14T11:00:21.6024433Z ##[section]Finishing: Finalize Job
