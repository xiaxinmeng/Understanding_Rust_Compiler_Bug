plain
2019-12-09T18:39:44.2950414Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-09T18:39:44.8310556Z ##[command]git config gc.auto 0
2019-12-09T18:39:44.8318836Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-09T18:39:44.8324739Z ##[command]git config --get-all http.proxy
2019-12-09T18:39:44.8329384Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67116/merge:refs/remotes/pull/67116/merge
---
2019-12-09T18:48:03.7256110Z 
2019-12-09T18:48:03.7261299Z error: variant is never constructed: `Other`
2019-12-09T18:48:03.7262094Z   --> src/librustc/traits/error_reporting.rs:50:5
2019-12-09T18:48:03.7262684Z    |
2019-12-09T18:48:03.7263322Z 50 |     Other(&'a dyn fmt::Display)
2019-12-09T18:48:03.7264312Z 
2019-12-09T18:48:04.9299528Z error: aborting due to 2 previous errors
2019-12-09T18:48:04.9300465Z 
2019-12-09T18:48:05.2403839Z error: could not compile `rustc`.
---
2019-12-09T18:48:05.2515129Z   local time: Mon Dec  9 18:48:05 UTC 2019
2019-12-09T18:48:05.5700061Z   network time: Mon, 09 Dec 2019 18:48:05 GMT
2019-12-09T18:48:05.5700643Z == end clock drift check ==
2019-12-09T18:48:06.1261410Z 
2019-12-09T18:48:06.1373900Z ##[error]Bash exited with code '1'.
2019-12-09T18:48:06.1404348Z ##[section]Starting: Checkout
2019-12-09T18:48:06.1406633Z ==============================================================================
2019-12-09T18:48:06.1406716Z Task         : Get sources
2019-12-09T18:48:06.1406770Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
