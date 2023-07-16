plain
2019-11-30T15:40:40.2961667Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T15:40:41.1149840Z ##[command]git config gc.auto 0
2019-11-30T15:40:41.1156193Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T15:40:41.1162227Z ##[command]git config --get-all http.proxy
2019-11-30T15:40:41.1167033Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66606/merge:refs/remotes/pull/66606/merge
---
2019-11-30T15:46:50.9251004Z * 589 error codes
2019-11-30T15:46:50.9251196Z * highest error code: E0745
2019-11-30T15:46:51.3264787Z * 274 features
2019-11-30T15:46:52.0876202Z Checking which error codes lack tests...
2019-11-30T15:46:52.2740654Z Couldn't read `/checkout/src/librustc_error_codes/./error_codes/E0017.md`: No such file or directory (os error 2)
2019-11-30T15:46:52.2958892Z Error code E0017 needs to have at least one UI test!
2019-11-30T15:46:52.2959086Z some tidy checks failed
2019-11-30T15:46:52.2959188Z Found 1 error codes with no tests
2019-11-30T15:46:52.2959274Z Done!
2019-11-30T15:46:52.2959307Z 
2019-11-30T15:46:52.2959337Z 
2019-11-30T15:46:52.2959337Z 
2019-11-30T15:46:52.2960201Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-30T15:46:52.2960348Z 
2019-11-30T15:46:52.2960377Z 
2019-11-30T15:46:52.2960450Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-30T15:46:52.2960508Z Build completed unsuccessfully in 0:01:31
2019-11-30T15:46:52.2960508Z Build completed unsuccessfully in 0:01:31
2019-11-30T15:46:52.3010060Z == clock drift check ==
2019-11-30T15:46:52.3032606Z   local time: Sat Nov 30 15:46:52 UTC 2019
2019-11-30T15:46:52.3858025Z   network time: Sat, 30 Nov 2019 15:46:52 GMT
2019-11-30T15:46:52.3878029Z == end clock drift check ==
2019-11-30T15:46:53.6696858Z 
2019-11-30T15:46:53.6805044Z ##[error]Bash exited with code '1'.
2019-11-30T15:46:53.6862199Z ##[section]Starting: Checkout
2019-11-30T15:46:53.6864415Z ==============================================================================
2019-11-30T15:46:53.6864478Z Task         : Get sources
2019-11-30T15:46:53.6864531Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
