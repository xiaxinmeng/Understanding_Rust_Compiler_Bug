plain
2020-01-06T02:03:24.4607448Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T02:03:24.4622791Z ##[command]git config gc.auto 0
2020-01-06T02:03:24.4626152Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T02:03:24.4630145Z ##[command]git config --get-all http.proxy
2020-01-06T02:03:24.4634281Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67913/merge:refs/remotes/pull/67913/merge
---
2020-01-06T02:08:48.6928914Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-06T02:08:48.8669309Z     Checking backtrace v0.3.40
2020-01-06T02:08:50.5482540Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-01-06T02:08:50.5491731Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-06T02:08:53.5541214Z error: use of item 'core::num::<impl u64>::max_value' that will be deprecated in future version 1.42.0: replaced by associated constant MAX
2020-01-06T02:08:53.5542380Z   --> src/libstd/../stdarch/crates/std_detect/src/detect/cache.rs:84:30
2020-01-06T02:08:53.5542826Z    |
2020-01-06T02:08:53.5543306Z 84 |         Cache(AtomicU64::new(u64::max_value()))
2020-01-06T02:08:53.5544139Z    |
2020-01-06T02:08:53.5544605Z    = note: `-D deprecated-in-future` implied by `-D warnings`
2020-01-06T02:08:53.5545371Z 
2020-01-06T02:08:53.5545371Z 
2020-01-06T02:08:53.5554291Z error: use of item 'core::num::<impl u64>::max_value' that will be deprecated in future version 1.42.0: replaced by associated constant MAX
2020-01-06T02:08:53.5554680Z   --> src/libstd/../stdarch/crates/std_detect/src/detect/cache.rs:89:43
2020-01-06T02:08:53.5554911Z    |
2020-01-06T02:08:53.5555228Z 89 |         self.0.load(Ordering::Relaxed) == u64::max_value()
2020-01-06T02:08:53.5555596Z 
2020-01-06T02:08:53.9143896Z error: aborting due to 2 previous errors
2020-01-06T02:08:53.9144057Z 
2020-01-06T02:08:53.9295575Z error: could not compile `std`.
---
2020-01-06T02:08:53.9404136Z   local time: Mon Jan  6 02:08:53 UTC 2020
2020-01-06T02:08:54.2062867Z   network time: Mon, 06 Jan 2020 02:08:54 GMT
2020-01-06T02:08:54.2068846Z == end clock drift check ==
2020-01-06T02:09:02.6859683Z 
2020-01-06T02:09:02.6971067Z ##[error]Bash exited with code '1'.
2020-01-06T02:09:02.7112246Z ##[section]Starting: Checkout
2020-01-06T02:09:02.7114445Z ==============================================================================
2020-01-06T02:09:02.7114519Z Task         : Get sources
2020-01-06T02:09:02.7114883Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
