plain
2019-11-24T10:51:19.2365480Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T10:51:19.2381065Z ##[command]git config gc.auto 0
2019-11-24T10:51:19.2385748Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T10:51:19.2389241Z ##[command]git config --get-all http.proxy
2019-11-24T10:51:19.2394008Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66660/merge:refs/remotes/pull/66660/merge
---
2019-11-24T10:57:31.5270337Z   = note: `#[warn(unused_imports)]` on by default
2019-11-24T10:57:31.5294117Z 
2019-11-24T10:57:43.1105653Z     Finished release [optimized] target(s) in 1m 34s
2019-11-24T10:57:43.1216895Z tidy check
2019-11-24T10:57:43.4229210Z tidy error: /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:7: line longer than 100 chars
2019-11-24T10:57:43.4229687Z tidy error: /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:19: line longer than 100 chars
2019-11-24T10:57:43.4230100Z tidy error: /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:22: line longer than 100 chars
2019-11-24T10:57:43.4230434Z tidy error: /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:24: line longer than 100 chars
2019-11-24T10:57:46.1125524Z some tidy checks failed
2019-11-24T10:57:46.1126655Z Found 485 error codes
2019-11-24T10:57:46.1127026Z Found 0 error codes with no tests
2019-11-24T10:57:46.1127175Z Done!
2019-11-24T10:57:46.1127175Z Done!
2019-11-24T10:57:46.1127262Z 
2019-11-24T10:57:46.1127294Z 
2019-11-24T10:57:46.1128185Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-24T10:57:46.1128336Z 
2019-11-24T10:57:46.1128367Z 
2019-11-24T10:57:46.1134210Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-24T10:57:46.1134821Z Build completed unsuccessfully in 0:01:38
2019-11-24T10:57:46.1134821Z Build completed unsuccessfully in 0:01:38
2019-11-24T10:57:46.1187187Z == clock drift check ==
2019-11-24T10:57:46.1202447Z   local time: Sun Nov 24 10:57:46 UTC 2019
2019-11-24T10:57:46.3986679Z   network time: Sun, 24 Nov 2019 10:57:46 GMT
2019-11-24T10:57:46.3989870Z == end clock drift check ==
2019-11-24T10:57:47.7771776Z 
2019-11-24T10:57:47.7890247Z ##[error]Bash exited with code '1'.
2019-11-24T10:57:47.7978075Z ##[section]Starting: Checkout
2019-11-24T10:57:47.7981668Z ==============================================================================
2019-11-24T10:57:47.7981746Z Task         : Get sources
2019-11-24T10:57:47.7981795Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
