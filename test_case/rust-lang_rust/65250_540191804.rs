plain
2019-10-09T20:38:08.1523022Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T20:38:08.1616667Z ##[command]git config gc.auto 0
2019-10-09T20:38:08.1697636Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T20:38:08.1757983Z ##[command]git config --get-all http.proxy
2019-10-09T20:38:08.1907886Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65250/merge:refs/remotes/pull/65250/merge
---
2019-10-09T20:43:49.8575369Z    Compiling serde_json v1.0.40
2019-10-09T20:43:51.5864310Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-09T20:44:02.9250192Z     Finished release [optimized] target(s) in 1m 28s
2019-10-09T20:44:02.9340719Z tidy check
2019-10-09T20:44:03.1374302Z tidy error: /checkout/src/test/ui/issues/issue-pr29383.rs:9: line longer than 100 chars
2019-10-09T20:44:03.1375401Z tidy error: /checkout/src/test/ui/issues/issue-pr29383.rs:10: line longer than 100 chars
2019-10-09T20:44:03.1380019Z tidy error: /checkout/src/test/ui/issues/issue-63983.rs:9: line longer than 100 chars
2019-10-09T20:44:03.1380704Z tidy error: /checkout/src/test/ui/issues/issue-63983.rs:11: line longer than 100 chars
2019-10-09T20:44:03.1507318Z tidy error: /checkout/src/test/ui/issues/issue-32004.rs:11: line longer than 100 chars
2019-10-09T20:44:03.1997117Z tidy error: /checkout/src/test/ui/empty/empty-struct-tuple-pat.rs:30: line longer than 100 chars
2019-10-09T20:44:03.1998944Z tidy error: /checkout/src/test/ui/empty/empty-struct-tuple-pat.rs:34: line longer than 100 chars
2019-10-09T20:44:03.2006534Z tidy error: /checkout/src/test/ui/empty/empty-struct-braces-pat-1.rs:25: line longer than 100 chars
2019-10-09T20:44:03.2006859Z tidy error: /checkout/src/test/ui/empty/empty-struct-braces-pat-1.rs:32: line longer than 100 chars
2019-10-09T20:44:03.2007156Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:21: line longer than 100 chars
2019-10-09T20:44:03.2007422Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:24: line longer than 100 chars
2019-10-09T20:44:03.2008329Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:27: line longer than 100 chars
2019-10-09T20:44:03.2008642Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:30: line longer than 100 chars
2019-10-09T20:44:03.2008940Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:34: line longer than 100 chars
2019-10-09T20:44:03.2009220Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:42: line longer than 100 chars
2019-10-09T20:44:03.2193221Z tidy error: /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:11: line longer than 100 chars
2019-10-09T20:44:03.2193491Z tidy error: /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:13: line longer than 100 chars
2019-10-09T20:44:03.2193743Z tidy error: /checkout/src/test/ui/pattern/pattern-error-continue.rs:18: line longer than 100 chars
2019-10-09T20:44:03.2193972Z tidy error: /checkout/src/test/ui/pattern/pattern-binding-disambiguation.rs:52: line longer than 100 chars
2019-10-09T20:44:03.2194196Z tidy error: /checkout/src/test/ui/pattern/pattern-binding-disambiguation.rs:53: line longer than 100 chars
2019-10-09T20:44:03.2234329Z tidy error: /checkout/src/test/ui/privacy/privacy5.rs:104: line longer than 100 chars
2019-10-09T20:44:03.2234587Z tidy error: /checkout/src/test/ui/privacy/privacy5.rs:105: line longer than 100 chars
2019-10-09T20:44:03.2483090Z tidy error: /checkout/src/test/ui/ufcs/ufcs-partially-resolved.rs:28: line longer than 100 chars
2019-10-09T20:44:03.2668383Z tidy error: /checkout/src/test/ui/match/match-pattern-field-mismatch.rs:11: line longer than 100 chars
2019-10-09T20:44:04.8225328Z some tidy checks failed
2019-10-09T20:44:04.8225542Z Found 482 error codes
2019-10-09T20:44:04.8225974Z Found 0 error codes with no tests
2019-10-09T20:44:04.8226018Z Done!
2019-10-09T20:44:04.8226018Z Done!
2019-10-09T20:44:04.8232626Z 
2019-10-09T20:44:04.8233136Z 
2019-10-09T20:44:04.8234143Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-09T20:44:04.8234348Z 
2019-10-09T20:44:04.8234371Z 
2019-10-09T20:44:04.8246978Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-09T20:44:04.8247374Z Build completed unsuccessfully in 0:01:31
2019-10-09T20:44:04.8247374Z Build completed unsuccessfully in 0:01:31
2019-10-09T20:44:04.8299229Z == clock drift check ==
2019-10-09T20:44:04.8314591Z   local time: Wed Oct  9 20:44:04 UTC 2019
2019-10-09T20:44:04.9280534Z   network time: Wed, 09 Oct 2019 20:44:04 GMT
2019-10-09T20:44:04.9288066Z == end clock drift check ==
2019-10-09T20:44:05.8701926Z ##[error]Bash exited with code '1'.
2019-10-09T20:44:05.8745025Z ##[section]Starting: Checkout
2019-10-09T20:44:05.8746785Z ==============================================================================
2019-10-09T20:44:05.8746830Z Task         : Get sources
2019-10-09T20:44:05.8746888Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
