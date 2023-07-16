plain
2019-12-12T14:08:47.3164335Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-12T14:08:47.3390629Z ##[command]git config gc.auto 0
2019-12-12T14:08:47.3462163Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-12T14:08:47.3510646Z ##[command]git config --get-all http.proxy
2019-12-12T14:08:47.3651809Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67253/merge:refs/remotes/pull/67253/merge
---
2019-12-12T14:14:17.7010564Z    Compiling serde_json v1.0.40
2019-12-12T14:14:19.3858010Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-12T14:14:29.9550435Z     Finished release [optimized] target(s) in 1m 19s
2019-12-12T14:14:29.9648066Z tidy check
2019-12-12T14:14:30.1698495Z tidy error: /checkout/src/libcore/fmt/mod.rs:725: line longer than 100 chars
2019-12-12T14:14:32.4601737Z some tidy checks failed
2019-12-12T14:14:32.4602166Z Found 485 error codes
2019-12-12T14:14:32.4602206Z Found 0 error codes with no tests
2019-12-12T14:14:32.4602244Z Done!
2019-12-12T14:14:32.4602244Z Done!
2019-12-12T14:14:32.4604949Z 
2019-12-12T14:14:32.4605200Z 
2019-12-12T14:14:32.4606475Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-12T14:14:32.4606962Z 
2019-12-12T14:14:32.4606991Z 
2019-12-12T14:14:32.4664782Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-12T14:14:32.4665797Z Build completed unsuccessfully in 0:01:23
2019-12-12T14:14:32.4665797Z Build completed unsuccessfully in 0:01:23
2019-12-12T14:14:32.4670459Z == clock drift check ==
2019-12-12T14:14:32.4683462Z   local time: Thu Dec 12 14:14:32 UTC 2019
2019-12-12T14:14:32.5673827Z   network time: Thu, 12 Dec 2019 14:14:32 GMT
2019-12-12T14:14:32.5680167Z == end clock drift check ==
2019-12-12T14:14:33.9590974Z 
2019-12-12T14:14:33.9688523Z ##[error]Bash exited with code '1'.
2019-12-12T14:14:33.9729046Z ##[section]Starting: Checkout
2019-12-12T14:14:33.9730656Z ==============================================================================
2019-12-12T14:14:33.9730706Z Task         : Get sources
2019-12-12T14:14:33.9730764Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
