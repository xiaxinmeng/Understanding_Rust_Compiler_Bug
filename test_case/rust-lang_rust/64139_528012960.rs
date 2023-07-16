plain
2019-09-04T17:50:10.9711955Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-04T17:50:10.9918398Z ##[command]git config gc.auto 0
2019-09-04T17:50:11.0026075Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-04T17:50:11.0081312Z ##[command]git config --get-all http.proxy
2019-09-04T17:50:11.0236076Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64139/merge:refs/remotes/pull/64139/merge
---
2019-09-04T17:53:50.7475154Z #################################################################         90.7%
2019-09-04T17:53:50.7476264Z ######################################################################## 100.0%
2019-09-04T17:53:52.1100479Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-04T17:53:52.4210867Z     Updating crates.io index
2019-09-04T17:54:27.7694363Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-04T17:54:27.7720814Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-04T17:54:27.7768715Z Makefile:67: recipe for target 'prepare' failed
2019-09-04T17:54:27.7771421Z make: *** [prepare] Error 1
2019-09-04T17:54:28.7788609Z Command failed. Attempt 2/5:
2019-09-04T17:54:28.7788609Z Command failed. Attempt 2/5:
2019-09-04T17:54:28.9878251Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-04T17:54:28.9896847Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-04T17:54:28.9953506Z Makefile:67: recipe for target 'prepare' failed
2019-09-04T17:54:28.9953622Z make: *** [prepare] Error 1
2019-09-04T17:54:30.9967413Z Command failed. Attempt 3/5:
2019-09-04T17:54:30.9967413Z Command failed. Attempt 3/5:
2019-09-04T17:54:31.2198176Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-04T17:54:31.2214377Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-04T17:54:31.2260514Z Makefile:67: recipe for target 'prepare' failed
2019-09-04T17:54:31.2264218Z make: *** [prepare] Error 1
2019-09-04T17:54:34.2281922Z Command failed. Attempt 4/5:
2019-09-04T17:54:34.2281922Z Command failed. Attempt 4/5:
2019-09-04T17:54:34.4368883Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-04T17:54:34.4386214Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-04T17:54:34.4434037Z Makefile:67: recipe for target 'prepare' failed
2019-09-04T17:54:34.4434453Z make: *** [prepare] Error 1
2019-09-04T17:54:38.4461465Z Command failed. Attempt 5/5:
2019-09-04T17:54:38.4461465Z Command failed. Attempt 5/5:
2019-09-04T17:54:38.6533562Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-04T17:54:38.6548030Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-04T17:54:38.6594443Z Makefile:67: recipe for target 'prepare' failed
2019-09-04T17:54:38.6599814Z make: *** [prepare] Error 1
2019-09-04T17:54:38.6605362Z The command has failed after 5 attempts.
2019-09-04T17:54:38.6605434Z == clock drift check ==
2019-09-04T17:54:38.6605434Z == clock drift check ==
2019-09-04T17:54:38.6611865Z   local time: Wed Sep  4 17:54:38 UTC 2019
2019-09-04T17:54:38.8181040Z   network time: Wed, 04 Sep 2019 17:54:38 GMT
2019-09-04T17:54:38.8186244Z == end clock drift check ==
2019-09-04T17:54:39.8459283Z ##[error]Bash exited with code '1'.
2019-09-04T17:54:39.8499628Z ##[section]Starting: Checkout
2019-09-04T17:54:39.8501479Z ==============================================================================
2019-09-04T17:54:39.8501553Z Task         : Get sources
2019-09-04T17:54:39.8501898Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
