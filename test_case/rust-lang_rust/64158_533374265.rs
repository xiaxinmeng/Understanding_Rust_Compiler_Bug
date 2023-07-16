plain
2019-09-20T02:01:00.8612674Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T02:01:00.8837867Z ##[command]git config gc.auto 0
2019-09-20T02:01:00.8914804Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T02:01:00.8962601Z ##[command]git config --get-all http.proxy
2019-09-20T02:01:00.9117345Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64158/merge:refs/remotes/pull/64158/merge
---
2019-09-20T02:07:36.5033686Z     Checking test v0.0.0 (/checkout/src/libtest)
2019-09-20T02:07:36.6412457Z error: unused import: `TestOpts`
2019-09-20T02:07:36.6412957Z     --> src/libtest/lib.rs:1812:79
2019-09-20T02:07:36.6413389Z      |
2019-09-20T02:07:36.6413742Z 1812 |         BenchMode, BenchSamples, Bencher, MonitorMsg, Sender, Sink, TestDesc, TestOpts, TestResult
2019-09-20T02:07:36.6414274Z      |
2019-09-20T02:07:36.6414519Z      = note: `-D unused-imports` implied by `-D warnings`
2019-09-20T02:07:36.6414554Z 
2019-09-20T02:07:36.9760649Z error: aborting due to previous error
---
2019-09-20T02:07:36.9986349Z == clock drift check ==
2019-09-20T02:07:37.0009749Z   local time: Fri Sep 20 02:07:37 UTC 2019
2019-09-20T02:07:37.1499334Z   network time: Fri, 20 Sep 2019 02:07:37 GMT
2019-09-20T02:07:37.1504233Z == end clock drift check ==
2019-09-20T02:07:38.5047300Z ##[error]Bash exited with code '1'.
2019-09-20T02:07:38.5079064Z ##[section]Starting: Checkout
2019-09-20T02:07:38.5080758Z ==============================================================================
2019-09-20T02:07:38.5080983Z Task         : Get sources
2019-09-20T02:07:38.5081025Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
