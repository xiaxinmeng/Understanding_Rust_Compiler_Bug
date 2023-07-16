plain
2019-11-02T19:55:01.9269595Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-02T19:55:01.9496956Z ##[command]git config gc.auto 0
2019-11-02T19:55:02.9244378Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-02T19:55:02.9251689Z ##[command]git config --get-all http.proxy
2019-11-02T19:55:02.9257650Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66045/merge:refs/remotes/pull/66045/merge
---
2019-11-02T20:01:12.0256749Z Found 0 error codes with no tests
2019-11-02T20:01:12.0257402Z Done!
2019-11-02T20:01:12.0257436Z 
2019-11-02T20:01:12.0257486Z 
2019-11-02T20:01:12.0258285Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-02T20:01:12.0258380Z 
2019-11-02T20:01:12.0258419Z 
2019-11-02T20:01:12.0266522Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-02T20:01:12.0266604Z Build completed unsuccessfully in 0:01:20
2019-11-02T20:01:12.0266604Z Build completed unsuccessfully in 0:01:20
2019-11-02T20:01:12.0310686Z == clock drift check ==
2019-11-02T20:01:12.0322949Z   local time: Sat Nov  2 20:01:12 UTC 2019
2019-11-02T20:01:12.1036837Z   network time: Sat, 02 Nov 2019 20:01:12 GMT
2019-11-02T20:01:12.1041908Z == end clock drift check ==
2019-11-02T20:01:13.5224593Z 
2019-11-02T20:01:13.5328795Z ##[error]Bash exited with code '1'.
2019-11-02T20:01:13.5355489Z ##[section]Starting: Checkout
2019-11-02T20:01:13.5357419Z ==============================================================================
2019-11-02T20:01:13.5357488Z Task         : Get sources
2019-11-02T20:01:13.5357530Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
