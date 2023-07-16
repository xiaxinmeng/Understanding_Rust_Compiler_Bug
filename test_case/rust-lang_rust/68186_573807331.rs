plain
2020-01-13T18:29:30.0821730Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-13T18:29:30.0914665Z ##[command]git config gc.auto 0
2020-01-13T18:29:30.0987293Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-13T18:29:30.1051339Z ##[command]git config --get-all http.proxy
2020-01-13T18:29:30.1207209Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68186/merge:refs/remotes/pull/68186/merge
---
2020-01-13T18:34:27.7949247Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-13T18:34:27.9756482Z     Checking backtrace v0.3.40
2020-01-13T18:34:29.4544818Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-01-13T18:34:29.4547743Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-13T18:34:31.2790877Z error[E0119]: conflicting implementations of trait `core::convert::From<alloc_crate::boxed::Box<dyn error::Error>>` for type `alloc_crate::boxed::Box<dyn error::Error>`:
2020-01-13T18:34:31.2791943Z    --> src/libstd/error.rs:163:1
2020-01-13T18:34:31.2792501Z     |
2020-01-13T18:34:31.2793018Z 163 | impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {
2020-01-13T18:34:31.2793993Z     |
2020-01-13T18:34:31.2794928Z     = note: conflicting implementation in crate `core`:
2020-01-13T18:34:31.2794928Z     = note: conflicting implementation in crate `core`:
2020-01-13T18:34:31.2795748Z             - impl<T> core::convert::From<T> for T;
2020-01-13T18:34:31.2796058Z 
2020-01-13T18:34:31.2800635Z error[E0119]: conflicting implementations of trait `core::convert::From<alloc_crate::boxed::Box<dyn error::Error + core::marker::Send + core::marker::Sync>>` for type `alloc_crate::boxed::Box<dyn error::Error + core::marker::Send + core::marker::Sync>`:
2020-01-13T18:34:31.2801373Z    --> src/libstd/error.rs:197:1
2020-01-13T18:34:31.2801839Z     |
2020-01-13T18:34:31.2802375Z 197 | impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<dyn Error + Send + Sync + 'a> {
2020-01-13T18:34:31.2803357Z     |
2020-01-13T18:34:31.2803840Z     = note: conflicting implementation in crate `core`:
2020-01-13T18:34:31.2803840Z     = note: conflicting implementation in crate `core`:
2020-01-13T18:34:31.2804940Z             - impl<T> core::convert::From<T> for T;
2020-01-13T18:34:31.3196127Z error: aborting due to 2 previous errors
2020-01-13T18:34:31.8590895Z 
2020-01-13T18:34:31.8592407Z For more information about this error, try `rustc --explain E0119`.
2020-01-13T18:34:31.8592986Z error: could not compile `std`.
---
2020-01-13T18:34:31.8596409Z   local time: Mon Jan 13 18:34:31 UTC 2020
2020-01-13T18:34:31.8596555Z   network time: Mon, 13 Jan 2020 18:34:31 GMT
2020-01-13T18:34:31.8596701Z == end clock drift check ==
2020-01-13T18:34:32.5915947Z 
2020-01-13T18:34:32.6017337Z ##[error]Bash exited with code '1'.
2020-01-13T18:34:32.6046492Z ##[section]Starting: Checkout
2020-01-13T18:34:32.6048861Z ==============================================================================
2020-01-13T18:34:32.6048913Z Task         : Get sources
2020-01-13T18:34:32.6048961Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
