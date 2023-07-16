plain
2020-04-24T19:59:52.7083858Z ========================== Starting Command Output ===========================
2020-04-24T19:59:52.7086672Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e3eee844-e74e-4ce1-92d4-3e863ad0c32e.sh
2020-04-24T19:59:52.7087018Z 
2020-04-24T19:59:52.7090339Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T19:59:52.7106983Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71529/merge to s
2020-04-24T19:59:52.7109835Z Task         : Get sources
2020-04-24T19:59:52.7110092Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T19:59:52.7110347Z Version      : 1.0.0
2020-04-24T19:59:52.7110522Z Author       : Microsoft
---
2020-04-24T19:59:53.7102356Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T19:59:53.7110014Z ##[command]git config gc.auto 0
2020-04-24T19:59:53.7112789Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T19:59:53.7115402Z ##[command]git config --get-all http.proxy
2020-04-24T19:59:53.7120197Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71529/merge:refs/remotes/pull/71529/merge
---
2020-04-24T20:03:00.2924464Z  ---> f7353ccad5b1
2020-04-24T20:03:00.2924747Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T20:03:00.2925155Z  ---> Using cache
2020-04-24T20:03:00.2925540Z  ---> ed38efbaa060
2020-04-24T20:03:00.2926667Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T20:03:00.2927889Z  ---> c5008ef7ae8e
2020-04-24T20:03:00.2944723Z Successfully built c5008ef7ae8e
2020-04-24T20:03:00.2962955Z Successfully tagged rust-ci:latest
2020-04-24T20:03:00.3221913Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T20:03:00.3221913Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T20:03:00.3232327Z Looks like docker image is the same as before, not uploading
2020-04-24T20:03:01.0581355Z [CI_JOB_NAME=mingw-check]
2020-04-24T20:03:01.0792453Z [CI_JOB_NAME=mingw-check]
2020-04-24T20:03:01.0822883Z == clock drift check ==
2020-04-24T20:03:01.0835058Z   local time: Fri Apr 24 20:03:01 UTC 2020
2020-04-24T20:03:01.3741851Z   network time: Fri, 24 Apr 2020 20:03:01 GMT
2020-04-24T20:03:01.3775016Z Starting sccache server...
2020-04-24T20:03:01.4845063Z configure: processing command line
2020-04-24T20:03:01.4846021Z configure: 
2020-04-24T20:03:01.4847092Z configure: rust.parallel-compiler := True
---
2020-04-24T20:06:29.5232384Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T20:06:29.5575602Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T20:06:29.7408479Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T20:06:29.9016116Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T20:06:30.2964995Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T20:06:32.3937986Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T20:06:32.8188009Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T20:06:34.6731420Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T20:06:35.0969159Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T20:08:13.9077458Z configure: llvm.assertions      := True
2020-04-24T20:08:13.9077867Z configure: build.locked-deps    := True
2020-04-24T20:08:13.9078273Z configure: build.cargo-native-static := True
2020-04-24T20:08:13.9078529Z configure: llvm.ccache          := sccache
2020-04-24T20:08:13.9079049Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-24T20:08:13.9079520Z configure: writing `config.toml` in current directory
2020-04-24T20:08:13.9079734Z configure: 
2020-04-24T20:08:13.9080085Z configure: run `python /checkout/x.py --help`
2020-04-24T20:08:13.9080276Z configure: 
---
2020-04-24T20:09:41.5985273Z Hugepagesize:       2048 kB
2020-04-24T20:09:41.5985461Z DirectMap4k:      147392 kB
2020-04-24T20:09:41.5985648Z DirectMap2M:     4046848 kB
2020-04-24T20:09:41.5986026Z DirectMap1G:     5242880 kB
2020-04-24T20:09:41.5987275Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-24T20:09:42.9134320Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T20:09:42.9134320Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T20:09:42.9147290Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-24T20:09:43.1325117Z    Compiling unicode-xid v0.2.0
2020-04-24T20:09:43.2430285Z    Compiling syn v1.0.11
2020-04-24T20:09:44.0108358Z    Compiling linked-hash-map v0.5.2
2020-04-24T20:09:44.0533927Z    Compiling lazy_static v1.4.0
2020-04-24T20:09:44.0533927Z    Compiling lazy_static v1.4.0
2020-04-24T20:09:44.2224610Z    Compiling yaml-rust v0.4.3
2020-04-24T20:09:48.1263312Z    Compiling quote v1.0.2
2020-04-24T20:10:02.3906213Z    Compiling thiserror-impl v1.0.5
2020-04-24T20:10:06.1524861Z    Compiling thiserror v1.0.5
2020-04-24T20:10:06.2173273Z    Compiling yaml-merge-keys v0.4.0
2020-04-24T20:10:07.3364533Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-24T20:10:09.8245481Z Build completed successfully in 0:00:28
2020-04-24T20:10:09.8341516Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-24T20:10:10.0944583Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-24T20:10:11.6641877Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-24T20:12:06.1067877Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T20:12:06.3447306Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T20:12:06.4873348Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T20:12:06.5392009Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T20:12:07.0628367Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T20:12:09.1648600Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T20:12:09.6079388Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T20:12:11.4932675Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T20:12:11.8940832Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T20:15:47.9914648Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-24T20:15:47.9919692Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-24T20:15:47.9924295Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-24T20:15:47.9930283Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-24T20:15:52.2101314Z Diff in /checkout/src/libstd/sync/mod.rs at line 167:
2020-04-24T20:15:52.2102222Z  pub use self::once::{Once, OnceState, ONCE_INIT};
2020-04-24T20:15:52.2103634Z  #[stable(feature = "rust1", since = "1.0.0")]
2020-04-24T20:15:52.2104648Z  pub use self::rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
2020-04-24T20:15:52.2106154Z -#[stable(feature = "rust1", since = "1.0.0")]
2020-04-24T20:15:52.2106844Z -pub use crate::sys_common::poison::{LockResult, PoisonError, TryLockError, TryLockResult};
2020-04-24T20:15:52.2107311Z  #[unstable(feature = "static_mutex", issue = "71521")]
2020-04-24T20:15:52.2107671Z  pub use self::static_mutex::StaticMutex;
2020-04-24T20:15:52.2108010Z +#[stable(feature = "rust1", since = "1.0.0")]
2020-04-24T20:15:52.2108435Z +pub use crate::sys_common::poison::{LockResult, PoisonError, TryLockError, TryLockResult};
2020-04-24T20:15:52.2109017Z  pub mod mpsc;
2020-04-24T20:15:52.2109255Z  
2020-04-24T20:15:52.2109255Z  
2020-04-24T20:15:52.2110175Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/sync/mod.rs"` failed.
2020-04-24T20:15:52.2111118Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-24T20:15:52.2117236Z Build completed unsuccessfully in 0:00:36
2020-04-24T20:15:52.2224603Z == clock drift check ==
2020-04-24T20:15:52.2224603Z == clock drift check ==
2020-04-24T20:15:52.2238658Z   local time: Fri Apr 24 20:15:52 UTC 2020
2020-04-24T20:15:52.3202138Z   network time: Fri, 24 Apr 2020 20:15:52 GMT
2020-04-24T20:15:54.1847145Z 
2020-04-24T20:15:54.1847145Z 
2020-04-24T20:15:54.1911024Z ##[error]Bash exited with code '1'.
2020-04-24T20:15:54.1924610Z ##[section]Finishing: Run build
2020-04-24T20:15:54.1965947Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71529/merge to s
2020-04-24T20:15:54.1972353Z Task         : Get sources
2020-04-24T20:15:54.1972667Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T20:15:54.1972970Z Version      : 1.0.0
2020-04-24T20:15:54.1973183Z Author       : Microsoft
2020-04-24T20:15:54.1973183Z Author       : Microsoft
2020-04-24T20:15:54.1973508Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T20:15:54.1973887Z ==============================================================================
2020-04-24T20:15:54.5012291Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T20:15:54.5068279Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71529/merge to s
2020-04-24T20:15:54.5151130Z Cleaning up task key
2020-04-24T20:15:54.5152377Z Start cleaning up orphan processes.
2020-04-24T20:15:54.5326235Z Terminate orphan process: pid (3554) (python)
2020-04-24T20:15:54.5470823Z ##[section]Finishing: Finalize Job
