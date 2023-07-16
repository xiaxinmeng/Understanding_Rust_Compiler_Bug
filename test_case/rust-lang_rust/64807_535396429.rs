plain
2019-09-26T08:18:49.8785210Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T08:18:49.8992634Z ##[command]git config gc.auto 0
2019-09-26T08:18:49.9073029Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T08:18:49.9142050Z ##[command]git config --get-all http.proxy
2019-09-26T08:18:49.9269291Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64807/merge:refs/remotes/pull/64807/merge
---
2019-09-26T08:21:47.6553149Z #############################################################             84.9%
2019-09-26T08:21:47.6553519Z ######################################################################## 100.0%
2019-09-26T08:21:48.0344662Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-26T08:21:48.1088406Z     Updating crates.io index
2019-09-26T08:22:22.6002917Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-26T08:22:22.6003659Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-26T08:22:22.6021852Z Makefile:67: recipe for target 'prepare' failed
2019-09-26T08:22:22.6022198Z make: *** [prepare] Error 1
2019-09-26T08:22:23.0655369Z Command failed. Attempt 2/5:
2019-09-26T08:22:23.0655369Z Command failed. Attempt 2/5:
2019-09-26T08:22:23.2837018Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-26T08:22:23.2853288Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-26T08:22:23.2896993Z Makefile:67: recipe for target 'prepare' failed
2019-09-26T08:22:23.2901367Z make: *** [prepare] Error 1
2019-09-26T08:22:25.2929103Z Command failed. Attempt 3/5:
2019-09-26T08:22:25.2929103Z Command failed. Attempt 3/5:
2019-09-26T08:22:25.5005216Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-26T08:22:25.5024243Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-26T08:22:25.5070417Z Makefile:67: recipe for target 'prepare' failed
2019-09-26T08:22:25.5075602Z make: *** [prepare] Error 1
2019-09-26T08:22:28.5086931Z Command failed. Attempt 4/5:
2019-09-26T08:22:28.5086931Z Command failed. Attempt 4/5:
2019-09-26T08:22:28.7045439Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-26T08:22:28.7062442Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-26T08:22:28.7103284Z Makefile:67: recipe for target 'prepare' failed
2019-09-26T08:22:28.7108401Z make: *** [prepare] Error 1
2019-09-26T08:22:32.7119457Z Command failed. Attempt 5/5:
2019-09-26T08:22:32.7119457Z Command failed. Attempt 5/5:
2019-09-26T08:22:32.9057642Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-26T08:22:32.9070707Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-26T08:22:32.9113776Z Makefile:67: recipe for target 'prepare' failed
2019-09-26T08:22:32.9116933Z make: *** [prepare] Error 1
2019-09-26T08:22:32.9120600Z The command has failed after 5 attempts.
2019-09-26T08:22:32.9120940Z == clock drift check ==
2019-09-26T08:22:32.9120940Z == clock drift check ==
2019-09-26T08:22:32.9126142Z   local time: Thu Sep 26 08:22:32 UTC 2019
2019-09-26T08:22:33.1894375Z   network time: Thu, 26 Sep 2019 08:22:33 GMT
2019-09-26T08:22:33.1895164Z == end clock drift check ==
2019-09-26T08:22:34.5149494Z ##[error]Bash exited with code '1'.
2019-09-26T08:22:34.5180864Z ##[section]Starting: Checkout
2019-09-26T08:22:34.5183496Z ==============================================================================
2019-09-26T08:22:34.5183552Z Task         : Get sources
2019-09-26T08:22:34.5183647Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
