plain
2019-10-25T07:11:33.8850877Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T07:11:33.9058574Z ##[command]git config gc.auto 0
2019-10-25T07:11:33.9110811Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T07:11:33.9168378Z ##[command]git config --get-all http.proxy
2019-10-25T07:11:33.9320629Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65167/merge:refs/remotes/pull/65167/merge
---
2019-10-25T07:15:02.3333584Z ###############################################################           88.5%
2019-10-25T07:15:02.3336616Z ######################################################################## 100.0%
2019-10-25T07:15:02.7371187Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-10-25T07:15:02.8179004Z     Updating crates.io index
2019-10-25T07:15:07.2433986Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-25T07:15:07.2464099Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-25T07:15:07.2508396Z make: *** [prepare] Error 1
2019-10-25T07:15:07.2509309Z Makefile:67: recipe for target 'prepare' failed
2019-10-25T07:15:08.2536852Z Command failed. Attempt 2/5:
2019-10-25T07:15:08.2536852Z Command failed. Attempt 2/5:
2019-10-25T07:15:08.4720877Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-25T07:15:08.4741246Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-25T07:15:08.4786957Z Makefile:67: recipe for target 'prepare' failed
2019-10-25T07:15:08.4787054Z make: *** [prepare] Error 1
2019-10-25T07:15:10.4807512Z Command failed. Attempt 3/5:
2019-10-25T07:15:10.4807512Z Command failed. Attempt 3/5:
2019-10-25T07:15:10.6917223Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-25T07:15:10.6931622Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-25T07:15:10.6980631Z make: *** [prepare] Error 1
2019-10-25T07:15:10.6981103Z Makefile:67: recipe for target 'prepare' failed
2019-10-25T07:15:13.7005263Z Command failed. Attempt 4/5:
2019-10-25T07:15:13.7005263Z Command failed. Attempt 4/5:
2019-10-25T07:15:13.9239281Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-25T07:15:13.9253552Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-25T07:15:13.9297942Z Makefile:67: recipe for target 'prepare' failed
2019-10-25T07:15:13.9303503Z make: *** [prepare] Error 1
2019-10-25T07:15:17.9318710Z Command failed. Attempt 5/5:
2019-10-25T07:15:17.9318710Z Command failed. Attempt 5/5:
2019-10-25T07:15:18.1522689Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-25T07:15:18.1535295Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-25T07:15:18.1587338Z make: *** [prepare] Error 1
2019-10-25T07:15:18.1588271Z Makefile:67: recipe for target 'prepare' failed
2019-10-25T07:15:18.1592850Z The command has failed after 5 attempts.
2019-10-25T07:15:18.1596712Z == clock drift check ==
2019-10-25T07:15:18.1596712Z == clock drift check ==
2019-10-25T07:15:18.1615135Z   local time: Fri Oct 25 07:15:18 UTC 2019
2019-10-25T07:15:18.2108550Z   network time: Fri, 25 Oct 2019 07:15:18 GMT
2019-10-25T07:15:18.2113800Z == end clock drift check ==
2019-10-25T07:15:26.8659484Z 
2019-10-25T07:15:26.8760492Z ##[error]Bash exited with code '1'.
2019-10-25T07:15:26.8793747Z ##[section]Starting: Checkout
2019-10-25T07:15:26.8795521Z ==============================================================================
2019-10-25T07:15:26.8795580Z Task         : Get sources
2019-10-25T07:15:26.8795646Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
