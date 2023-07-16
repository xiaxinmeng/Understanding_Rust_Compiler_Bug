plain
2019-11-16T23:46:04.3569138Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T23:46:04.4215174Z ##[command]git config gc.auto 0
2019-11-16T23:46:04.4288632Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T23:46:04.4349390Z ##[command]git config --get-all http.proxy
2019-11-16T23:46:04.4481873Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66470/merge:refs/remotes/pull/66470/merge
---
2019-11-16T23:52:45.6743080Z Done!
2019-11-16T23:52:45.6744077Z some tidy checks failed
2019-11-16T23:52:45.6744360Z 
2019-11-16T23:52:45.6744676Z 
2019-11-16T23:52:45.6745669Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-16T23:52:45.6745787Z 
2019-11-16T23:52:45.6745812Z 
2019-11-16T23:52:45.6750712Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-16T23:52:45.6750806Z Build completed unsuccessfully in 0:01:37
2019-11-16T23:52:45.6750806Z Build completed unsuccessfully in 0:01:37
2019-11-16T23:52:45.6805322Z == clock drift check ==
2019-11-16T23:52:45.6814382Z   local time: Sat Nov 16 23:52:45 UTC 2019
2019-11-16T23:52:45.9574612Z   network time: Sat, 16 Nov 2019 23:52:45 GMT
2019-11-16T23:52:45.9575436Z == end clock drift check ==
2019-11-16T23:52:47.5421908Z 
2019-11-16T23:52:47.5515943Z ##[error]Bash exited with code '1'.
2019-11-16T23:52:47.5540394Z ##[section]Starting: Checkout
2019-11-16T23:52:47.5542132Z ==============================================================================
2019-11-16T23:52:47.5542215Z Task         : Get sources
2019-11-16T23:52:47.5542264Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
