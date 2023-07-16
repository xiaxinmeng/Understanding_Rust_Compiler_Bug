plain
2019-08-06T09:49:32.7551487Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-06T09:49:33.7027375Z ##[command]git config gc.auto 0
2019-08-06T09:49:33.7038358Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-06T09:49:33.7044488Z ##[command]git config --get-all http.proxy
2019-08-06T09:49:33.7049724Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62800/merge:refs/remotes/pull/62800/merge
---
2019-08-06T09:50:07.4653796Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-06T09:50:07.4654332Z 
2019-08-06T09:50:07.4654763Z   git checkout -b <new-branch-name>
2019-08-06T09:50:07.4654974Z 
2019-08-06T09:50:07.4655165Z HEAD is now at a99d5d13c Merge 40548d0afa738b87f0c86e054414567c1364bdfa into 766b10a8d544550712fd6352863457a86f46db3c
2019-08-06T09:50:07.4787569Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-06T09:50:07.4790600Z ==============================================================================
2019-08-06T09:50:07.4790643Z Task         : Bash
2019-08-06T09:50:07.4790676Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-06T09:52:53.7901600Z ##############                                                            20.3%
2019-08-06T09:52:53.8546854Z ###########################################                               60.9%
2019-08-06T09:52:53.8547174Z ######################################################################## 100.0%
2019-08-06T09:52:56.3463918Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-06T09:52:56.3659739Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-06T09:52:56.3660506Z Caused by:
2019-08-06T09:52:56.3660680Z   No such file or directory (os error 2)
2019-08-06T09:52:56.3660680Z   No such file or directory (os error 2)
2019-08-06T09:52:56.3667358Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-06T09:52:56.3667652Z Build completed unsuccessfully in 0:00:15
2019-08-06T09:52:56.3706411Z make: *** [prepare] Error 1
2019-08-06T09:52:56.3707046Z Makefile:67: recipe for target 'prepare' failed
2019-08-06T09:52:58.2588781Z Command failed. Attempt 2/5:
2019-08-06T09:52:58.2591368Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-06T09:52:58.2591972Z Caused by:
2019-08-06T09:52:58.2592245Z   No such file or directory (os error 2)
2019-08-06T09:52:58.2592245Z   No such file or directory (os error 2)
2019-08-06T09:52:58.2592812Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-06T09:52:58.2593043Z Build completed unsuccessfully in 0:00:00
2019-08-06T09:52:58.2593876Z Makefile:67: recipe for target 'prepare' failed
2019-08-06T09:52:58.2594068Z make: *** [prepare] Error 1
2019-08-06T09:52:59.4347213Z Command failed. Attempt 3/5:
2019-08-06T09:52:59.4832511Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-06T09:52:59.4833578Z Caused by:
2019-08-06T09:52:59.4833805Z   No such file or directory (os error 2)
2019-08-06T09:52:59.4833805Z   No such file or directory (os error 2)
2019-08-06T09:52:59.4840667Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-06T09:52:59.4840797Z Build completed unsuccessfully in 0:00:00
2019-08-06T09:52:59.4871545Z make: *** [prepare] Error 1
2019-08-06T09:52:59.4873727Z Makefile:67: recipe for target 'prepare' failed
2019-08-06T09:53:02.4886915Z Command failed. Attempt 4/5:
2019-08-06T09:53:02.5395199Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-06T09:53:02.5395823Z Caused by:
2019-08-06T09:53:02.5395960Z   No such file or directory (os error 2)
2019-08-06T09:53:02.5395960Z   No such file or directory (os error 2)
2019-08-06T09:53:02.5399477Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-06T09:53:02.5399765Z Build completed unsuccessfully in 0:00:00
2019-08-06T09:53:02.5428960Z Makefile:67: recipe for target 'prepare' failed
2019-08-06T09:53:02.5429384Z make: *** [prepare] Error 1
2019-08-06T09:53:06.5444744Z Command failed. Attempt 5/5:
2019-08-06T09:53:06.5949700Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-06T09:53:06.5950919Z Caused by:
2019-08-06T09:53:06.5951189Z   No such file or directory (os error 2)
2019-08-06T09:53:06.5951189Z   No such file or directory (os error 2)
2019-08-06T09:53:06.5955831Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-06T09:53:06.5956082Z Build completed unsuccessfully in 0:00:00
2019-08-06T09:53:06.5988825Z make: *** [prepare] Error 1
2019-08-06T09:53:06.5989469Z Makefile:67: recipe for target 'prepare' failed
2019-08-06T09:53:06.5992924Z The command has failed after 5 attempts.
2019-08-06T09:53:17.5087935Z ##[error]Bash exited with code '1'.
2019-08-06T09:53:17.5128803Z ##[section]Starting: Checkout
2019-08-06T09:53:17.5130268Z ==============================================================================
2019-08-06T09:53:17.5130310Z Task         : Get sources
2019-08-06T09:53:17.5130345Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
