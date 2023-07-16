plain
2019-11-29T14:17:48.2421754Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T14:17:48.2637298Z ##[command]git config gc.auto 0
2019-11-29T14:17:48.2741230Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T14:17:48.2821736Z ##[command]git config --get-all http.proxy
2019-11-29T14:17:48.3016476Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66613/merge:refs/remotes/pull/66613/merge
---
2019-11-29T14:21:05.8028914Z ################################################                          67.9%
2019-11-29T14:21:05.8029610Z ######################################################################## 100.0%
2019-11-29T14:21:06.2181003Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-29T14:21:06.3011988Z     Updating crates.io index
2019-11-29T14:21:14.3225924Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-29T14:21:14.3263909Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-29T14:21:14.3322620Z make: *** [prepare] Error 1
2019-11-29T14:21:14.3324046Z Makefile:67: recipe for target 'prepare' failed
2019-11-29T14:21:15.3337050Z Command failed. Attempt 2/5:
2019-11-29T14:21:15.4588173Z     Updating crates.io index
2019-11-29T14:21:15.4588173Z     Updating crates.io index
2019-11-29T14:21:15.8339023Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-29T14:21:15.8365541Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-29T14:21:15.8426062Z make: *** [prepare] Error 1
2019-11-29T14:21:15.8426855Z Makefile:67: recipe for target 'prepare' failed
2019-11-29T14:21:17.8433250Z Command failed. Attempt 3/5:
2019-11-29T14:21:17.9728983Z     Updating crates.io index
2019-11-29T14:21:17.9728983Z     Updating crates.io index
2019-11-29T14:21:18.3125239Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-29T14:21:18.3145710Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-29T14:21:18.3192754Z Makefile:67: recipe for target 'prepare' failed
2019-11-29T14:21:18.3192869Z make: *** [prepare] Error 1
2019-11-29T14:21:21.3205860Z Command failed. Attempt 4/5:
2019-11-29T14:21:21.4460010Z     Updating crates.io index
2019-11-29T14:21:21.4460010Z     Updating crates.io index
2019-11-29T14:21:21.7857274Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-29T14:21:21.7874557Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-29T14:21:21.7922837Z Makefile:67: recipe for target 'prepare' failed
2019-11-29T14:21:21.7927489Z make: *** [prepare] Error 1
2019-11-29T14:21:25.7940466Z Command failed. Attempt 5/5:
2019-11-29T14:21:25.9162891Z     Updating crates.io index
2019-11-29T14:21:25.9162891Z     Updating crates.io index
2019-11-29T14:21:26.2644477Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-29T14:21:26.2663543Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-29T14:21:26.2759068Z Makefile:67: recipe for target 'prepare' failed
2019-11-29T14:21:26.2759271Z The command has failed after 5 attempts.
2019-11-29T14:21:26.2759321Z == clock drift check ==
2019-11-29T14:21:26.2759372Z   local time: Fri Nov 29 14:21:26 UTC 2019
2019-11-29T14:21:26.2759372Z   local time: Fri Nov 29 14:21:26 UTC 2019
2019-11-29T14:21:26.2759444Z   network time: make: *** [prepare] Error 1
2019-11-29T14:21:26.8071480Z Fri, 29 Nov 2019 14:21:26 GMT
2019-11-29T14:21:26.8082427Z == end clock drift check ==
2019-11-29T14:21:39.7126354Z 
2019-11-29T14:21:39.7229033Z ##[error]Bash exited with code '1'.
2019-11-29T14:21:39.7259395Z ##[section]Starting: Checkout
2019-11-29T14:21:39.7261074Z ==============================================================================
2019-11-29T14:21:39.7261130Z Task         : Get sources
2019-11-29T14:21:39.7261179Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
