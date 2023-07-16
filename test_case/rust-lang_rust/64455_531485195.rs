plain
2019-09-14T14:24:47.4015987Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-14T14:24:47.4216428Z ##[command]git config gc.auto 0
2019-09-14T14:24:47.4293963Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-14T14:24:47.4354918Z ##[command]git config --get-all http.proxy
2019-09-14T14:24:47.4523665Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64455/merge:refs/remotes/pull/64455/merge
---
2019-09-14T14:31:34.7926959Z    Compiling serde_json v1.0.40
2019-09-14T14:31:36.4876944Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-14T14:31:47.0630555Z     Finished release [optimized] target(s) in 1m 27s
2019-09-14T14:31:47.0711111Z tidy check
2019-09-14T14:31:47.2605020Z tidy error: /checkout/src/librustc_resolve/error_codes.rs:1551: line longer than 80 chars
2019-09-14T14:31:48.7887201Z some tidy checks failed
2019-09-14T14:31:48.7892422Z 
2019-09-14T14:31:48.7892422Z 
2019-09-14T14:31:48.7893521Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-14T14:31:48.7893667Z 
2019-09-14T14:31:48.7893693Z 
2019-09-14T14:31:48.7902192Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-14T14:31:48.7902451Z Build completed unsuccessfully in 0:01:30
2019-09-14T14:31:48.7902451Z Build completed unsuccessfully in 0:01:30
2019-09-14T14:31:48.7953918Z == clock drift check ==
2019-09-14T14:31:48.7971121Z   local time: Sat Sep 14 14:31:48 UTC 2019
2019-09-14T14:31:48.9591494Z   network time: Sat, 14 Sep 2019 14:31:48 GMT
2019-09-14T14:31:48.9596704Z == end clock drift check ==
2019-09-14T14:31:50.4513023Z ##[error]Bash exited with code '1'.
2019-09-14T14:31:50.4561042Z ##[section]Starting: Checkout
2019-09-14T14:31:50.4563005Z ==============================================================================
2019-09-14T14:31:50.4563059Z Task         : Get sources
2019-09-14T14:31:50.4563261Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
