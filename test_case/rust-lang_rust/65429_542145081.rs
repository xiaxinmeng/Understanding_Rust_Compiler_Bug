plain
2019-10-15T10:05:58.8165849Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T10:05:58.8282316Z ##[command]git config gc.auto 0
2019-10-15T10:05:58.8360094Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T10:05:58.8428512Z ##[command]git config --get-all http.proxy
2019-10-15T10:05:58.8564825Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65429/merge:refs/remotes/pull/65429/merge
---
2019-10-15T10:12:12.6439582Z    Compiling serde_json v1.0.40
2019-10-15T10:12:14.4764614Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-15T10:12:26.4029220Z     Finished release [optimized] target(s) in 1m 32s
2019-10-15T10:12:26.4108649Z tidy check
2019-10-15T10:12:27.5127335Z tidy error: /checkout/src/libstd/fs.rs:77: trailing whitespace
2019-10-15T10:12:28.9271492Z Found 482 error codes
2019-10-15T10:12:28.9272529Z Found 0 error codes with no tests
2019-10-15T10:12:28.9272589Z Done!
2019-10-15T10:12:28.9272629Z some tidy checks failed
2019-10-15T10:12:28.9272629Z some tidy checks failed
2019-10-15T10:12:28.9272688Z 
2019-10-15T10:12:28.9272714Z 
2019-10-15T10:12:28.9273514Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-15T10:12:28.9273896Z 
2019-10-15T10:12:28.9273922Z 
2019-10-15T10:12:28.9279562Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-15T10:12:28.9279658Z Build completed unsuccessfully in 0:01:35
2019-10-15T10:12:28.9279658Z Build completed unsuccessfully in 0:01:35
2019-10-15T10:12:28.9336796Z == clock drift check ==
2019-10-15T10:12:28.9356132Z   local time: Tue Oct 15 10:12:28 UTC 2019
2019-10-15T10:12:29.0213301Z   network time: Tue, 15 Oct 2019 10:12:29 GMT
2019-10-15T10:12:29.0213804Z == end clock drift check ==
2019-10-15T10:12:29.8354832Z ##[error]Bash exited with code '1'.
2019-10-15T10:12:29.8405829Z ##[section]Starting: Checkout
2019-10-15T10:12:29.8408275Z ==============================================================================
2019-10-15T10:12:29.8408340Z Task         : Get sources
2019-10-15T10:12:29.8408388Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
