plain
2019-11-21T16:22:29.1607324Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T16:22:29.1620984Z ##[command]git config gc.auto 0
2019-11-21T16:22:29.1624148Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T16:22:29.1627504Z ##[command]git config --get-all http.proxy
2019-11-21T16:22:29.1631478Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66547/merge:refs/remotes/pull/66547/merge
---
2019-11-21T16:27:58.7502788Z tidy check
2019-11-21T16:27:59.7506603Z * 588 error codes
2019-11-21T16:27:59.7506725Z * highest error code: E0744
2019-11-21T16:28:00.0822242Z * 270 features
2019-11-21T16:28:00.3027299Z tidy error: /checkout/src/libstd/rt.rs:44: platform-specific cfg: cfg(not(all(target_os = "linux", target_env = "gnu")))
2019-11-21T16:28:00.9006434Z some tidy checks failed
2019-11-21T16:28:00.9006539Z Found 441 error codes
2019-11-21T16:28:00.9006648Z Found 0 error codes with no tests
2019-11-21T16:28:00.9006692Z Done!
2019-11-21T16:28:00.9006692Z Done!
2019-11-21T16:28:00.9006720Z 
2019-11-21T16:28:00.9006745Z 
2019-11-21T16:28:00.9007531Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-21T16:28:00.9007630Z 
2019-11-21T16:28:00.9007675Z 
2019-11-21T16:28:00.9014782Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-21T16:28:00.9015070Z Build completed unsuccessfully in 0:01:19
2019-11-21T16:28:00.9015070Z Build completed unsuccessfully in 0:01:19
2019-11-21T16:28:00.9063969Z == clock drift check ==
2019-11-21T16:28:00.9074767Z   local time: Thu Nov 21 16:28:00 UTC 2019
2019-11-21T16:28:01.1702061Z   network time: Thu, 21 Nov 2019 16:28:01 GMT
2019-11-21T16:28:01.1702207Z == end clock drift check ==
2019-11-21T16:28:02.5068956Z 
2019-11-21T16:28:02.5161810Z ##[error]Bash exited with code '1'.
2019-11-21T16:28:02.5187570Z ##[section]Starting: Checkout
2019-11-21T16:28:02.5189688Z ==============================================================================
2019-11-21T16:28:02.5189756Z Task         : Get sources
2019-11-21T16:28:02.5189803Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
