plain
2020-01-07T06:30:41.0799674Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T06:30:41.0813532Z ##[command]git config gc.auto 0
2020-01-07T06:30:41.0816831Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T06:30:41.0820428Z ##[command]git config --get-all http.proxy
2020-01-07T06:30:41.0824467Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67965/merge:refs/remotes/pull/67965/merge
---
2020-01-07T06:33:34.2964768Z ########################################                                  56.5%
2020-01-07T06:33:34.3552198Z ############################################################              84.0%
2020-01-07T06:33:34.3552400Z ######################################################################## 100.0%
2020-01-07T06:33:34.6571275Z extracting /checkout/obj/build/cache/2019-12-18/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-01-07T06:33:34.7262909Z     Updating git repository `https://github.com/Zoxc/rayon.git`
---
2020-01-07T06:36:57.5663635Z tidy check
2020-01-07T06:36:59.2021018Z * 588 error codes
2020-01-07T06:36:59.2021185Z * highest error code: E0745
2020-01-07T06:36:59.6208033Z * 275 features
2020-01-07T06:37:00.3576084Z invalid source: "git+https://github.com/Zoxc/rayon.git?branch=single-lifetime-scope#f5d9d8b52dbfc64728d3a42ed3f7292e2eb6ee6b"
2020-01-07T06:37:00.6503019Z Found 486 error codes
2020-01-07T06:37:00.6504157Z Found 0 error codes with no tests
2020-01-07T06:37:00.6504240Z Done!
2020-01-07T06:37:00.6504294Z some tidy checks failed
2020-01-07T06:37:00.6504294Z some tidy checks failed
2020-01-07T06:37:00.6511032Z 
2020-01-07T06:37:00.6511272Z 
2020-01-07T06:37:00.6512314Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-07T06:37:00.6512717Z 
2020-01-07T06:37:00.6512831Z 
2020-01-07T06:37:00.6522453Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-07T06:37:00.6522535Z Build completed unsuccessfully in 0:01:39
2020-01-07T06:37:00.6522535Z Build completed unsuccessfully in 0:01:39
2020-01-07T06:37:00.6572914Z == clock drift check ==
2020-01-07T06:37:00.6585512Z   local time: Tue Jan  7 06:37:00 UTC 2020
2020-01-07T06:37:00.9387522Z   network time: Tue, 07 Jan 2020 06:37:00 GMT
2020-01-07T06:37:00.9387656Z == end clock drift check ==
2020-01-07T06:37:01.6911751Z 
2020-01-07T06:37:01.7033365Z ##[error]Bash exited with code '1'.
2020-01-07T06:37:01.7067686Z ##[section]Starting: Checkout
2020-01-07T06:37:01.7069356Z ==============================================================================
2020-01-07T06:37:01.7069412Z Task         : Get sources
2020-01-07T06:37:01.7069460Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
