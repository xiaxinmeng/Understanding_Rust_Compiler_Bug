plain
2019-08-16T15:38:38.2358342Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T15:38:38.2521584Z ##[command]git config gc.auto 0
2019-08-16T15:38:38.2605425Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T15:38:38.2659076Z ##[command]git config --get-all http.proxy
2019-08-16T15:38:38.2801348Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62800/merge:refs/remotes/pull/62800/merge
---
2019-08-16T15:39:14.4381396Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T15:39:14.4381432Z 
2019-08-16T15:39:14.4381679Z   git checkout -b <new-branch-name>
2019-08-16T15:39:14.4381736Z 
2019-08-16T15:39:14.4381794Z HEAD is now at 290f9e4f0 Merge 7f6020c36a70934df861d6f17e1c362cd7d3de74 into 9dd5c191993aab6c2f1538eb8ab69afdc4b6e67a
2019-08-16T15:39:14.4529892Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T15:39:14.4533169Z ==============================================================================
2019-08-16T15:39:14.4533227Z Task         : Bash
2019-08-16T15:39:14.4533291Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T15:42:28.2365733Z #########################                                                 35.9%
2019-08-16T15:42:28.3162775Z ##############################################                            64.4%
2019-08-16T15:42:28.3166841Z ######################################################################## 100.0%
2019-08-16T15:42:29.6586372Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-16T15:42:29.6823227Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-16T15:42:29.6823375Z Caused by:
2019-08-16T15:42:29.6823462Z   No such file or directory (os error 2)
2019-08-16T15:42:29.6823462Z   No such file or directory (os error 2)
2019-08-16T15:42:29.6833024Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-16T15:42:29.6833140Z Build completed unsuccessfully in 0:00:37
2019-08-16T15:42:29.6882299Z Makefile:67: recipe for target 'prepare' failed
2019-08-16T15:42:29.6882384Z make: *** [prepare] Error 1
2019-08-16T15:42:30.6900214Z Command failed. Attempt 2/5:
2019-08-16T15:42:30.7513411Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-16T15:42:30.7513606Z Caused by:
2019-08-16T15:42:30.7513653Z   No such file or directory (os error 2)
2019-08-16T15:42:30.7513653Z   No such file or directory (os error 2)
2019-08-16T15:42:30.7521568Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-16T15:42:30.7521698Z Build completed unsuccessfully in 0:00:00
2019-08-16T15:42:30.7561242Z Makefile:67: recipe for target 'prepare' failed
2019-08-16T15:42:30.7561340Z make: *** [prepare] Error 1
2019-08-16T15:42:32.7579021Z Command failed. Attempt 3/5:
2019-08-16T15:42:32.8179085Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-16T15:42:32.8180271Z Caused by:
2019-08-16T15:42:32.8180671Z   No such file or directory (os error 2)
2019-08-16T15:42:32.8180671Z   No such file or directory (os error 2)
2019-08-16T15:42:32.8184000Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-16T15:42:32.8184470Z Build completed unsuccessfully in 0:00:00
2019-08-16T15:42:32.8224164Z Makefile:67: recipe for target 'prepare' failed
2019-08-16T15:42:32.8224856Z make: *** [prepare] Error 1
2019-08-16T15:42:35.8243943Z Command failed. Attempt 4/5:
2019-08-16T15:42:35.8894933Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-16T15:42:35.8897547Z Caused by:
2019-08-16T15:42:35.8897968Z   No such file or directory (os error 2)
2019-08-16T15:42:35.8897968Z   No such file or directory (os error 2)
2019-08-16T15:42:35.8900130Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-16T15:42:35.8900453Z Build completed unsuccessfully in 0:00:00
2019-08-16T15:42:35.8943319Z Makefile:67: recipe for target 'prepare' failed
2019-08-16T15:42:35.8943673Z make: *** [prepare] Error 1
2019-08-16T15:42:39.8959987Z Command failed. Attempt 5/5:
2019-08-16T15:42:39.9575907Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-16T15:42:39.9576059Z Caused by:
2019-08-16T15:42:39.9576160Z   No such file or directory (os error 2)
2019-08-16T15:42:39.9576160Z   No such file or directory (os error 2)
2019-08-16T15:42:39.9581481Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-16T15:42:39.9581569Z Build completed unsuccessfully in 0:00:00
2019-08-16T15:42:39.9625964Z Makefile:67: recipe for target 'prepare' failed
2019-08-16T15:42:39.9626042Z make: *** [prepare] Error 1
2019-08-16T15:42:39.9631520Z == clock drift check ==
2019-08-16T15:42:39.9638928Z   local time: Fri Aug 16 15:42:39 UTC 2019
2019-08-16T15:42:40.2402047Z   network time: Fri, 16 Aug 2019 15:42:40 GMT
2019-08-16T15:42:40.2404473Z == end clock drift check ==
2019-08-16T15:42:40.2404473Z == end clock drift check ==
2019-08-16T15:42:53.6894858Z ##[error]Bash exited with code '1'.
2019-08-16T15:42:53.6930479Z ##[section]Starting: Checkout
2019-08-16T15:42:53.6932134Z ==============================================================================
2019-08-16T15:42:53.6932189Z Task         : Get sources
2019-08-16T15:42:53.6932265Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
