plain
2020-01-11T08:58:14.2752251Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T08:58:14.2836394Z ##[command]git config gc.auto 0
2020-01-11T08:58:14.2921207Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T08:58:14.2998595Z ##[command]git config --get-all http.proxy
2020-01-11T08:58:14.3155453Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68123/merge:refs/remotes/pull/68123/merge
---
2020-01-11T09:03:12.0502648Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-01-11T09:03:12.3207028Z     Checking rustc-demangle v0.1.16
2020-01-11T09:03:12.6393541Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-11T09:03:12.8181543Z     Checking backtrace v0.3.40
2020-01-11T09:03:14.3225518Z error: type does not implement `fmt::Debug`; consider adding `#[derive(Debug)]` or a manual implementation
2020-01-11T09:03:14.3229983Z      |
2020-01-11T09:03:14.3229983Z      |
2020-01-11T09:03:14.3230591Z 1069 | / pub struct Cursor<'a, T: 'a> {
2020-01-11T09:03:14.3231149Z 1070 | |     index: usize,
2020-01-11T09:03:14.3231692Z 1071 | |     current: Option<NonNull<Node<T>>>,
2020-01-11T09:03:14.3232206Z 1072 | |     list: &'a LinkedList<T>,
2020-01-11T09:03:14.3233180Z      | |_^
2020-01-11T09:03:14.3233627Z      |
2020-01-11T09:03:14.3234658Z      = note: `-D missing-debug-implementations` implied by `-D warnings`
2020-01-11T09:03:14.3235027Z 
2020-01-11T09:03:14.3235027Z 
2020-01-11T09:03:14.3235664Z error: type does not implement `fmt::Debug`; consider adding `#[derive(Debug)]` or a manual implementation
2020-01-11T09:03:14.3236725Z      |
2020-01-11T09:03:14.3236725Z      |
2020-01-11T09:03:14.3237292Z 1088 | / pub struct CursorMut<'a, T: 'a> {
2020-01-11T09:03:14.3238051Z 1089 | |     index: usize,
2020-01-11T09:03:14.3238618Z 1090 | |     current: Option<NonNull<Node<T>>>,
2020-01-11T09:03:14.3239512Z 1091 | |     list: &'a mut LinkedList<T>,
2020-01-11T09:03:14.3240877Z      | |_^
2020-01-11T09:03:14.3241079Z 
2020-01-11T09:03:14.3742229Z error: aborting due to 2 previous errors
2020-01-11T09:03:14.3742668Z 
---
2020-01-11T09:03:14.3955112Z   local time: Sat Jan 11 09:03:14 UTC 2020
2020-01-11T09:03:14.6799054Z   network time: Sat, 11 Jan 2020 09:03:14 GMT
2020-01-11T09:03:14.6800077Z == end clock drift check ==
2020-01-11T09:03:20.8361279Z 
2020-01-11T09:03:20.8481110Z ##[error]Bash exited with code '1'.
2020-01-11T09:03:20.8514707Z ##[section]Starting: Checkout
2020-01-11T09:03:20.8516420Z ==============================================================================
2020-01-11T09:03:20.8516480Z Task         : Get sources
2020-01-11T09:03:20.8516548Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
