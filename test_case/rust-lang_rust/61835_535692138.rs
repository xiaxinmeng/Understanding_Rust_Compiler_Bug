plain
2019-09-26T21:13:02.1206346Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T21:13:02.1441781Z ##[command]git config gc.auto 0
2019-09-26T21:13:02.1507063Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T21:13:02.1953465Z ##[command]git config --get-all http.proxy
2019-09-26T21:13:02.2128307Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-09-26T21:21:46.0987520Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-26T21:21:52.3666877Z error[E0107]: wrong number of lifetime arguments: expected 1, found 2
2019-09-26T21:21:52.3667838Z   --> src/librustc/ty/constness.rs:72:45
2019-09-26T21:21:52.3668394Z    |
2019-09-26T21:21:52.3668989Z 72 |     fn is_const_intrinsic(tcx: TyCtxt<'tcx, 'tcx>, def_id: DefId) -> bool {
2019-09-26T21:21:52.3669849Z 
2019-09-26T21:21:52.4763210Z error: aborting due to previous error
2019-09-26T21:21:52.4764436Z 
2019-09-26T21:21:52.4765233Z For more information about this error, try `rustc --explain E0107`.
---
2019-09-26T21:21:52.6370617Z == clock drift check ==
2019-09-26T21:21:52.6393096Z   local time: Thu Sep 26 21:21:52 UTC 2019
2019-09-26T21:21:52.7249830Z   network time: Thu, 26 Sep 2019 21:21:52 GMT
2019-09-26T21:21:52.7249982Z == end clock drift check ==
2019-09-26T21:21:53.9638098Z ##[error]Bash exited with code '1'.
2019-09-26T21:21:53.9679477Z ##[section]Starting: Checkout
2019-09-26T21:21:53.9684245Z ==============================================================================
2019-09-26T21:21:53.9684310Z Task         : Get sources
2019-09-26T21:21:53.9684359Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
