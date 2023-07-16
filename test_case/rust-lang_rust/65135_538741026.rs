plain
2019-10-06T11:50:39.9403656Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T11:50:39.9601590Z ##[command]git config gc.auto 0
2019-10-06T11:50:39.9689184Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T11:50:39.9763801Z ##[command]git config --get-all http.proxy
2019-10-06T11:50:39.9904295Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65135/merge:refs/remotes/pull/65135/merge
---
2019-10-06T11:58:07.3732864Z Found 1 error codes with no tests
2019-10-06T11:58:07.3732933Z Done!
2019-10-06T11:58:07.3732967Z 
2019-10-06T11:58:07.3732998Z 
2019-10-06T11:58:07.3734149Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-06T11:58:07.3734305Z 
2019-10-06T11:58:07.3734336Z 
2019-10-06T11:58:07.3743871Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-06T11:58:07.3743992Z Build completed unsuccessfully in 0:01:36
2019-10-06T11:58:07.3743992Z Build completed unsuccessfully in 0:01:36
2019-10-06T11:58:07.3803223Z == clock drift check ==
2019-10-06T11:58:07.3823087Z   local time: Sun Oct  6 11:58:07 UTC 2019
2019-10-06T11:58:07.5329559Z   network time: Sun, 06 Oct 2019 11:58:07 GMT
2019-10-06T11:58:07.5329740Z == end clock drift check ==
2019-10-06T11:58:08.9320980Z ##[error]Bash exited with code '1'.
2019-10-06T11:58:08.9369097Z ##[section]Starting: Checkout
2019-10-06T11:58:08.9371054Z ==============================================================================
2019-10-06T11:58:08.9371121Z Task         : Get sources
2019-10-06T11:58:08.9371177Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
