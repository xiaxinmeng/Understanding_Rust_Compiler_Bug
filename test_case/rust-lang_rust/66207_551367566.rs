plain
2019-11-08T03:16:33.4249800Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T03:16:33.4426385Z ##[command]git config gc.auto 0
2019-11-08T03:16:33.4508848Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T03:16:33.4560407Z ##[command]git config --get-all http.proxy
2019-11-08T03:16:33.4711374Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66207/merge:refs/remotes/pull/66207/merge
---
2019-11-08T03:19:56.4540482Z ##############                                                            20.1%
2019-11-08T03:19:56.4541293Z ######################################################################## 100.0%
2019-11-08T03:20:02.1263846Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-08T03:20:02.2098443Z     Updating crates.io index
2019-11-08T03:20:07.8889674Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:20:07.8920803Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:20:07.8964955Z make: *** [prepare] Error 1
2019-11-08T03:20:07.8966449Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:20:08.8985045Z Command failed. Attempt 2/5:
2019-11-08T03:20:08.8985045Z Command failed. Attempt 2/5:
2019-11-08T03:20:09.0951442Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:20:09.0968774Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:20:09.1012081Z make: *** [prepare] Error 1
2019-11-08T03:20:09.1012933Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:20:11.1251179Z Command failed. Attempt 3/5:
2019-11-08T03:20:11.1251179Z Command failed. Attempt 3/5:
2019-11-08T03:20:11.3034796Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:20:11.3045253Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:20:11.3086968Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:20:11.3091863Z make: *** [prepare] Error 1
2019-11-08T03:20:14.3106972Z Command failed. Attempt 4/5:
2019-11-08T03:20:14.3106972Z Command failed. Attempt 4/5:
2019-11-08T03:20:14.5197193Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:20:14.5210636Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:20:14.5256025Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:20:14.5256570Z make: *** [prepare] Error 1
2019-11-08T03:20:18.5272076Z Command failed. Attempt 5/5:
2019-11-08T03:20:18.5272076Z Command failed. Attempt 5/5:
2019-11-08T03:20:18.7460768Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:20:18.7475402Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:20:18.7522984Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:20:18.7523766Z make: *** [prepare] Error 1
2019-11-08T03:20:18.7524422Z The command has failed after 5 attempts.
2019-11-08T03:20:18.7524706Z == clock drift check ==
2019-11-08T03:20:18.7524706Z == clock drift check ==
2019-11-08T03:20:18.7531940Z   local time: Fri Nov  8 03:20:18 UTC 2019
2019-11-08T03:20:18.7885671Z   network time: Fri, 08 Nov 2019 03:20:18 GMT
2019-11-08T03:20:18.7890122Z == end clock drift check ==
2019-11-08T03:20:25.6786745Z 
2019-11-08T03:20:25.6920156Z ##[error]Bash exited with code '1'.
2019-11-08T03:20:25.6949431Z ##[section]Starting: Checkout
2019-11-08T03:20:25.6951437Z ==============================================================================
2019-11-08T03:20:25.6951482Z Task         : Get sources
2019-11-08T03:20:25.6951523Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
