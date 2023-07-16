plain
2019-12-16T16:59:19.4005229Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T16:59:19.4238653Z ##[command]git config gc.auto 0
2019-12-16T16:59:19.4329858Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T16:59:19.4432892Z ##[command]git config --get-all http.proxy
2019-12-16T16:59:19.4583695Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67163/merge:refs/remotes/pull/67163/merge
---
2019-12-16T17:04:23.8548558Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-12-16T17:04:23.9275383Z error: expected identifier, found `<<`
2019-12-16T17:04:23.9276398Z     --> src/libcore/ptr/mod.rs:1039:1
2019-12-16T17:04:23.9276827Z      |
2019-12-16T17:04:23.9277690Z 1039 | <<<<<<< HEAD
2019-12-16T17:04:23.9278063Z      | ^^ expected identifier
2019-12-16T17:04:23.9303258Z error: aborting due to previous error
2019-12-16T17:04:23.9303606Z 
2019-12-16T17:04:23.9304023Z error: could not compile `core`.
2019-12-16T17:04:23.9304512Z warning: build failed, waiting for other jobs to finish...
---
2019-12-16T17:04:28.7279897Z   local time: Mon Dec 16 17:04:28 UTC 2019
2019-12-16T17:04:28.8053378Z   network time: Mon, 16 Dec 2019 17:04:28 GMT
2019-12-16T17:04:28.8059078Z == end clock drift check ==
2019-12-16T17:04:43.6673159Z 
2019-12-16T17:04:43.6781902Z ##[error]Bash exited with code '1'.
2019-12-16T17:04:43.6813808Z ##[section]Starting: Checkout
2019-12-16T17:04:43.6815486Z ==============================================================================
2019-12-16T17:04:43.6815554Z Task         : Get sources
2019-12-16T17:04:43.6815598Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
