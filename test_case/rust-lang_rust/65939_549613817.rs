plain
2019-11-05T00:44:07.7977159Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-05T00:44:07.8177692Z ##[command]git config gc.auto 0
2019-11-05T00:44:07.8239742Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-05T00:44:07.8291622Z ##[command]git config --get-all http.proxy
2019-11-05T00:44:07.8420998Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65939/merge:refs/remotes/pull/65939/merge
---
2019-11-05T00:47:46.5154829Z 
2019-11-05T00:47:46.5169053Z ######################################################################## 100.0%
2019-11-05T00:47:46.8214111Z extracting /checkout/obj/build/cache/2019-09-25/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2019-11-05T00:47:46.9018105Z     Updating crates.io index
2019-11-05T00:47:52.7617897Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-05T00:47:52.7649877Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-05T00:47:52.7704479Z Makefile:67: recipe for target 'prepare' failed
2019-11-05T00:47:52.7709241Z make: *** [prepare] Error 1
2019-11-05T00:47:53.7717110Z Command failed. Attempt 2/5:
2019-11-05T00:47:53.7717110Z Command failed. Attempt 2/5:
2019-11-05T00:47:53.9901599Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-05T00:47:53.9923869Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-05T00:47:53.9973638Z Makefile:67: recipe for target 'prepare' failed
2019-11-05T00:47:53.9973758Z make: *** [prepare] Error 1
2019-11-05T00:47:55.9991411Z Command failed. Attempt 3/5:
2019-11-05T00:47:55.9991411Z Command failed. Attempt 3/5:
2019-11-05T00:47:56.2228215Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-05T00:47:56.2238778Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-05T00:47:56.2283317Z make: *** [prepare] Error 1
2019-11-05T00:47:56.2287081Z Makefile:67: recipe for target 'prepare' failed
2019-11-05T00:47:59.2323435Z Command failed. Attempt 4/5:
2019-11-05T00:47:59.2323435Z Command failed. Attempt 4/5:
2019-11-05T00:47:59.4459506Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-05T00:47:59.4481606Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-05T00:47:59.4533329Z Makefile:67: recipe for target 'prepare' failed
2019-11-05T00:47:59.4533442Z make: *** [prepare] Error 1
2019-11-05T00:48:03.4562157Z Command failed. Attempt 5/5:
2019-11-05T00:48:03.4562157Z Command failed. Attempt 5/5:
2019-11-05T00:48:03.6732464Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-05T00:48:03.6744045Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-05T00:48:03.6797816Z Makefile:67: recipe for target 'prepare' failed
2019-11-05T00:48:03.6797933Z make: *** [prepare] Error 1
2019-11-05T00:48:03.6797984Z The command has failed after 5 attempts.
2019-11-05T00:48:03.6798032Z == clock drift check ==
2019-11-05T00:48:03.6798032Z == clock drift check ==
2019-11-05T00:48:03.6805571Z   local time: Tue Nov  5 00:48:03 UTC 2019
2019-11-05T00:48:03.8338051Z   network time: Tue, 05 Nov 2019 00:48:03 GMT
2019-11-05T00:48:03.8338708Z == end clock drift check ==
2019-11-05T00:48:08.8328181Z 
2019-11-05T00:48:08.8397410Z ##[error]Bash exited with code '1'.
2019-11-05T00:48:08.8427894Z ##[section]Starting: Checkout
2019-11-05T00:48:08.8429596Z ==============================================================================
2019-11-05T00:48:08.8429670Z Task         : Get sources
2019-11-05T00:48:08.8429736Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
