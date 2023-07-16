plain
2020-02-15T21:33:24.9518572Z ========================== Starting Command Output ===========================
2020-02-15T21:33:24.9530235Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e4668914-53d3-433c-98e1-2a14bcd74274.sh
2020-02-15T21:33:24.9530598Z 
2020-02-15T21:33:24.9539476Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T21:33:24.9545969Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69196/merge to s
2020-02-15T21:33:24.9547854Z Task         : Get sources
2020-02-15T21:33:24.9547892Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T21:33:24.9547928Z Version      : 1.0.0
2020-02-15T21:33:24.9547964Z Author       : Microsoft
---
2020-02-15T21:33:26.0252055Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T21:33:26.0269093Z ##[command]git config gc.auto 0
2020-02-15T21:33:26.0271666Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T21:33:26.0273769Z ##[command]git config --get-all http.proxy
2020-02-15T21:33:26.0280568Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69196/merge:refs/remotes/pull/69196/merge
---
2020-02-15T21:38:33.4717988Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-02-15T21:38:33.6515645Z     Checking backtrace v0.3.44
2020-02-15T21:38:34.4308172Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-02-15T21:38:34.4361267Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-02-15T21:38:37.5165396Z error[E0277]: the trait bound `alloc_crate::boxed::Box<(dyn error::Error + core::marker::Send + core::marker::Sync + 'static)>: core::convert::From<core::num::TryFromIntError>` is not satisfied
2020-02-15T21:38:37.5166505Z    --> src/libstd/sys/unix/fs.rs:796:45
2020-02-15T21:38:37.5166950Z     |
2020-02-15T21:38:37.5167452Z 796 |                 size.try_into().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
2020-02-15T21:38:37.5168183Z     |                                             ^^^^^^^^^^^^^^ the trait `core::convert::From<core::num::TryFromIntError>` is not implemented for `alloc_crate::boxed::Box<(dyn error::Error + core::marker::Send + core::marker::Sync + 'static)>`
2020-02-15T21:38:37.5169046Z    ::: src/libstd/io/error.rs:247:12
2020-02-15T21:38:37.5169429Z     |
2020-02-15T21:38:37.5169429Z     |
2020-02-15T21:38:37.5169911Z 247 |     pub fn new<E>(kind: ErrorKind, error: E) -> Error
2020-02-15T21:38:37.5170839Z 248 |     where
2020-02-15T21:38:37.5171340Z 249 |         E: Into<Box<dyn error::Error + Send + Sync>>,
2020-02-15T21:38:37.5171892Z     |            ----------------------------------------- required by this bound in `io::error::Error::new`
2020-02-15T21:38:37.5172302Z     |
2020-02-15T21:38:37.5172302Z     |
2020-02-15T21:38:37.5172748Z     = help: the following implementations were found:
2020-02-15T21:38:37.5173222Z               <alloc_crate::boxed::Box<(dyn error::Error + 'static)> as core::convert::From<&str>>
2020-02-15T21:38:37.5173965Z               <alloc_crate::boxed::Box<(dyn error::Error + 'static)> as core::convert::From<alloc_crate::borrow::Cow<'a, str>>>
2020-02-15T21:38:37.5174473Z               <alloc_crate::boxed::Box<(dyn error::Error + 'static)> as core::convert::From<alloc_crate::string::String>>
2020-02-15T21:38:37.5175072Z               <alloc_crate::boxed::Box<(dyn error::Error + core::marker::Send + core::marker::Sync + 'a)> as core::convert::From<&str>>
2020-02-15T21:38:37.5175532Z             and 14 others
2020-02-15T21:38:37.5176140Z     = note: required because of the requirements on the impl of `core::convert::Into<alloc_crate::boxed::Box<(dyn error::Error + core::marker::Send + core::marker::Sync + 'static)>>` for `core::num::TryFromIntError`
2020-02-15T21:38:37.6992854Z error: aborting due to previous error
2020-02-15T21:38:37.6993518Z 
2020-02-15T21:38:37.6994094Z For more information about this error, try `rustc --explain E0277`.
2020-02-15T21:38:37.7105011Z error: could not compile `std`.
---
2020-02-15T21:38:37.7194576Z   local time: Sat Feb 15 21:38:37 UTC 2020
2020-02-15T21:38:38.2514213Z   network time: Sat, 15 Feb 2020 21:38:38 GMT
2020-02-15T21:38:38.2515644Z == end clock drift check ==
2020-02-15T21:38:39.2385593Z 
2020-02-15T21:38:39.2489296Z ##[error]Bash exited with code '1'.
2020-02-15T21:38:39.2507619Z ##[section]Finishing: Run build
2020-02-15T21:38:39.2523226Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69196/merge to s
2020-02-15T21:38:39.2525157Z Task         : Get sources
2020-02-15T21:38:39.2525210Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T21:38:39.2525281Z Version      : 1.0.0
2020-02-15T21:38:39.2525327Z Author       : Microsoft
2020-02-15T21:38:39.2525327Z Author       : Microsoft
2020-02-15T21:38:39.2525379Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T21:38:39.2525452Z ==============================================================================
2020-02-15T21:38:39.7186966Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T21:38:39.7222226Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69196/merge to s
2020-02-15T21:38:39.7411224Z Cleaning up task key
2020-02-15T21:38:39.7412098Z Start cleaning up orphan processes.
2020-02-15T21:38:39.7625337Z Terminate orphan process: pid (4084) (python)
2020-02-15T21:38:39.7845433Z ##[section]Finishing: Finalize Job
