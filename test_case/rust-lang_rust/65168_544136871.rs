plain
2019-10-19T11:57:51.4492400Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T11:57:51.4717202Z ##[command]git config gc.auto 0
2019-10-19T11:57:51.4789929Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T11:57:51.4845719Z ##[command]git config --get-all http.proxy
2019-10-19T11:57:51.4992962Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65168/merge:refs/remotes/pull/65168/merge
---
2019-10-19T12:03:29.0434107Z     Checking rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-10-19T12:03:31.7440849Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2019-10-19T12:03:32.0858280Z     Checking cfg-if v0.1.8
2019-10-19T12:03:32.1291855Z     Checking rustc-demangle v0.1.16
2019-10-19T12:03:32.2309202Z error[E0541]: unknown meta item 'issue'
2019-10-19T12:03:32.2309682Z     --> src/liballoc/boxed.rs:1107:41
2019-10-19T12:03:32.2309916Z      |
2019-10-19T12:03:32.2310396Z 1107 | #[stable(feature = "box_str_from_iter", issue = "0", since = "1.40.0")]
2019-10-19T12:03:32.2310919Z      |                                         ^^^^^^^^^^^ expected one of `since`, `note`
2019-10-19T12:03:33.1428091Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-10-19T12:03:33.1428690Z     Checking backtrace v0.3.37
2019-10-19T12:03:33.4506992Z error: aborting due to previous error
2019-10-19T12:03:33.4507581Z 
---
2019-10-19T12:03:33.4845292Z   local time: Sat Oct 19 12:03:33 UTC 2019
2019-10-19T12:03:33.6477457Z   network time: Sat, 19 Oct 2019 12:03:33 GMT
2019-10-19T12:03:33.6482523Z == end clock drift check ==
2019-10-19T12:03:38.4444458Z 
2019-10-19T12:03:38.4554207Z ##[error]Bash exited with code '1'.
2019-10-19T12:03:38.4588600Z ##[section]Starting: Checkout
2019-10-19T12:03:38.4590504Z ==============================================================================
2019-10-19T12:03:38.4590558Z Task         : Get sources
2019-10-19T12:03:38.4590604Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
