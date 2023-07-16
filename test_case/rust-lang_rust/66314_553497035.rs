plain
2019-11-13T17:02:12.5254453Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-13T17:02:13.3023164Z ##[command]git config gc.auto 0
2019-11-13T17:02:13.3027337Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-13T17:02:13.3028933Z ##[command]git config --get-all http.proxy
2019-11-13T17:02:13.3036495Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66314/merge:refs/remotes/pull/66314/merge
---
2019-11-13T17:05:15.1073939Z 
2019-11-13T17:05:15.1074707Z ######################################################################## 100.0%
2019-11-13T17:05:15.4094561Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-13T17:05:15.4721266Z     Updating crates.io index
2019-11-13T17:05:20.6012781Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-13T17:05:20.6039458Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-13T17:05:20.6080117Z make: *** [prepare] Error 1
2019-11-13T17:05:20.6080922Z Makefile:67: recipe for target 'prepare' failed
2019-11-13T17:05:21.6109683Z Command failed. Attempt 2/5:
2019-11-13T17:05:21.6109683Z Command failed. Attempt 2/5:
2019-11-13T17:05:21.7710280Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-13T17:05:21.7724394Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-13T17:05:21.7776183Z Makefile:67: recipe for target 'prepare' failed
2019-11-13T17:05:21.7776254Z make: *** [prepare] Error 1
2019-11-13T17:05:23.7797070Z Command failed. Attempt 3/5:
2019-11-13T17:05:23.7797070Z Command failed. Attempt 3/5:
2019-11-13T17:05:23.9529870Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-13T17:05:23.9539970Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-13T17:05:23.9574439Z Makefile:67: recipe for target 'prepare' failed
2019-11-13T17:05:23.9576830Z make: *** [prepare] Error 1
2019-11-13T17:05:26.9588238Z Command failed. Attempt 4/5:
2019-11-13T17:05:26.9588238Z Command failed. Attempt 4/5:
2019-11-13T17:05:27.1512211Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-13T17:05:27.1522422Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-13T17:05:27.1559639Z Makefile:67: recipe for target 'prepare' failed
2019-11-13T17:05:27.1562071Z make: *** [prepare] Error 1
2019-11-13T17:05:31.1575692Z Command failed. Attempt 5/5:
2019-11-13T17:05:31.1575692Z Command failed. Attempt 5/5:
2019-11-13T17:05:31.3200091Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-13T17:05:31.3212175Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-13T17:05:31.3264899Z Makefile:67: recipe for target 'prepare' failed
2019-11-13T17:05:31.3283712Z The command has failed after 5 attempts.
2019-11-13T17:05:31.3283851Z == clock drift check ==
2019-11-13T17:05:31.3283975Z   local time: Wed Nov 13 17:05:31 UTC 2019
2019-11-13T17:05:31.3283975Z   local time: Wed Nov 13 17:05:31 UTC 2019
2019-11-13T17:05:31.3284032Z   network time: make: *** [prepare] Error 1
2019-11-13T17:05:31.3483590Z Wed, 13 Nov 2019 17:05:31 GMT
2019-11-13T17:05:31.3485859Z == end clock drift check ==
2019-11-13T17:05:43.4925006Z 
2019-11-13T17:05:43.5021277Z ##[error]Bash exited with code '1'.
2019-11-13T17:05:43.5044871Z ##[section]Starting: Checkout
2019-11-13T17:05:43.5046431Z ==============================================================================
2019-11-13T17:05:43.5046474Z Task         : Get sources
2019-11-13T17:05:43.5046511Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
