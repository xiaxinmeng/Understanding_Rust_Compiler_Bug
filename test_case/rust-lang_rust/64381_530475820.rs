plain
2019-09-11T17:07:27.0650370Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-11T17:07:27.0888707Z ##[command]git config gc.auto 0
2019-09-11T17:07:27.1004673Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-11T17:07:27.1079270Z ##[command]git config --get-all http.proxy
2019-09-11T17:07:27.1290676Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64381/merge:refs/remotes/pull/64381/merge
---
2019-09-11T17:11:14.2003390Z 
2019-09-11T17:11:14.2004076Z ######################################################################## 100.0%
2019-09-11T17:11:14.8226313Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-11T17:11:15.8594074Z     Updating crates.io index
2019-09-11T17:11:50.7367579Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-11T17:11:50.7402035Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-11T17:11:50.7451243Z Makefile:67: recipe for target 'prepare' failed
2019-09-11T17:11:50.7451634Z make: *** [prepare] Error 1
2019-09-11T17:11:51.7468750Z Command failed. Attempt 2/5:
2019-09-11T17:11:51.7468750Z Command failed. Attempt 2/5:
2019-09-11T17:11:51.9696773Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-11T17:11:51.9717643Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-11T17:11:51.9767832Z Makefile:67: recipe for target 'prepare' failed
2019-09-11T17:11:51.9768258Z make: *** [prepare] Error 1
2019-09-11T17:11:53.9785694Z Command failed. Attempt 3/5:
2019-09-11T17:11:53.9785694Z Command failed. Attempt 3/5:
2019-09-11T17:11:54.2002421Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-11T17:11:54.2020443Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-11T17:11:54.2069881Z make: *** [prepare] Error 1
2019-09-11T17:11:54.2070405Z Makefile:67: recipe for target 'prepare' failed
2019-09-11T17:11:57.2083711Z Command failed. Attempt 4/5:
2019-09-11T17:11:57.2083711Z Command failed. Attempt 4/5:
2019-09-11T17:11:57.4123218Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-11T17:11:57.4140937Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-11T17:11:57.4189378Z Makefile:67: recipe for target 'prepare' failed
2019-09-11T17:11:57.4195397Z make: *** [prepare] Error 1
2019-09-11T17:12:01.4207590Z Command failed. Attempt 5/5:
2019-09-11T17:12:01.4207590Z Command failed. Attempt 5/5:
2019-09-11T17:12:01.6495506Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-11T17:12:01.6520919Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-11T17:12:01.6576140Z Makefile:67: recipe for target 'prepare' failed
2019-09-11T17:12:01.6577151Z The command has failed after 5 attempts.
2019-09-11T17:12:01.6577217Z == clock drift check ==
2019-09-11T17:12:01.6577269Z   local time: make: *** [prepare] Error 1
2019-09-11T17:12:01.6577269Z   local time: make: *** [prepare] Error 1
2019-09-11T17:12:01.6590341Z Wed Sep 11 17:12:01 UTC 2019
2019-09-11T17:12:01.7316286Z   network time: Wed, 11 Sep 2019 17:12:01 GMT
2019-09-11T17:12:01.7325944Z == end clock drift check ==
2019-09-11T17:12:03.0584510Z ##[error]Bash exited with code '1'.
2019-09-11T17:12:03.0618813Z ##[section]Starting: Checkout
2019-09-11T17:12:03.0620715Z ==============================================================================
2019-09-11T17:12:03.0620772Z Task         : Get sources
2019-09-11T17:12:03.0620837Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
