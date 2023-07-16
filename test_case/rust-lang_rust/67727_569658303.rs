plain
2019-12-30T11:52:57.3794755Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-30T11:52:57.9798645Z ##[command]git config gc.auto 0
2019-12-30T11:52:57.9806083Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-30T11:52:57.9810360Z ##[command]git config --get-all http.proxy
2019-12-30T11:52:57.9813549Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67727/merge:refs/remotes/pull/67727/merge
---
2019-12-30T11:56:06.6577270Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-12-30T11:56:06.6864413Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-12-30T11:56:06.6864547Z 
2019-12-30T11:56:06.6864904Z Caused by:
2019-12-30T11:56:06.6865655Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-12-30T11:56:06.6878552Z Build completed unsuccessfully in 0:00:16
2019-12-30T11:56:06.6940106Z Makefile:67: recipe for target 'prepare' failed
2019-12-30T11:56:06.6940281Z make: *** [prepare] Error 1
2019-12-30T11:56:07.6958444Z Command failed. Attempt 2/5:
2019-12-30T11:56:07.6958444Z Command failed. Attempt 2/5:
2019-12-30T11:56:07.7881745Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-12-30T11:56:07.8177745Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-12-30T11:56:07.8178422Z 
2019-12-30T11:56:07.8178756Z Caused by:
2019-12-30T11:56:07.8179597Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-12-30T11:56:07.8190647Z Build completed unsuccessfully in 0:00:00
2019-12-30T11:56:07.8242731Z Makefile:67: recipe for target 'prepare' failed
2019-12-30T11:56:07.8242847Z make: *** [prepare] Error 1
2019-12-30T11:56:09.8261624Z Command failed. Attempt 3/5:
2019-12-30T11:56:09.8261624Z Command failed. Attempt 3/5:
2019-12-30T11:56:09.9173682Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-12-30T11:56:09.9464899Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-12-30T11:56:09.9464976Z 
2019-12-30T11:56:09.9465041Z Caused by:
2019-12-30T11:56:09.9465896Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-12-30T11:56:09.9474402Z Build completed unsuccessfully in 0:00:00
2019-12-30T11:56:09.9522089Z Makefile:67: recipe for target 'prepare' failed
2019-12-30T11:56:09.9522208Z make: *** [prepare] Error 1
2019-12-30T11:56:12.9545471Z Command failed. Attempt 4/5:
2019-12-30T11:56:12.9545471Z Command failed. Attempt 4/5:
2019-12-30T11:56:13.0469125Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-12-30T11:56:13.0758046Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-12-30T11:56:13.0758121Z 
2019-12-30T11:56:13.0758176Z Caused by:
2019-12-30T11:56:13.0758662Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-12-30T11:56:13.0765989Z Build completed unsuccessfully in 0:00:00
2019-12-30T11:56:13.0814502Z Makefile:67: recipe for target 'prepare' failed
2019-12-30T11:56:13.0814590Z make: *** [prepare] Error 1
2019-12-30T11:56:17.0828603Z Command failed. Attempt 5/5:
2019-12-30T11:56:17.0828603Z Command failed. Attempt 5/5:
2019-12-30T11:56:17.1781843Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-12-30T11:56:17.2083677Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-12-30T11:56:17.2083763Z 
2019-12-30T11:56:17.2083857Z Caused by:
2019-12-30T11:56:17.2084483Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-12-30T11:56:17.2094793Z Build completed unsuccessfully in 0:00:00
2019-12-30T11:56:17.2141632Z Makefile:67: recipe for target 'prepare' failed
2019-12-30T11:56:17.2141754Z make: *** [prepare] Error 1
2019-12-30T11:56:17.2149440Z The command has failed after 5 attempts.
2019-12-30T11:56:17.2149440Z The command has failed after 5 attempts.
2019-12-30T11:56:17.2149577Z == clock drift check ==
2019-12-30T11:56:17.2160483Z   local time: Mon Dec 30 11:56:17 UTC 2019
2019-12-30T11:56:17.2443381Z   network time: Mon, 30 Dec 2019 11:56:17 GMT
2019-12-30T11:56:17.2443568Z == end clock drift check ==
2019-12-30T11:56:28.4335215Z 
2019-12-30T11:56:28.4449135Z ##[error]Bash exited with code '1'.
2019-12-30T11:56:28.4481621Z ##[section]Starting: Checkout
2019-12-30T11:56:28.4483444Z ==============================================================================
2019-12-30T11:56:28.4483504Z Task         : Get sources
2019-12-30T11:56:28.4483553Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
