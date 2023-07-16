plain
2019-11-24T10:53:52.9939827Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T10:53:52.9955448Z ##[command]git config gc.auto 0
2019-11-24T10:53:52.9959602Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T10:53:52.9963924Z ##[command]git config --get-all http.proxy
2019-11-24T10:53:52.9968077Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66329/merge:refs/remotes/pull/66329/merge
---
2019-11-24T10:59:27.9730045Z Found 0 error codes with no tests
2019-11-24T10:59:27.9730285Z Done!
2019-11-24T10:59:27.9730480Z 
2019-11-24T10:59:27.9730961Z 
2019-11-24T10:59:27.9733229Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-24T10:59:27.9733803Z 
2019-11-24T10:59:27.9733926Z 
2019-11-24T10:59:27.9734928Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-24T10:59:27.9735129Z Build completed unsuccessfully in 0:01:23
2019-11-24T10:59:27.9735129Z Build completed unsuccessfully in 0:01:23
2019-11-24T10:59:27.9782867Z == clock drift check ==
2019-11-24T10:59:27.9809005Z   local time: Sun Nov 24 10:59:27 UTC 2019
2019-11-24T10:59:28.2531808Z   network time: Sun, 24 Nov 2019 10:59:28 GMT
2019-11-24T10:59:28.2535305Z == end clock drift check ==
2019-11-24T10:59:29.6092456Z 
2019-11-24T10:59:29.6186459Z ##[error]Bash exited with code '1'.
2019-11-24T10:59:29.6239117Z ##[section]Starting: Checkout
2019-11-24T10:59:29.6240528Z ==============================================================================
2019-11-24T10:59:29.6240569Z Task         : Get sources
2019-11-24T10:59:29.6240620Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
