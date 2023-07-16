plain
2019-12-03T21:24:43.5696387Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-03T21:24:43.5886804Z ##[command]git config gc.auto 0
2019-12-03T21:24:43.5965970Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-03T21:24:43.6039059Z ##[command]git config --get-all http.proxy
2019-12-03T21:24:43.6201055Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66984/merge:refs/remotes/pull/66984/merge
---
2019-12-03T21:30:50.7788214Z    Compiling serde_json v1.0.40
2019-12-03T21:30:52.4907392Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-03T21:31:03.5937623Z     Finished release [optimized] target(s) in 1m 26s
2019-12-03T21:31:03.6043379Z tidy check
2019-12-03T21:31:04.2640034Z tidy error: /checkout/src/librustdoc/clean/types.rs: ignoring file length unnecessarily
2019-12-03T21:31:06.3856586Z some tidy checks failed
2019-12-03T21:31:06.3856763Z Found 486 error codes
2019-12-03T21:31:06.3856818Z Found 0 error codes with no tests
2019-12-03T21:31:06.3856872Z Done!
2019-12-03T21:31:06.3856872Z Done!
2019-12-03T21:31:06.3856924Z 
2019-12-03T21:31:06.3856970Z 
2019-12-03T21:31:06.3857964Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-03T21:31:06.3858119Z 
2019-12-03T21:31:06.3858149Z 
2019-12-03T21:31:06.3863958Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-03T21:31:06.3864125Z Build completed unsuccessfully in 0:01:30
2019-12-03T21:31:06.3864125Z Build completed unsuccessfully in 0:01:30
2019-12-03T21:31:06.3914997Z == clock drift check ==
2019-12-03T21:31:06.3937764Z   local time: Tue Dec  3 21:31:06 UTC 2019
2019-12-03T21:31:06.5405052Z   network time: Tue, 03 Dec 2019 21:31:06 GMT
2019-12-03T21:31:06.5405173Z == end clock drift check ==
2019-12-03T21:31:07.8509744Z 
2019-12-03T21:31:07.8612331Z ##[error]Bash exited with code '1'.
2019-12-03T21:31:07.8642762Z ##[section]Starting: Checkout
2019-12-03T21:31:07.8644556Z ==============================================================================
2019-12-03T21:31:07.8644621Z Task         : Get sources
2019-12-03T21:31:07.8644692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
