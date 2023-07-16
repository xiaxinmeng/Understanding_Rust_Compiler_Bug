plain
2019-09-25T12:00:06.2748624Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T12:00:06.2930042Z ##[command]git config gc.auto 0
2019-09-25T12:00:06.3016422Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T12:00:06.3079358Z ##[command]git config --get-all http.proxy
2019-09-25T12:00:06.3224855Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64763/merge:refs/remotes/pull/64763/merge
---
2019-09-25T12:07:32.8981132Z     Finished release [optimized] target(s) in 1m 32s
2019-09-25T12:07:32.9067197Z tidy check
2019-09-25T12:07:33.9818003Z * 579 error codes
2019-09-25T12:07:33.9818743Z * highest error code: E0734
2019-09-25T12:07:34.0806413Z tidy error: /checkout/src/librustc/error_codes.rs:2228: malformed stability attribute: can't parse `since` key
2019-09-25T12:07:35.0307682Z some tidy checks failed
2019-09-25T12:07:35.0313434Z 
2019-09-25T12:07:35.0313434Z 
2019-09-25T12:07:35.0315782Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-25T12:07:35.0318958Z 
2019-09-25T12:07:35.0319307Z 
2019-09-25T12:07:35.0333972Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-25T12:07:35.0334095Z Build completed unsuccessfully in 0:01:35
2019-09-25T12:07:35.0334095Z Build completed unsuccessfully in 0:01:35
2019-09-25T12:07:35.0385728Z == clock drift check ==
2019-09-25T12:07:35.0407283Z   local time: Wed Sep 25 12:07:35 UTC 2019
2019-09-25T12:07:35.1902227Z   network time: Wed, 25 Sep 2019 12:07:35 GMT
2019-09-25T12:07:35.1906695Z == end clock drift check ==
2019-09-25T12:07:36.5107261Z ##[error]Bash exited with code '1'.
2019-09-25T12:07:36.5138556Z ##[section]Starting: Checkout
2019-09-25T12:07:36.5140368Z ==============================================================================
2019-09-25T12:07:36.5140455Z Task         : Get sources
2019-09-25T12:07:36.5140498Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
