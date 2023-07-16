plain
2019-11-08T03:24:55.4111250Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T03:24:55.4300866Z ##[command]git config gc.auto 0
2019-11-08T03:24:55.4377941Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T03:24:55.4433615Z ##[command]git config --get-all http.proxy
2019-11-08T03:24:55.4599106Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66207/merge:refs/remotes/pull/66207/merge
---
2019-11-08T03:28:40.9004070Z ######################################################################## 100.0%
2019-11-08T03:28:41.3903288Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-08T03:28:41.4810153Z     Updating crates.io index
2019-11-08T03:28:46.9276192Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:28:51.5620825Z     Updating git repository `***-clippy`
2019-11-08T03:28:56.6786898Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:28:56.6813478Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:28:56.6858316Z make: *** [prepare] Error 1
2019-11-08T03:28:56.6859042Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:28:57.6881321Z Command failed. Attempt 2/5:
2019-11-08T03:28:57.8253964Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:28:57.8253964Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:28:58.1224989Z     Updating git repository `***-clippy`
2019-11-08T03:28:59.2179686Z     Updating crates.io index
2019-11-08T03:29:00.4642777Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:29:00.4679920Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:29:00.4726193Z make: *** [prepare] Error 1
2019-11-08T03:29:00.4726818Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:29:02.4741367Z Command failed. Attempt 3/5:
2019-11-08T03:29:02.6662939Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:29:02.6662939Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:29:02.8960012Z     Updating git repository `***-clippy`
2019-11-08T03:29:03.1111367Z     Updating crates.io index
2019-11-08T03:29:03.3640294Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:29:03.3658385Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:29:03.3700135Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:29:03.3703050Z make: *** [prepare] Error 1
2019-11-08T03:29:06.3717985Z Command failed. Attempt 4/5:
2019-11-08T03:29:06.5202641Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:29:06.5202641Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:29:08.6242346Z     Updating git repository `***-clippy`
2019-11-08T03:29:08.8303846Z     Updating crates.io index
2019-11-08T03:29:09.0755202Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:29:09.0774290Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:29:09.0816447Z make: *** [prepare] Error 1
2019-11-08T03:29:09.0820614Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:29:13.0833970Z Command failed. Attempt 5/5:
2019-11-08T03:29:13.2239082Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:29:13.2239082Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-08T03:29:13.4502615Z     Updating git repository `***-clippy`
2019-11-08T03:29:13.6524019Z     Updating crates.io index
2019-11-08T03:29:13.9002370Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-08T03:29:13.9018859Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-08T03:29:13.9072427Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T03:29:13.9072559Z make: *** [prepare] Error 1
2019-11-08T03:29:13.9073705Z The command has failed after 5 attempts.
2019-11-08T03:29:13.9077339Z == clock drift check ==
2019-11-08T03:29:13.9077339Z == clock drift check ==
2019-11-08T03:29:13.9090863Z   local time: Fri Nov  8 03:29:13 UTC 2019
2019-11-08T03:29:13.9799996Z   network time: Fri, 08 Nov 2019 03:29:13 GMT
2019-11-08T03:29:13.9803137Z == end clock drift check ==
2019-11-08T03:29:16.0240300Z 
2019-11-08T03:29:16.0349454Z ##[error]Bash exited with code '1'.
2019-11-08T03:29:16.0378675Z ##[section]Starting: Checkout
2019-11-08T03:29:16.0380366Z ==============================================================================
2019-11-08T03:29:16.0380425Z Task         : Get sources
2019-11-08T03:29:16.0380476Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
