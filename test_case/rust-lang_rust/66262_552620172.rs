plain
2019-11-11T20:58:39.7713120Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T20:58:39.7882339Z ##[command]git config gc.auto 0
2019-11-11T20:58:39.7946813Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T20:58:39.8007468Z ##[command]git config --get-all http.proxy
2019-11-11T20:58:39.8141016Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66262/merge:refs/remotes/pull/66262/merge
---
2019-11-11T21:05:07.9274354Z Done!
2019-11-11T21:05:07.9276662Z some tidy checks failed
2019-11-11T21:05:07.9278839Z 
2019-11-11T21:05:07.9279097Z 
2019-11-11T21:05:07.9280095Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-11T21:05:07.9280582Z 
2019-11-11T21:05:07.9280742Z 
2019-11-11T21:05:07.9284436Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-11T21:05:07.9284771Z Build completed unsuccessfully in 0:01:25
2019-11-11T21:05:07.9284771Z Build completed unsuccessfully in 0:01:25
2019-11-11T21:05:07.9333149Z == clock drift check ==
2019-11-11T21:05:07.9347104Z   local time: Mon Nov 11 21:05:07 UTC 2019
2019-11-11T21:05:08.0265625Z   network time: Mon, 11 Nov 2019 21:05:08 GMT
2019-11-11T21:05:08.0267511Z == end clock drift check ==
2019-11-11T21:05:09.3645183Z 
2019-11-11T21:05:09.3736792Z ##[error]Bash exited with code '1'.
2019-11-11T21:05:09.3759571Z ##[section]Starting: Checkout
2019-11-11T21:05:09.3760927Z ==============================================================================
2019-11-11T21:05:09.3760985Z Task         : Get sources
2019-11-11T21:05:09.3761023Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
