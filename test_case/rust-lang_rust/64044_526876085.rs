plain
2019-08-31T23:21:33.4743852Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-31T23:21:33.4948105Z ##[command]git config gc.auto 0
2019-08-31T23:21:33.5047768Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-31T23:21:33.5100474Z ##[command]git config --get-all http.proxy
2019-08-31T23:21:33.5280624Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64044/merge:refs/remotes/pull/64044/merge
---
2019-08-31T23:31:27.0684399Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-08-31T23:31:27.5919299Z error: hidden lifetime parameters in types are deprecated
2019-08-31T23:31:27.5923999Z   --> src/librustdoc/html/format.rs:92:38
2019-08-31T23:31:27.5924464Z    |
2019-08-31T23:31:27.5925226Z 92 |     crate fn write_fmt(&mut self, v: fmt::Arguments) {
2019-08-31T23:31:27.5925812Z    |                                      ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-08-31T23:31:27.5926377Z error: hidden lifetime parameters in types are deprecated
2019-08-31T23:31:27.5926738Z    --> src/librustdoc/html/format.rs:102:44
2019-08-31T23:31:27.5927442Z     |
2019-08-31T23:31:27.5927442Z     |
2019-08-31T23:31:27.5927869Z 102 |     crate fn with_formatter<T: FnOnce(&mut fmt::Formatter) -> fmt::Result>(&mut self, t: T) {
2019-08-31T23:31:27.5928728Z     |                                            ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-08-31T23:31:30.2011722Z error: aborting due to 2 previous errors
2019-08-31T23:31:30.2012501Z 
2019-08-31T23:31:30.2391728Z error: Could not compile `rustdoc`.
2019-08-31T23:31:30.2392396Z 
---
2019-08-31T23:31:30.2496624Z == clock drift check ==
2019-08-31T23:31:30.2513033Z   local time: Sat Aug 31 23:31:30 UTC 2019
2019-08-31T23:31:30.4004304Z   network time: Sat, 31 Aug 2019 23:31:30 GMT
2019-08-31T23:31:30.4005195Z == end clock drift check ==
2019-08-31T23:31:32.4764949Z ##[error]Bash exited with code '1'.
2019-08-31T23:31:32.4802094Z ##[section]Starting: Checkout
2019-08-31T23:31:32.4804044Z ==============================================================================
2019-08-31T23:31:32.4804111Z Task         : Get sources
2019-08-31T23:31:32.4804153Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
