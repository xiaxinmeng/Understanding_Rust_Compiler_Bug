plain
2019-11-19T18:47:48.8507858Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-19T18:47:48.8727338Z ##[command]git config gc.auto 0
2019-11-19T18:47:48.8804383Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-19T18:47:48.8870357Z ##[command]git config --get-all http.proxy
2019-11-19T18:47:48.9004366Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66549/merge:refs/remotes/pull/66549/merge
---
2019-11-19T18:54:56.7573395Z    Compiling rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2019-11-19T18:54:56.9922207Z    Compiling cc v1.0.47
2019-11-19T18:54:58.2692905Z    Compiling lazy_static v0.2.11
2019-11-19T18:54:58.3478175Z    Compiling datafrog v2.0.1
2019-11-19T18:54:58.6047010Z error[E0277]: can't compare `Val` with `&Val`
2019-11-19T18:54:58.6047468Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/datafrog-2.0.1/src/treefrog.rs:379:50
2019-11-19T18:54:58.6047788Z     |
2019-11-19T18:54:58.6048109Z 362 |         Func: Fn(&Tuple) -> Key,
2019-11-19T18:54:58.6048614Z     |                                 - help: consider further restricting type parameter `Val`: `, Val: std::cmp::PartialOrd<&Val>`
2019-11-19T18:54:58.6048869Z ...
2019-11-19T18:54:58.6049206Z 379 |                 slice = gallop(slice, |kv| &kv.1 < v);
2019-11-19T18:54:58.6049679Z     |                                                  ^ no implementation for `Val < &Val` and `Val > &Val`
2019-11-19T18:54:58.6049939Z     |
2019-11-19T18:54:58.6050303Z     = help: the trait `std::cmp::PartialOrd<&Val>` is not implemented for `Val`
2019-11-19T18:54:58.6050886Z     = note: required because of the requirements on the impl of `std::cmp::PartialOrd<&&Val>` for `&Val`
2019-11-19T18:54:58.6195112Z error[E0277]: can't compare `Val` with `&Val`
2019-11-19T18:54:58.6195112Z error[E0277]: can't compare `Val` with `&Val`
2019-11-19T18:54:58.6195564Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/datafrog-2.0.1/src/treefrog.rs:464:54
2019-11-19T18:54:58.6195855Z     |
2019-11-19T18:54:58.6196262Z 448 |         Func: Fn(&Tuple) -> Key,
2019-11-19T18:54:58.6196752Z     |                                 - help: consider further restricting type parameter `Val`: `, Val: std::cmp::PartialOrd<&Val>`
2019-11-19T18:54:58.6197036Z ...
2019-11-19T18:54:58.6197406Z 464 |                     slice = gallop(slice, |kv| &kv.1 < v);
2019-11-19T18:54:58.6198075Z     |                                                      ^ no implementation for `Val < &Val` and `Val > &Val`
2019-11-19T18:54:58.6198383Z     |
2019-11-19T18:54:58.6198754Z     = help: the trait `std::cmp::PartialOrd<&Val>` is not implemented for `Val`
2019-11-19T18:54:58.6199384Z     = note: required because of the requirements on the impl of `std::cmp::PartialOrd<&&Val>` for `&Val`
2019-11-19T18:54:58.6367678Z error: aborting due to 2 previous errors
2019-11-19T18:54:58.6367757Z 
2019-11-19T18:54:58.6368185Z For more information about this error, try `rustc --explain E0277`.
2019-11-19T18:54:58.6432778Z error: could not compile `datafrog`.
---
2019-11-19T18:55:02.0492989Z   local time: Tue Nov 19 18:55:02 UTC 2019
2019-11-19T18:55:02.1844769Z   network time: Tue, 19 Nov 2019 18:55:02 GMT
2019-11-19T18:55:02.1845013Z == end clock drift check ==
2019-11-19T18:55:04.1638227Z 
2019-11-19T18:55:04.1717257Z ##[error]Bash exited with code '1'.
2019-11-19T18:55:04.1758033Z ##[section]Starting: Checkout
2019-11-19T18:55:04.1759848Z ==============================================================================
2019-11-19T18:55:04.1759911Z Task         : Get sources
2019-11-19T18:55:04.1760000Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
