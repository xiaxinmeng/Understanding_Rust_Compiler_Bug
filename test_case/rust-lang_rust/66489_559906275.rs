plain
2019-11-30T02:28:44.2829896Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T02:28:44.2990989Z ##[command]git config gc.auto 0
2019-11-30T02:28:44.3088281Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T02:28:44.3164694Z ##[command]git config --get-all http.proxy
2019-11-30T02:28:44.3324116Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66489/merge:refs/remotes/pull/66489/merge
---
2019-11-30T02:31:46.7267259Z #########################################################                 79.9%
2019-11-30T02:31:46.7268627Z ######################################################################## 100.0%
2019-11-30T02:31:47.1534826Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-30T02:31:47.2410996Z     Updating crates.io index
2019-11-30T02:31:54.8383871Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T02:31:54.8418675Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T02:31:54.8465875Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T02:31:54.8466515Z make: *** [prepare] Error 1
2019-11-30T02:31:55.8487755Z Command failed. Attempt 2/5:
2019-11-30T02:31:55.9753452Z     Updating crates.io index
2019-11-30T02:31:55.9753452Z     Updating crates.io index
2019-11-30T02:31:56.3499681Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T02:31:56.3524037Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T02:31:56.3570223Z make: *** [prepare] Error 1
2019-11-30T02:31:56.3573364Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T02:31:58.3593476Z Command failed. Attempt 3/5:
2019-11-30T02:31:58.4891095Z     Updating crates.io index
2019-11-30T02:31:58.4891095Z     Updating crates.io index
2019-11-30T02:31:58.8271307Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T02:31:58.8296106Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T02:31:58.8343861Z make: *** [prepare] Error 1
2019-11-30T02:31:58.8344826Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T02:32:01.8364778Z Command failed. Attempt 4/5:
2019-11-30T02:32:01.9612805Z     Updating crates.io index
2019-11-30T02:32:01.9612805Z     Updating crates.io index
2019-11-30T02:32:02.5724492Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T02:32:02.5747708Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T02:32:02.5793146Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T02:32:02.5793574Z make: *** [prepare] Error 1
2019-11-30T02:32:07.3767882Z Command failed. Attempt 5/5:
2019-11-30T02:32:07.3768990Z     Updating crates.io index
2019-11-30T02:32:07.3768990Z     Updating crates.io index
2019-11-30T02:32:07.3770061Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-30T02:32:07.3770704Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-30T02:32:07.3771426Z make: *** [prepare] Error 1
2019-11-30T02:32:07.3771952Z Makefile:67: recipe for target 'prepare' failed
2019-11-30T02:32:07.3772242Z The command has failed after 5 attempts.
2019-11-30T02:32:07.3772397Z == clock drift check ==
2019-11-30T02:32:07.3772397Z == clock drift check ==
2019-11-30T02:32:07.3772586Z   local time: Sat Nov 30 02:32:07 UTC 2019
2019-11-30T02:32:07.3772748Z   network time: Sat, 30 Nov 2019 02:32:07 GMT
2019-11-30T02:32:07.3772903Z == end clock drift check ==
2019-11-30T02:32:19.1700950Z 
2019-11-30T02:32:19.1821154Z ##[error]Bash exited with code '1'.
2019-11-30T02:32:19.1854861Z ##[section]Starting: Checkout
2019-11-30T02:32:19.1856976Z ==============================================================================
2019-11-30T02:32:19.1857040Z Task         : Get sources
2019-11-30T02:32:19.1857130Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
