plain
2019-08-31T20:26:31.6196785Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-31T20:26:31.6400507Z ##[command]git config gc.auto 0
2019-08-31T20:26:31.6475114Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-31T20:26:31.6543803Z ##[command]git config --get-all http.proxy
2019-08-31T20:26:31.6676754Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64044/merge:refs/remotes/pull/64044/merge
---
2019-08-31T20:36:12.2155265Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-08-31T20:36:12.7431996Z error: hidden lifetime parameters in types are deprecated
2019-08-31T20:36:12.7433105Z   --> src/librustdoc/html/format.rs:94:38
2019-08-31T20:36:12.7433577Z    |
2019-08-31T20:36:12.7434100Z 94 |     crate fn write_fmt(&mut self, v: fmt::Arguments) {
2019-08-31T20:36:12.7435030Z    |                                      ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-08-31T20:36:12.7440361Z error: hidden lifetime parameters in types are deprecated
2019-08-31T20:36:12.7440919Z    --> src/librustdoc/html/format.rs:104:44
2019-08-31T20:36:12.7441347Z     |
2019-08-31T20:36:12.7441347Z     |
2019-08-31T20:36:12.7442073Z 104 |     crate fn with_formatter<T: FnOnce(&mut fmt::Formatter) -> fmt::Result>(&mut self, t: T) {
2019-08-31T20:36:12.7442737Z     |                                            ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-08-31T20:36:12.7443133Z 
2019-08-31T20:36:12.9694425Z error[E0201]: duplicate definitions with name `insert_str`:
2019-08-31T20:36:12.9695555Z   --> src/librustdoc/html/format.rs:82:5
2019-08-31T20:36:12.9695804Z    |
2019-08-31T20:36:12.9696168Z 74 | /     crate fn insert_str(&mut self, idx: usize, s: &str) {
2019-08-31T20:36:12.9696538Z 75 | |         self.buffer.insert_str(idx, s);
2019-08-31T20:36:12.9696863Z 76 | |     }
2019-08-31T20:36:12.9697212Z    | |_____- previous definition of `insert_str` here
2019-08-31T20:36:12.9697432Z ...
2019-08-31T20:36:12.9697786Z 82 | /     crate fn insert_str(&mut self, idx: usize, s: &str) {
2019-08-31T20:36:12.9698117Z 83 | |         self.buffer.insert_str(idx, s);
2019-08-31T20:36:12.9698966Z    | |_____^ duplicate definition
2019-08-31T20:36:12.9699214Z 
2019-08-31T20:36:12.9699214Z 
2019-08-31T20:36:12.9710488Z error[E0201]: duplicate definitions with name `push_str`:
2019-08-31T20:36:12.9710745Z   --> src/librustdoc/html/format.rs:86:5
2019-08-31T20:36:12.9710951Z    |
2019-08-31T20:36:12.9711271Z 78 | /     crate fn push_str(&mut self, s: &str) {
2019-08-31T20:36:12.9711525Z 79 | |         self.buffer.push_str(s);
2019-08-31T20:36:12.9711781Z 80 | |     }
2019-08-31T20:36:12.9712209Z    | |_____- previous definition of `push_str` here
2019-08-31T20:36:12.9712405Z ...
2019-08-31T20:36:12.9712874Z 86 | /     crate fn push_str(&mut self, s: &str) {
2019-08-31T20:36:12.9713139Z 87 | |         self.buffer.push_str(s);
2019-08-31T20:36:12.9713645Z    | |_____^ duplicate definition
2019-08-31T20:36:12.9713679Z 
2019-08-31T20:36:12.9713886Z error: aborting due to 4 previous errors
2019-08-31T20:36:12.9713932Z 
---
2019-08-31T20:36:13.0042534Z == clock drift check ==
2019-08-31T20:36:13.0058871Z   local time: Sat Aug 31 20:36:13 UTC 2019
2019-08-31T20:36:13.0902224Z   network time: Sat, 31 Aug 2019 20:36:13 GMT
2019-08-31T20:36:13.0907243Z == end clock drift check ==
2019-08-31T20:36:15.2224675Z ##[error]Bash exited with code '1'.
2019-08-31T20:36:15.2262507Z ##[section]Starting: Checkout
2019-08-31T20:36:15.2264169Z ==============================================================================
2019-08-31T20:36:15.2264237Z Task         : Get sources
2019-08-31T20:36:15.2264278Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
