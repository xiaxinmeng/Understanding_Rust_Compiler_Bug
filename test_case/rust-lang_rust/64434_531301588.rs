plain
2019-09-13T16:11:20.8133714Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-13T16:11:20.8307150Z ##[command]git config gc.auto 0
2019-09-13T16:11:20.8371219Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-13T16:11:20.8428452Z ##[command]git config --get-all http.proxy
2019-09-13T16:11:20.8549875Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64434/merge:refs/remotes/pull/64434/merge
---
2019-09-13T16:17:26.8574758Z     Checking hashbrown v0.5.0
2019-09-13T16:17:27.8998126Z error: incorrect close delimiter: `)`
2019-09-13T16:17:27.8998725Z    --> src/libstd/sys/wasi/fs.rs:631:64
2019-09-13T16:17:27.8999458Z     |
2019-09-13T16:17:27.9000253Z 631 |                 let path = PathBuf::from(OsString { inner: buf );
2019-09-13T16:17:27.9000673Z     |                                         -         -            ^ incorrect close delimiter
2019-09-13T16:17:27.9001287Z     |                                         |         un-closed delimiter
2019-09-13T16:17:27.9001287Z     |                                         |         un-closed delimiter
2019-09-13T16:17:27.9001576Z     |                                         close delimiter possibly meant for this
2019-09-13T16:17:30.2430860Z error: aborting due to previous error
2019-09-13T16:17:30.2431650Z 
2019-09-13T16:17:30.2763532Z error: Could not compile `std`.
2019-09-13T16:17:30.2764180Z 
---
2019-09-13T16:17:30.2859944Z == clock drift check ==
2019-09-13T16:17:30.2875489Z   local time: Fri Sep 13 16:17:30 UTC 2019
2019-09-13T16:17:30.3786523Z   network time: Fri, 13 Sep 2019 16:17:30 GMT
2019-09-13T16:17:30.3791382Z == end clock drift check ==
2019-09-13T16:17:32.5495669Z ##[error]Bash exited with code '1'.
2019-09-13T16:17:32.5524242Z ##[section]Starting: Checkout
2019-09-13T16:17:32.5525674Z ==============================================================================
2019-09-13T16:17:32.5525716Z Task         : Get sources
2019-09-13T16:17:32.5525753Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
