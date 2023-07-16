plain
2019-11-23T06:42:55.3343987Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T06:42:55.3357218Z ##[command]git config gc.auto 0
2019-11-23T06:42:55.3359894Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T06:42:55.3362647Z ##[command]git config --get-all http.proxy
2019-11-23T06:42:55.3370213Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66660/merge:refs/remotes/pull/66660/merge
---
2019-11-23T06:48:17.6611501Z    Compiling serde_json v1.0.40
2019-11-23T06:48:19.2051229Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-23T06:48:29.1804382Z     Finished release [optimized] target(s) in 1m 19s
2019-11-23T06:48:29.1902741Z tidy check
2019-11-23T06:48:29.4458225Z tidy error: /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:20: line longer than 100 chars
2019-11-23T06:48:29.4459195Z tidy error: /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:23: line longer than 100 chars
2019-11-23T06:48:31.6394834Z Found 441 error codes
2019-11-23T06:48:31.6395156Z Found 0 error codes with no tests
2019-11-23T06:48:31.6395201Z Done!
2019-11-23T06:48:31.6395242Z some tidy checks failed
2019-11-23T06:48:31.6395242Z some tidy checks failed
2019-11-23T06:48:31.6408862Z 
2019-11-23T06:48:31.6408964Z 
2019-11-23T06:48:31.6409965Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-23T06:48:31.6410092Z 
2019-11-23T06:48:31.6410116Z 
2019-11-23T06:48:31.6410161Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-23T06:48:31.6410246Z Build completed unsuccessfully in 0:01:23
2019-11-23T06:48:31.6410246Z Build completed unsuccessfully in 0:01:23
2019-11-23T06:48:31.6464349Z == clock drift check ==
2019-11-23T06:48:31.6474879Z   local time: Sat Nov 23 06:48:31 UTC 2019
2019-11-23T06:48:31.7835472Z   network time: Sat, 23 Nov 2019 06:48:31 GMT
2019-11-23T06:48:31.7839979Z == end clock drift check ==
2019-11-23T06:48:32.7252547Z 
2019-11-23T06:48:32.7307953Z ##[error]Bash exited with code '1'.
2019-11-23T06:48:32.7347393Z ##[section]Starting: Checkout
2019-11-23T06:48:32.7348930Z ==============================================================================
2019-11-23T06:48:32.7348977Z Task         : Get sources
2019-11-23T06:48:32.7349019Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
