plain
2019-10-18T21:50:13.8096711Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T21:50:13.8306663Z ##[command]git config gc.auto 0
2019-10-18T21:50:13.8365003Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T21:50:13.8440158Z ##[command]git config --get-all http.proxy
2019-10-18T21:50:13.8577981Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65167/merge:refs/remotes/pull/65167/merge
---
2019-10-18T21:53:49.6158540Z 
2019-10-18T21:53:49.6158831Z ######################################################################## 100.0%
2019-10-18T21:53:49.6528366Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-10-18T21:53:49.7342213Z     Updating crates.io index
2019-10-18T21:53:53.3652051Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-18T21:53:53.3686137Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-18T21:53:53.3740899Z Makefile:67: recipe for target 'prepare' failed
2019-10-18T21:53:53.3741710Z make: *** [prepare] Error 1
2019-10-18T21:53:54.3764940Z Command failed. Attempt 2/5:
2019-10-18T21:53:54.5035964Z     Updating crates.io index
2019-10-18T21:53:54.5035964Z     Updating crates.io index
2019-10-18T21:53:54.8418412Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-18T21:53:54.8442993Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-18T21:53:54.8488461Z make: *** [prepare] Error 1
2019-10-18T21:53:54.8494064Z Makefile:67: recipe for target 'prepare' failed
2019-10-18T21:53:56.8507380Z Command failed. Attempt 3/5:
2019-10-18T21:53:56.9782982Z     Updating crates.io index
2019-10-18T21:53:56.9782982Z     Updating crates.io index
2019-10-18T21:53:57.3160424Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-18T21:53:57.3188398Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-18T21:53:57.3239636Z Makefile:67: recipe for target 'prepare' failed
2019-10-18T21:53:57.3243468Z make: *** [prepare] Error 1
2019-10-18T21:54:00.3265328Z Command failed. Attempt 4/5:
2019-10-18T21:54:00.4538330Z     Updating crates.io index
2019-10-18T21:54:00.4538330Z     Updating crates.io index
2019-10-18T21:54:00.7882051Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-18T21:54:00.7937952Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-18T21:54:00.7952969Z Makefile:67: recipe for target 'prepare' failed
2019-10-18T21:54:00.7953413Z make: *** [prepare] Error 1
2019-10-18T21:54:04.7966503Z Command failed. Attempt 5/5:
2019-10-18T21:54:04.9265721Z     Updating crates.io index
2019-10-18T21:54:04.9265721Z     Updating crates.io index
2019-10-18T21:54:05.2724038Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-18T21:54:05.2750883Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-18T21:54:05.2802482Z make: *** [prepare] Error 1
2019-10-18T21:54:05.2806721Z Makefile:67: recipe for target 'prepare' failed
2019-10-18T21:54:05.2813084Z The command has failed after 5 attempts.
2019-10-18T21:54:05.2816179Z == clock drift check ==
2019-10-18T21:54:05.2816179Z == clock drift check ==
2019-10-18T21:54:05.2828634Z   local time: Fri Oct 18 21:54:05 UTC 2019
2019-10-18T21:54:05.3067985Z   network time: Fri, 18 Oct 2019 21:54:05 GMT
2019-10-18T21:54:05.3068450Z == end clock drift check ==
2019-10-18T21:54:11.6540753Z 
2019-10-18T21:54:11.6653382Z ##[error]Bash exited with code '1'.
2019-10-18T21:54:11.6691284Z ##[section]Starting: Checkout
2019-10-18T21:54:11.6693369Z ==============================================================================
2019-10-18T21:54:11.6693498Z Task         : Get sources
2019-10-18T21:54:11.6693554Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
