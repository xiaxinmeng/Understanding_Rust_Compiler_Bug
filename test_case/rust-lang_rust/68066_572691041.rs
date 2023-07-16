plain
2020-01-09T18:21:57.4773862Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-09T18:21:57.4863339Z ##[command]git config gc.auto 0
2020-01-09T18:21:57.4954161Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-09T18:21:57.5028742Z ##[command]git config --get-all http.proxy
2020-01-09T18:21:57.5169928Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68066/merge:refs/remotes/pull/68066/merge
---
2020-01-09T18:26:51.9814565Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-09T18:26:56.9953112Z error: the feature `manually_drop_take` has been stable since 1.42.0 and no longer requires an attribute to enable
2020-01-09T18:26:56.9955545Z    --> src/libstd/lib.rs:278:12
2020-01-09T18:26:56.9956314Z     |
2020-01-09T18:26:56.9956947Z 278 | #![feature(manually_drop_take)]
2020-01-09T18:26:56.9958816Z     |
2020-01-09T18:26:56.9959516Z     = note: `-D stable-features` implied by `-D warnings`
2020-01-09T18:26:56.9959820Z 
2020-01-09T18:26:57.1484266Z error: aborting due to previous error
---
2020-01-09T18:26:57.1740185Z   local time: Thu Jan  9 18:26:57 UTC 2020
2020-01-09T18:26:57.4632250Z   network time: Thu, 09 Jan 2020 18:26:57 GMT
2020-01-09T18:26:57.4635094Z == end clock drift check ==
2020-01-09T18:26:58.7467296Z 
2020-01-09T18:26:58.7630996Z ##[error]Bash exited with code '1'.
2020-01-09T18:26:58.7657976Z ##[section]Starting: Checkout
2020-01-09T18:26:58.7659892Z ==============================================================================
2020-01-09T18:26:58.7659949Z Task         : Get sources
2020-01-09T18:26:58.7659999Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
