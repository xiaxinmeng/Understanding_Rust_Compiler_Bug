plain
2020-02-15T21:18:35.3348435Z ========================== Starting Command Output ===========================
2020-02-15T21:18:35.3350600Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0de489e1-6726-41bf-9cff-1d7cdbe35553.sh
2020-02-15T21:18:35.3350652Z 
2020-02-15T21:18:35.3355454Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T21:18:35.3362531Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69196/merge to s
2020-02-15T21:18:35.3364524Z Task         : Get sources
2020-02-15T21:18:35.3364563Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T21:18:35.3364600Z Version      : 1.0.0
2020-02-15T21:18:35.3364696Z Author       : Microsoft
---
2020-02-15T21:18:36.3645816Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T21:18:36.3657690Z ##[command]git config gc.auto 0
2020-02-15T21:18:36.3660505Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T21:18:36.3663561Z ##[command]git config --get-all http.proxy
2020-02-15T21:18:36.3671004Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69196/merge:refs/remotes/pull/69196/merge
---
2020-02-15T21:23:30.8523763Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-02-15T21:23:31.0078455Z     Checking backtrace v0.3.44
2020-02-15T21:23:31.3345223Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-02-15T21:23:31.3376312Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-02-15T21:23:32.8277179Z error[E0119]: conflicting implementations of trait `core::convert::From<alloc_crate::boxed::Box<dyn error::Error>>` for type `alloc_crate::boxed::Box<dyn error::Error>`:
2020-02-15T21:23:32.8278407Z    --> src/libstd/error.rs:163:1
2020-02-15T21:23:32.8279076Z     |
2020-02-15T21:23:32.8279797Z 163 | impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {
2020-02-15T21:23:32.8281224Z     |
2020-02-15T21:23:32.8281861Z     = note: conflicting implementation in crate `core`:
2020-02-15T21:23:32.8281861Z     = note: conflicting implementation in crate `core`:
2020-02-15T21:23:32.8282474Z             - impl<T> core::convert::From<T> for T;
2020-02-15T21:23:32.8282755Z 
2020-02-15T21:23:32.8288641Z error[E0119]: conflicting implementations of trait `core::convert::From<alloc_crate::boxed::Box<dyn error::Error + core::marker::Send + core::marker::Sync>>` for type `alloc_crate::boxed::Box<dyn error::Error + core::marker::Send + core::marker::Sync>`:
2020-02-15T21:23:32.8289380Z    --> src/libstd/error.rs:197:1
2020-02-15T21:23:32.8290201Z     |
2020-02-15T21:23:32.8291024Z 197 | impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<dyn Error + Send + Sync + 'a> {
2020-02-15T21:23:32.8292353Z     |
2020-02-15T21:23:32.8292996Z     = note: conflicting implementation in crate `core`:
2020-02-15T21:23:32.8292996Z     = note: conflicting implementation in crate `core`:
2020-02-15T21:23:32.8293804Z             - impl<T> core::convert::From<T> for T;
2020-02-15T21:23:32.8499393Z error: aborting due to 2 previous errors
2020-02-15T21:23:32.8500107Z 
2020-02-15T21:23:32.8500702Z For more information about this error, try `rustc --explain E0119`.
2020-02-15T21:23:32.8615699Z error: could not compile `std`.
---
2020-02-15T21:23:32.8726734Z   local time: Sat Feb 15 21:23:32 UTC 2020
2020-02-15T21:23:33.1615123Z   network time: Sat, 15 Feb 2020 21:23:33 GMT
2020-02-15T21:23:33.1618295Z == end clock drift check ==
2020-02-15T21:23:34.1715626Z 
2020-02-15T21:23:34.1829585Z ##[error]Bash exited with code '1'.
2020-02-15T21:23:34.1852547Z ##[section]Finishing: Run build
2020-02-15T21:23:34.1868804Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69196/merge to s
2020-02-15T21:23:34.1870952Z Task         : Get sources
2020-02-15T21:23:34.1870996Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T21:23:34.1871084Z Version      : 1.0.0
2020-02-15T21:23:34.1871124Z Author       : Microsoft
2020-02-15T21:23:34.1871124Z Author       : Microsoft
2020-02-15T21:23:34.1871165Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T21:23:34.1871234Z ==============================================================================
2020-02-15T21:23:34.6122880Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T21:23:34.6172538Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69196/merge to s
2020-02-15T21:23:34.6276260Z Cleaning up task key
2020-02-15T21:23:34.6277072Z Start cleaning up orphan processes.
2020-02-15T21:23:34.6379981Z Terminate orphan process: pid (5498) (python)
2020-02-15T21:23:34.6550316Z ##[section]Finishing: Finalize Job
