plain
2019-08-27T20:14:31.2014900Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T20:14:31.2208584Z ##[command]git config gc.auto 0
2019-08-27T20:14:31.2296650Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T20:14:31.2375070Z ##[command]git config --get-all http.proxy
2019-08-27T20:14:31.2521066Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63919/merge:refs/remotes/pull/63919/merge
---
2019-08-27T20:15:04.4735177Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T20:15:04.4735425Z 
2019-08-27T20:15:04.4735796Z   git checkout -b <new-branch-name>
2019-08-27T20:15:04.4735990Z 
2019-08-27T20:15:04.4776655Z HEAD is now at 3a6257f97 Merge 9f9ff32f00c8e588406b73583c5095578a4b2200 into 53df91a9b24ad999e7ca896447af6f5f74fe43bc
2019-08-27T20:15:04.4901002Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T20:15:04.4905070Z ==============================================================================
2019-08-27T20:15:04.4905127Z Task         : Bash
2019-08-27T20:15:04.4905191Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T20:48:00.8052869Z == clock drift check ==
2019-08-27T20:48:00.8070204Z   local time: Tue Aug 27 20:48:00 UTC 2019
2019-08-27T20:48:00.9596579Z   network time: Tue, 27 Aug 2019 20:48:00 GMT
2019-08-27T20:48:00.9597618Z == end clock drift check ==
2019-08-27T20:48:03.2548281Z ##[error]Bash exited with code '1'.
2019-08-27T20:48:03.2595346Z ##[section]Starting: Checkout
2019-08-27T20:48:03.2596992Z ==============================================================================
2019-08-27T20:48:03.2597305Z Task         : Get sources
2019-08-27T20:48:03.2597557Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
