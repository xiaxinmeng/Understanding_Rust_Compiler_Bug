plain
2019-10-03T08:46:38.2947498Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T08:46:38.3149143Z ##[command]git config gc.auto 0
2019-10-03T08:46:38.3214588Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T08:46:38.3263542Z ##[command]git config --get-all http.proxy
2019-10-03T08:46:38.3424795Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65020/merge:refs/remotes/pull/65020/merge
---
2019-10-03T08:53:38.3438362Z    Compiling serde_json v1.0.40
2019-10-03T08:53:40.1489854Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-03T08:53:51.4231551Z     Finished release [optimized] target(s) in 1m 30s
2019-10-03T08:53:51.4322676Z tidy check
2019-10-03T08:53:51.7129818Z tidy error: /checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs: too many trailing newlines (2)
2019-10-03T08:53:53.4046066Z some tidy checks failed
2019-10-03T08:53:53.4051240Z 
2019-10-03T08:53:53.4051240Z 
2019-10-03T08:53:53.4052413Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-03T08:53:53.4053051Z 
2019-10-03T08:53:53.4053252Z 
2019-10-03T08:53:53.4058503Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-03T08:53:53.4058890Z Build completed unsuccessfully in 0:01:33
2019-10-03T08:53:53.4058890Z Build completed unsuccessfully in 0:01:33
2019-10-03T08:53:53.4111242Z == clock drift check ==
2019-10-03T08:53:53.4126455Z   local time: Thu Oct  3 08:53:53 UTC 2019
2019-10-03T08:53:53.5754572Z   network time: Thu, 03 Oct 2019 08:53:53 GMT
2019-10-03T08:53:53.5756887Z == end clock drift check ==
2019-10-03T08:53:54.9567336Z ##[error]Bash exited with code '1'.
2019-10-03T08:53:54.9598965Z ##[section]Starting: Checkout
2019-10-03T08:53:54.9601198Z ==============================================================================
2019-10-03T08:53:54.9601256Z Task         : Get sources
2019-10-03T08:53:54.9601303Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
