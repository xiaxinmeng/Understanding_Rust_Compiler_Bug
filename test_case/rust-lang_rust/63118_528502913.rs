plain
2019-09-05T17:39:37.5165043Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-05T17:39:37.5372580Z ##[command]git config gc.auto 0
2019-09-05T17:39:37.5453893Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-05T17:39:37.5514716Z ##[command]git config --get-all http.proxy
2019-09-05T17:39:37.5655820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63118/merge:refs/remotes/pull/63118/merge
---
2019-09-05T17:46:45.4417116Z     Finished release [optimized] target(s) in 1m 31s
2019-09-05T17:46:45.4490227Z tidy check
2019-09-05T17:46:46.5040487Z * 578 error codes
2019-09-05T17:46:46.5040604Z * highest error code: E0733
2019-09-05T17:46:46.5432189Z tidy error: /checkout/src/libsyntax/feature_gate/accepted.rs:245: feature bind_by_move_pattern_guards is not sorted by since
2019-09-05T17:46:47.5412535Z some tidy checks failed
2019-09-05T17:46:47.5412911Z 
2019-09-05T17:46:47.5412911Z 
2019-09-05T17:46:47.5413933Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-05T17:46:47.5414101Z 
2019-09-05T17:46:47.5414122Z 
2019-09-05T17:46:47.5423336Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-05T17:46:47.5423422Z Build completed unsuccessfully in 0:01:34
2019-09-05T17:46:47.5423422Z Build completed unsuccessfully in 0:01:34
2019-09-05T17:46:47.5476736Z == clock drift check ==
2019-09-05T17:46:47.5513954Z   local time: Thu Sep  5 17:46:47 UTC 2019
2019-09-05T17:46:47.6352778Z   network time: Thu, 05 Sep 2019 17:46:47 GMT
2019-09-05T17:46:47.6357225Z == end clock drift check ==
2019-09-05T17:46:49.0186789Z ##[error]Bash exited with code '1'.
2019-09-05T17:46:49.0220729Z ##[section]Starting: Checkout
2019-09-05T17:46:49.0222488Z ==============================================================================
2019-09-05T17:46:49.0222541Z Task         : Get sources
2019-09-05T17:46:49.0222588Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
