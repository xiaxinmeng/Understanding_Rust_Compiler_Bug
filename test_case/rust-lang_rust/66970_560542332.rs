plain
2019-12-02T19:06:15.0902369Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T19:06:15.8179400Z ##[command]git config gc.auto 0
2019-12-02T19:06:15.8185652Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T19:06:15.8189264Z ##[command]git config --get-all http.proxy
2019-12-02T19:06:15.8193385Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66970/merge:refs/remotes/pull/66970/merge
---
2019-12-02T19:11:54.9249458Z    Compiling serde_json v1.0.40
2019-12-02T19:11:56.5307930Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-02T19:12:06.6472459Z     Finished release [optimized] target(s) in 1m 20s
2019-12-02T19:12:06.6565171Z tidy check
2019-12-02T19:12:06.9953553Z tidy error: /checkout/src/test/ui/enum/union-in-enum.rs: too many trailing newlines (2)
2019-12-02T19:12:09.0454003Z Found 486 error codes
2019-12-02T19:12:09.0455741Z Found 0 error codes with no tests
2019-12-02T19:12:09.0456108Z Done!
2019-12-02T19:12:09.0456324Z some tidy checks failed
2019-12-02T19:12:09.0456324Z some tidy checks failed
2019-12-02T19:12:09.0456536Z 
2019-12-02T19:12:09.0456692Z 
2019-12-02T19:12:09.0457650Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-02T19:12:09.0458142Z 
2019-12-02T19:12:09.0458305Z 
2019-12-02T19:12:09.0458587Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-02T19:12:09.0459127Z Build completed unsuccessfully in 0:01:24
2019-12-02T19:12:09.0459127Z Build completed unsuccessfully in 0:01:24
2019-12-02T19:12:09.0503203Z == clock drift check ==
2019-12-02T19:12:09.0512980Z   local time: Mon Dec  2 19:12:09 UTC 2019
2019-12-02T19:12:09.3282464Z   network time: Mon, 02 Dec 2019 19:12:09 GMT
2019-12-02T19:12:09.3286468Z == end clock drift check ==
2019-12-02T19:12:10.7382910Z 
2019-12-02T19:12:10.7477158Z ##[error]Bash exited with code '1'.
2019-12-02T19:12:10.7502052Z ##[section]Starting: Checkout
2019-12-02T19:12:10.7503921Z ==============================================================================
2019-12-02T19:12:10.7503990Z Task         : Get sources
2019-12-02T19:12:10.7504032Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
