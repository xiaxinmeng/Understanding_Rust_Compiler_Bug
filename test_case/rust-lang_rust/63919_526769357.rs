plain
2019-08-30T22:14:59.1218203Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-30T22:14:59.1426775Z ##[command]git config gc.auto 0
2019-08-30T22:14:59.1550038Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-30T22:14:59.1590772Z ##[command]git config --get-all http.proxy
2019-08-30T22:14:59.1737016Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63919/merge:refs/remotes/pull/63919/merge
---
2019-08-30T22:21:58.5175288Z    Compiling serde_json v1.0.40
2019-08-30T22:22:00.2803316Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-30T22:22:11.1839795Z     Finished release [optimized] target(s) in 1m 29s
2019-08-30T22:22:11.1917870Z tidy check
2019-08-30T22:22:11.8842985Z tidy error: /checkout/src/test/ui/definition-reachable/auxiliary/underscore.rs: too many trailing newlines (2)
2019-08-30T22:22:13.2249391Z some tidy checks failed
2019-08-30T22:22:13.2257617Z 
2019-08-30T22:22:13.2257617Z 
2019-08-30T22:22:13.2258542Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-30T22:22:13.2260166Z 
2019-08-30T22:22:13.2261250Z 
2019-08-30T22:22:13.2264397Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-30T22:22:13.2264623Z Build completed unsuccessfully in 0:01:32
2019-08-30T22:22:13.2264623Z Build completed unsuccessfully in 0:01:32
2019-08-30T22:22:13.2321495Z == clock drift check ==
2019-08-30T22:22:13.2335375Z   local time: Fri Aug 30 22:22:13 UTC 2019
2019-08-30T22:22:13.3856834Z   network time: Fri, 30 Aug 2019 22:22:13 GMT
2019-08-30T22:22:13.3856963Z == end clock drift check ==
2019-08-30T22:22:14.8135952Z ##[error]Bash exited with code '1'.
2019-08-30T22:22:14.8178611Z ##[section]Starting: Checkout
2019-08-30T22:22:14.8180158Z ==============================================================================
2019-08-30T22:22:14.8180202Z Task         : Get sources
2019-08-30T22:22:14.8180239Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
