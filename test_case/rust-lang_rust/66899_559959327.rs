plain
2019-11-30T12:18:55.3728773Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T12:18:56.1855873Z ##[command]git config gc.auto 0
2019-11-30T12:18:56.1858187Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T12:18:56.1859719Z ##[command]git config --get-all http.proxy
2019-11-30T12:18:56.1863131Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66899/merge:refs/remotes/pull/66899/merge
---
2019-11-30T12:22:01.7358905Z 
2019-11-30T12:22:01.7359137Z ######################################################################## 100.0%
2019-11-30T12:22:02.0774549Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-30T12:22:02.1467530Z     Updating crates.io index
2019-11-30T12:22:09.1715736Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T12:22:09.1743593Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T12:22:09.1789754Z make: *** [prepare] Error 1
2019-11-30T12:22:09.1790539Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T12:22:10.1809697Z Command failed. Attempt 2/5:
2019-11-30T12:22:10.1809697Z Command failed. Attempt 2/5:
2019-11-30T12:22:10.3665068Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T12:22:10.3681239Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T12:22:10.3721755Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T12:22:10.3721821Z make: *** [prepare] Error 1
2019-11-30T12:22:12.3736968Z Command failed. Attempt 3/5:
2019-11-30T12:22:12.3736968Z Command failed. Attempt 3/5:
2019-11-30T12:22:12.5543169Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T12:22:12.5556228Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T12:22:12.5593612Z make: *** [prepare] Error 1
2019-11-30T12:22:12.5594238Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T12:22:15.5616289Z Command failed. Attempt 4/5:
2019-11-30T12:22:15.5616289Z Command failed. Attempt 4/5:
2019-11-30T12:22:15.7440508Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T12:22:15.7446619Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T12:22:15.7489682Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T12:22:15.7490094Z make: *** [prepare] Error 1
2019-11-30T12:22:19.7504716Z Command failed. Attempt 5/5:
2019-11-30T12:22:19.7504716Z Command failed. Attempt 5/5:
2019-11-30T12:22:19.9394611Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T12:22:19.9404811Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T12:22:19.9450529Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T12:22:19.9450897Z make: *** [prepare] Error 1
2019-11-30T12:22:19.9453152Z The command has failed after 5 attempts.
2019-11-30T12:22:19.9455524Z == clock drift check ==
2019-11-30T12:22:19.9455524Z == clock drift check ==
2019-11-30T12:22:19.9463662Z   local time: Sat Nov 30 12:22:19 UTC 2019
2019-11-30T12:22:20.0079656Z   network time: Sat, 30 Nov 2019 12:22:20 GMT
2019-11-30T12:22:20.0082403Z == end clock drift check ==
2019-11-30T12:22:32.2303698Z 
2019-11-30T12:22:32.2406350Z ##[error]Bash exited with code '1'.
2019-11-30T12:22:32.2432338Z ##[section]Starting: Checkout
2019-11-30T12:22:32.2434627Z ==============================================================================
2019-11-30T12:22:32.2434919Z Task         : Get sources
2019-11-30T12:22:32.2434964Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
