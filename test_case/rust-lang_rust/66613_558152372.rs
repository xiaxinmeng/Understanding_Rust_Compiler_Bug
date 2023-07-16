plain
2019-11-25T13:16:58.3479716Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T13:16:58.3492747Z ##[command]git config gc.auto 0
2019-11-25T13:16:58.3497449Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T13:16:58.3501950Z ##[command]git config --get-all http.proxy
2019-11-25T13:16:58.3507473Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66613/merge:refs/remotes/pull/66613/merge
---
2019-11-25T13:20:13.9565001Z ############################################################              84.3%
2019-11-25T13:20:13.9566792Z ######################################################################## 100.0%
2019-11-25T13:20:14.3440015Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-25T13:20:14.4227807Z     Updating crates.io index
2019-11-25T13:20:21.1660215Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T13:20:21.1691282Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T13:20:21.1738283Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T13:20:21.1742261Z make: *** [prepare] Error 1
2019-11-25T13:20:22.1751503Z Command failed. Attempt 2/5:
2019-11-25T13:20:22.1751503Z Command failed. Attempt 2/5:
2019-11-25T13:20:22.3906071Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T13:20:22.3922760Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T13:20:22.3965801Z make: *** [prepare] Error 1
2019-11-25T13:20:22.3966564Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T13:20:24.4016160Z Command failed. Attempt 3/5:
2019-11-25T13:20:24.4016160Z Command failed. Attempt 3/5:
2019-11-25T13:20:24.6046536Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T13:20:24.6067626Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T13:20:24.6118287Z make: *** [prepare] Error 1
2019-11-25T13:20:24.6118754Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T13:20:27.6134252Z Command failed. Attempt 4/5:
2019-11-25T13:20:27.6134252Z Command failed. Attempt 4/5:
2019-11-25T13:20:27.8205772Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T13:20:27.8218688Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T13:20:27.8266450Z make: *** [prepare] Error 1
2019-11-25T13:20:27.8267069Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T13:20:31.8301895Z Command failed. Attempt 5/5:
2019-11-25T13:20:31.8301895Z Command failed. Attempt 5/5:
2019-11-25T13:20:32.0330076Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T13:20:32.0345080Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T13:20:32.0391072Z make: *** [prepare] Error 1
2019-11-25T13:20:32.0392121Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T13:20:32.0394915Z The command has failed after 5 attempts.
2019-11-25T13:20:32.0398031Z == clock drift check ==
2019-11-25T13:20:32.0398031Z == clock drift check ==
2019-11-25T13:20:32.0405624Z   local time: Mon Nov 25 13:20:32 UTC 2019
2019-11-25T13:20:32.0690361Z   network time: Mon, 25 Nov 2019 13:20:32 GMT
2019-11-25T13:20:32.0693624Z == end clock drift check ==
2019-11-25T13:20:45.3561600Z 
2019-11-25T13:20:45.3659640Z ##[error]Bash exited with code '1'.
2019-11-25T13:20:45.3688717Z ##[section]Starting: Checkout
2019-11-25T13:20:45.3690504Z ==============================================================================
2019-11-25T13:20:45.3690562Z Task         : Get sources
2019-11-25T13:20:45.3690609Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
