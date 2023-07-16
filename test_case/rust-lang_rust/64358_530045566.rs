plain
2019-09-10T17:23:13.0157544Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T17:23:13.0356705Z ##[command]git config gc.auto 0
2019-09-10T17:23:13.0434208Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T17:23:13.0486354Z ##[command]git config --get-all http.proxy
2019-09-10T17:23:13.0645168Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64358/merge:refs/remotes/pull/64358/merge
---
2019-09-10T17:26:54.7081705Z ###################################################                       71.4%
2019-09-10T17:26:54.7428324Z #####################################################################     96.4%
2019-09-10T17:26:54.7430784Z ######################################################################## 100.0%
2019-09-10T17:26:55.1480572Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-10T17:26:55.2112365Z     Updating git repository `https://github.com/cuviper/rayon`
---
2019-09-10T17:30:29.7855488Z * highest error code: E0733
2019-09-10T17:30:30.1432526Z * 263 features
2019-09-10T17:30:30.8030413Z Dependencies not on the whitelist:
2019-09-10T17:30:30.8031282Z * crossbeam-queue 
2019-09-10T17:30:30.8031346Z invalid source: "git+https://github.com/cuviper/rayon?branch=rustc#7ffde12e0cc7b5ab2ed8bea8c6c08c3005ac4db7"
2019-09-10T17:30:30.8031423Z invalid source: "git+https://github.com/cuviper/rayon?branch=rustc#7ffde12e0cc7b5ab2ed8bea8c6c08c3005ac4db7"
2019-09-10T17:30:30.8495597Z some tidy checks failed
2019-09-10T17:30:30.8501469Z 
2019-09-10T17:30:30.8501469Z 
2019-09-10T17:30:30.8505513Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-10T17:30:30.8505732Z 
2019-09-10T17:30:30.8505808Z 
2019-09-10T17:30:30.8519880Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-10T17:30:30.8520357Z Build completed unsuccessfully in 0:01:35
2019-09-10T17:30:30.8520357Z Build completed unsuccessfully in 0:01:35
2019-09-10T17:30:30.8580734Z == clock drift check ==
2019-09-10T17:30:30.8600062Z   local time: Tue Sep 10 17:30:30 UTC 2019
2019-09-10T17:30:31.1403598Z   network time: Tue, 10 Sep 2019 17:30:31 GMT
2019-09-10T17:30:31.1407200Z == end clock drift check ==
2019-09-10T17:30:32.4425298Z ##[error]Bash exited with code '1'.
2019-09-10T17:30:32.4461074Z ##[section]Starting: Checkout
2019-09-10T17:30:32.4463317Z ==============================================================================
2019-09-10T17:30:32.4463366Z Task         : Get sources
2019-09-10T17:30:32.4463411Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
