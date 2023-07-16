plain
2019-11-26T04:24:45.8581574Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T04:24:45.8766297Z ##[command]git config gc.auto 0
2019-11-26T04:24:45.8842236Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T04:24:45.8896777Z ##[command]git config --get-all http.proxy
2019-11-26T04:24:45.9051944Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66765/merge:refs/remotes/pull/66765/merge
---
2019-11-26T04:31:20.6874465Z    Compiling rustc-demangle v0.1.16
2019-11-26T04:31:20.9129505Z error[E0412]: cannot find type `OsStr` in this scope
2019-11-26T04:31:20.9130138Z    --> src/liballoc/borrow.rs:426:12
2019-11-26T04:31:20.9130324Z     |
2019-11-26T04:31:20.9130764Z 426 | impl AsRef<OsStr> for Cow<'_, str> {
2019-11-26T04:31:20.9138080Z 
2019-11-26T04:31:20.9174375Z error[E0412]: cannot find type `OsStr` in this scope
2019-11-26T04:31:20.9174762Z    --> src/liballoc/borrow.rs:427:26
2019-11-26T04:31:20.9175024Z     |
2019-11-26T04:31:20.9175024Z     |
2019-11-26T04:31:20.9175519Z 427 |     fn as_ref(&self) -> &OsStr {
2019-11-26T04:31:20.9176063Z 
2019-11-26T04:31:21.0898499Z error[E0547]: missing 'issue'
2019-11-26T04:31:21.0898939Z    --> src/liballoc/borrow.rs:425:1
2019-11-26T04:31:21.0899198Z     |
2019-11-26T04:31:21.0899198Z     |
2019-11-26T04:31:21.0899522Z 425 | #[unstable(feature = "cow_asref_osstr", reason = "recently added")]
2019-11-26T04:31:21.0899888Z 
2019-11-26T04:31:22.1883819Z error: aborting due to 3 previous errors
2019-11-26T04:31:22.1883923Z 
2019-11-26T04:31:22.1888909Z For more information about this error, try `rustc --explain E0412`.
---
2019-11-26T04:31:22.2620932Z   local time: Tue Nov 26 04:31:22 UTC 2019
2019-11-26T04:31:22.2911315Z   network time: Tue, 26 Nov 2019 04:31:22 GMT
2019-11-26T04:31:22.2913767Z == end clock drift check ==
2019-11-26T04:31:25.0500811Z 
2019-11-26T04:31:25.0598425Z ##[error]Bash exited with code '1'.
2019-11-26T04:31:25.0661279Z ##[section]Starting: Checkout
2019-11-26T04:31:25.0663929Z ==============================================================================
2019-11-26T04:31:25.0664002Z Task         : Get sources
2019-11-26T04:31:25.0664059Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
