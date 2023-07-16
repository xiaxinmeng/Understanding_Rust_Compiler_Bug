plain
2019-11-04T02:46:56.0385550Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-04T02:46:56.0574423Z ##[command]git config gc.auto 0
2019-11-04T02:46:56.0673836Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-04T02:46:56.0716626Z ##[command]git config --get-all http.proxy
2019-11-04T02:46:56.0863809Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66072/merge:refs/remotes/pull/66072/merge
---
2019-11-04T02:50:49.8054760Z ################################################################          89.7%
2019-11-04T02:50:49.8055307Z ######################################################################## 100.0%
2019-11-04T02:50:50.2408111Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-04T02:50:50.3231274Z     Updating crates.io index
2019-11-04T02:50:55.4203406Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-04T02:50:55.4234082Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-04T02:50:55.4287219Z make: *** [prepare] Error 1
2019-11-04T02:50:55.4288337Z Makefile:67: recipe for target 'prepare' failed
2019-11-04T02:50:56.4307097Z Command failed. Attempt 2/5:
2019-11-04T02:50:56.4307097Z Command failed. Attempt 2/5:
2019-11-04T02:50:56.6368765Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-04T02:50:56.6385152Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-04T02:50:56.6428446Z Makefile:67: recipe for target 'prepare' failed
2019-11-04T02:50:56.6428544Z make: *** [prepare] Error 1
2019-11-04T02:50:58.6445168Z Command failed. Attempt 3/5:
2019-11-04T02:50:58.6445168Z Command failed. Attempt 3/5:
2019-11-04T02:50:58.8776385Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-04T02:50:58.8797376Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-04T02:50:58.8846721Z Makefile:67: recipe for target 'prepare' failed
2019-11-04T02:50:58.8852213Z make: *** [prepare] Error 1
2019-11-04T02:51:01.8870858Z Command failed. Attempt 4/5:
2019-11-04T02:51:01.8870858Z Command failed. Attempt 4/5:
2019-11-04T02:51:02.1210413Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-04T02:51:02.1228377Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-04T02:51:02.1280685Z Makefile:67: recipe for target 'prepare' failed
2019-11-04T02:51:02.1280778Z make: *** [prepare] Error 1
2019-11-04T02:51:06.1301036Z Command failed. Attempt 5/5:
2019-11-04T02:51:06.1301036Z Command failed. Attempt 5/5:
2019-11-04T02:51:06.3416889Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-04T02:51:06.3431581Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-04T02:51:06.3483143Z Makefile:67: recipe for target 'prepare' failed
2019-11-04T02:51:06.3488413Z make: *** [prepare] Error 1
2019-11-04T02:51:06.3501450Z The command has failed after 5 attempts.
2019-11-04T02:51:06.3501803Z == clock drift check ==
2019-11-04T02:51:06.3501803Z == clock drift check ==
2019-11-04T02:51:06.3501886Z   local time: Mon Nov  4 02:51:06 UTC 2019
2019-11-04T02:51:06.5242353Z   network time: Mon, 04 Nov 2019 02:51:06 GMT
2019-11-04T02:51:06.5245677Z == end clock drift check ==
2019-11-04T02:51:14.8308108Z 
2019-11-04T02:51:14.8425764Z ##[error]Bash exited with code '1'.
2019-11-04T02:51:14.8455773Z ##[section]Starting: Checkout
2019-11-04T02:51:14.8457733Z ==============================================================================
2019-11-04T02:51:14.8457815Z Task         : Get sources
2019-11-04T02:51:14.8457858Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
