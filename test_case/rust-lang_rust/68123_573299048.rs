plain
2020-01-11T09:13:53.6211453Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T09:13:53.6223710Z ##[command]git config gc.auto 0
2020-01-11T09:13:53.6227901Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T09:13:53.6231025Z ##[command]git config --get-all http.proxy
2020-01-11T09:13:53.6233948Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68123/merge:refs/remotes/pull/68123/merge
---
2020-01-11T09:18:37.3766405Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-01-11T09:18:37.6567906Z     Checking rustc-demangle v0.1.16
2020-01-11T09:18:37.9798651Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-11T09:18:38.1471625Z     Checking backtrace v0.3.40
2020-01-11T09:18:39.6720194Z error: type does not implement `fmt::Debug`; consider adding `#[derive(Debug)]` or a manual implementation
2020-01-11T09:18:39.6721989Z      |
2020-01-11T09:18:39.6721989Z      |
2020-01-11T09:18:39.6722334Z 1070 | / pub struct Cursor<'a, T: 'a> {
2020-01-11T09:18:39.6731542Z 1071 | |     index: usize,
2020-01-11T09:18:39.6732892Z 1072 | |     current: Option<NonNull<Node<T>>>,
2020-01-11T09:18:39.6733509Z 1073 | |     list: &'a LinkedList<T>,
2020-01-11T09:18:39.6734580Z      | |_^
2020-01-11T09:18:39.6735145Z      |
2020-01-11T09:18:39.6735814Z      = note: `-D missing-debug-implementations` implied by `-D warnings`
2020-01-11T09:18:39.6736022Z 
2020-01-11T09:18:39.6736022Z 
2020-01-11T09:18:39.6736543Z error: type does not implement `fmt::Debug`; consider adding `#[derive(Debug)]` or a manual implementation
2020-01-11T09:18:39.6737467Z      |
2020-01-11T09:18:39.6737467Z      |
2020-01-11T09:18:39.6738023Z 1089 | / pub struct CursorMut<'a, T: 'a> {
2020-01-11T09:18:39.6738541Z 1090 | |     index: usize,
2020-01-11T09:18:39.6739411Z 1091 | |     current: Option<NonNull<Node<T>>>,
2020-01-11T09:18:39.6740103Z 1092 | |     list: &'a mut LinkedList<T>,
2020-01-11T09:18:39.6741265Z      | |_^
2020-01-11T09:18:39.6741443Z 
2020-01-11T09:18:39.7230934Z error: aborting due to 2 previous errors
2020-01-11T09:18:39.7231050Z 
---
2020-01-11T09:18:39.7473865Z   local time: Sat Jan 11 09:18:39 UTC 2020
2020-01-11T09:18:39.9011440Z   network time: Sat, 11 Jan 2020 09:18:39 GMT
2020-01-11T09:18:39.9011518Z == end clock drift check ==
2020-01-11T09:18:40.8006471Z 
2020-01-11T09:18:40.8112786Z ##[error]Bash exited with code '1'.
2020-01-11T09:18:40.8169056Z ##[section]Starting: Checkout
2020-01-11T09:18:40.8170846Z ==============================================================================
2020-01-11T09:18:40.8170897Z Task         : Get sources
2020-01-11T09:18:40.8170938Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
