plain
2019-10-08T11:17:05.0285704Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-08T11:17:05.0495783Z ##[command]git config gc.auto 0
2019-10-08T11:17:05.0588106Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-08T11:17:05.0659617Z ##[command]git config --get-all http.proxy
2019-10-08T11:17:05.0798391Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65202/merge:refs/remotes/pull/65202/merge
---
2019-10-08T11:23:47.1619789Z    Compiling serde_json v1.0.40
2019-10-08T11:23:48.9290740Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-08T11:23:59.9722143Z     Finished release [optimized] target(s) in 1m 28s
2019-10-08T11:23:59.9800235Z tidy check
2019-10-08T11:24:00.4097909Z tidy error: /checkout/src/ci/scripts/install-clang.sh:41: line longer than 100 chars
2019-10-08T11:24:00.4098515Z tidy error: /checkout/src/ci/scripts/install-innosetup.sh:14: line longer than 100 chars
2019-10-08T11:24:00.4099018Z tidy error: /checkout/src/ci/scripts/enable-docker-ipv6.sh:12: line longer than 100 chars
2019-10-08T11:24:00.4099250Z tidy error: /checkout/src/ci/scripts/install-ninja.sh:11: line longer than 100 chars
2019-10-08T11:24:00.4099474Z tidy error: /checkout/src/ci/scripts/install-mingw.sh:41: line longer than 100 chars
2019-10-08T11:24:02.1016778Z some tidy checks failed
2019-10-08T11:24:02.1022892Z 
2019-10-08T11:24:02.1022892Z 
2019-10-08T11:24:02.1023925Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-08T11:24:02.1024041Z 
2019-10-08T11:24:02.1024095Z 
2019-10-08T11:24:02.1029354Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-08T11:24:02.1029663Z Build completed unsuccessfully in 0:01:31
2019-10-08T11:24:02.1029663Z Build completed unsuccessfully in 0:01:31
2019-10-08T11:24:02.1086864Z == clock drift check ==
2019-10-08T11:24:02.1110882Z   local time: Tue Oct  8 11:24:02 UTC 2019
2019-10-08T11:24:02.2600013Z   network time: Tue, 08 Oct 2019 11:24:02 GMT
2019-10-08T11:24:02.2603874Z == end clock drift check ==
2019-10-08T11:24:03.6450517Z ##[error]Bash exited with code '1'.
2019-10-08T11:24:03.6500949Z ##[section]Starting: Checkout
2019-10-08T11:24:03.6503139Z ==============================================================================
2019-10-08T11:24:03.6503201Z Task         : Get sources
2019-10-08T11:24:03.6503273Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
