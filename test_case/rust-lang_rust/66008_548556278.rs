plain
2019-10-31T20:27:05.7455743Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-31T20:27:06.4592301Z ##[command]git config gc.auto 0
2019-10-31T20:27:06.4594583Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-31T20:27:06.4597893Z ##[command]git config --get-all http.proxy
2019-10-31T20:27:06.4601494Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66008/merge:refs/remotes/pull/66008/merge
---
2019-10-31T20:30:38.9177689Z #####################################                                     52.0%
2019-10-31T20:30:38.9177762Z ######################################################################## 100.0%
2019-10-31T20:30:41.9535877Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-10-31T20:30:42.0133944Z     Updating crates.io index
2019-10-31T20:30:46.1710603Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-31T20:30:46.1737887Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-31T20:30:46.1776608Z Makefile:67: recipe for target 'prepare' failed
2019-10-31T20:30:46.1777860Z make: *** [prepare] Error 1
2019-10-31T20:30:47.1803935Z Command failed. Attempt 2/5:
2019-10-31T20:30:47.1803935Z Command failed. Attempt 2/5:
2019-10-31T20:30:47.3371209Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-31T20:30:47.3383254Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-31T20:30:47.3428655Z Makefile:67: recipe for target 'prepare' failed
2019-10-31T20:30:47.3428756Z make: *** [prepare] Error 1
2019-10-31T20:30:49.3443583Z Command failed. Attempt 3/5:
2019-10-31T20:30:49.3443583Z Command failed. Attempt 3/5:
2019-10-31T20:30:49.5064897Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-31T20:30:49.5076237Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-31T20:30:49.5113750Z Makefile:67: recipe for target 'prepare' failed
2019-10-31T20:30:49.5113863Z make: *** [prepare] Error 1
2019-10-31T20:30:52.5135134Z Command failed. Attempt 4/5:
2019-10-31T20:30:52.5135134Z Command failed. Attempt 4/5:
2019-10-31T20:30:53.5282350Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-31T20:30:53.5283468Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-31T20:30:53.5283868Z Makefile:67: recipe for target 'prepare' failed
2019-10-31T20:30:53.5283941Z make: *** [prepare] Error 1
2019-10-31T20:30:56.6833749Z Command failed. Attempt 5/5:
2019-10-31T20:30:56.6833749Z Command failed. Attempt 5/5:
2019-10-31T20:30:56.8429806Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-31T20:30:56.8447077Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-31T20:30:56.8486795Z Makefile:67: recipe for target 'prepare' failed
2019-10-31T20:30:56.8487218Z make: *** [prepare] Error 1
2019-10-31T20:30:56.8490399Z The command has failed after 5 attempts.
2019-10-31T20:30:56.8532968Z == clock drift check ==
2019-10-31T20:30:56.8532968Z == clock drift check ==
2019-10-31T20:30:56.8533554Z   local time: Thu Oct 31 20:30:56 UTC 2019
2019-10-31T20:30:56.8646525Z   network time: Thu, 31 Oct 2019 20:30:56 GMT
2019-10-31T20:30:56.8649010Z == end clock drift check ==
2019-10-31T20:31:08.3099675Z 
2019-10-31T20:31:08.3183336Z ##[error]Bash exited with code '1'.
2019-10-31T20:31:08.3208468Z ##[section]Starting: Checkout
2019-10-31T20:31:08.3210438Z ==============================================================================
2019-10-31T20:31:08.3210505Z Task         : Get sources
2019-10-31T20:31:08.3210543Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
