plain
2019-09-25T21:44:45.2246006Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T21:44:45.2443777Z ##[command]git config gc.auto 0
2019-09-25T21:44:45.2531771Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T21:44:45.2618325Z ##[command]git config --get-all http.proxy
2019-09-25T21:44:45.2775012Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64785/merge:refs/remotes/pull/64785/merge
---
2019-09-25T21:48:13.6508338Z ##########################                                                37.5%
2019-09-25T21:48:13.6508616Z ######################################################################## 100.0%
2019-09-25T21:48:14.6703582Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-25T21:48:14.7647608Z     Updating crates.io index
2019-09-25T21:48:51.9620953Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-25T21:48:51.9655933Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T21:48:51.9699515Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T21:48:51.9700220Z make: *** [prepare] Error 1
2019-09-25T21:48:52.9723406Z Command failed. Attempt 2/5:
2019-09-25T21:48:53.0926043Z     Updating crates.io index
2019-09-25T21:48:53.0926043Z     Updating crates.io index
2019-09-25T21:48:53.4743105Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-25T21:48:53.4772426Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T21:48:53.4827851Z make: *** [prepare] Error 1
2019-09-25T21:48:53.4829078Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T21:48:55.4851394Z Command failed. Attempt 3/5:
2019-09-25T21:48:55.6144790Z     Updating crates.io index
2019-09-25T21:48:55.6144790Z     Updating crates.io index
2019-09-25T21:48:56.0068975Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-25T21:48:56.0096798Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T21:48:56.0149859Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T21:48:56.0150002Z make: *** [prepare] Error 1
2019-09-25T21:48:59.0161492Z Command failed. Attempt 4/5:
2019-09-25T21:48:59.1363256Z     Updating crates.io index
2019-09-25T21:48:59.1363256Z     Updating crates.io index
2019-09-25T21:48:59.5051292Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-25T21:48:59.5080237Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T21:48:59.5123159Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T21:48:59.5126969Z make: *** [prepare] Error 1
2019-09-25T21:49:03.5141243Z Command failed. Attempt 5/5:
2019-09-25T21:49:03.6359879Z     Updating crates.io index
2019-09-25T21:49:03.6359879Z     Updating crates.io index
2019-09-25T21:49:03.9890504Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-25T21:49:03.9912942Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T21:49:03.9957038Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T21:49:03.9957643Z make: *** [prepare] Error 1
2019-09-25T21:49:03.9962217Z The command has failed after 5 attempts.
2019-09-25T21:49:03.9966668Z == clock drift check ==
2019-09-25T21:49:03.9966668Z == clock drift check ==
2019-09-25T21:49:03.9978372Z   local time: Wed Sep 25 21:49:03 UTC 2019
2019-09-25T21:49:04.1477791Z   network time: Wed, 25 Sep 2019 21:49:04 GMT
2019-09-25T21:49:04.1481486Z == end clock drift check ==
2019-09-25T21:49:05.2070296Z ##[error]Bash exited with code '1'.
2019-09-25T21:49:05.2113693Z ##[section]Starting: Checkout
2019-09-25T21:49:05.2116055Z ==============================================================================
2019-09-25T21:49:05.2116140Z Task         : Get sources
2019-09-25T21:49:05.2116188Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
