plain
2020-01-06T15:38:12.6757910Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T15:38:12.6778529Z ##[command]git config gc.auto 0
2020-01-06T15:38:12.6783351Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T15:38:12.6785657Z ##[command]git config --get-all http.proxy
2020-01-06T15:38:12.6789806Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67913/merge:refs/remotes/pull/67913/merge
---
2020-01-06T15:43:46.5096968Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-06T15:43:46.6337466Z     Checking backtrace v0.3.40
2020-01-06T15:43:46.6786652Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-01-06T15:43:46.8492944Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-06T15:43:49.8299530Z error: use of item 'core::num::<impl u64>::max_value' that will be deprecated in future version 1.42.0: replaced by associated constant MAX
2020-01-06T15:43:49.8300732Z   --> src/libstd/../stdarch/crates/std_detect/src/detect/cache.rs:84:30
2020-01-06T15:43:49.8301207Z    |
2020-01-06T15:43:49.8301729Z 84 |         Cache(AtomicU64::new(u64::max_value()))
2020-01-06T15:43:49.8302678Z    |
2020-01-06T15:43:49.8303218Z    = note: `-D deprecated-in-future` implied by `-D warnings`
2020-01-06T15:43:49.8303394Z 
2020-01-06T15:43:49.8303394Z 
2020-01-06T15:43:49.8309394Z error: use of item 'core::num::<impl u64>::max_value' that will be deprecated in future version 1.42.0: replaced by associated constant MAX
2020-01-06T15:43:49.8310013Z   --> src/libstd/../stdarch/crates/std_detect/src/detect/cache.rs:89:43
2020-01-06T15:43:49.8310440Z    |
2020-01-06T15:43:49.8310947Z 89 |         self.0.load(Ordering::Relaxed) == u64::max_value()
2020-01-06T15:43:49.8311620Z 
2020-01-06T15:43:50.1726861Z error: aborting due to 2 previous errors
2020-01-06T15:43:50.1726966Z 
2020-01-06T15:43:50.1850676Z error: could not compile `std`.
---
2020-01-06T15:43:50.1949876Z   local time: Mon Jan  6 15:43:50 UTC 2020
2020-01-06T15:43:50.4906769Z   network time: Mon, 06 Jan 2020 15:43:50 GMT
2020-01-06T15:43:50.4911299Z == end clock drift check ==
2020-01-06T15:43:58.7314620Z 
2020-01-06T15:43:58.7378907Z ##[error]Bash exited with code '1'.
2020-01-06T15:43:58.7406498Z ##[section]Starting: Checkout
2020-01-06T15:43:58.7408565Z ==============================================================================
2020-01-06T15:43:58.7408654Z Task         : Get sources
2020-01-06T15:43:58.7408701Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
