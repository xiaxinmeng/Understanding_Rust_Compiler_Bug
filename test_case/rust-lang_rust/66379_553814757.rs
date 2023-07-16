plain
2019-11-14T09:42:01.6563682Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-14T09:42:01.6745021Z ##[command]git config gc.auto 0
2019-11-14T09:42:01.6811802Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-14T09:42:01.6858689Z ##[command]git config --get-all http.proxy
2019-11-14T09:42:01.7016897Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66379/merge:refs/remotes/pull/66379/merge
---
2019-11-14T09:48:24.3526139Z Found 0 error codes with no tests
2019-11-14T09:48:24.3527830Z Done!
2019-11-14T09:48:24.3533370Z 
2019-11-14T09:48:24.3533509Z 
2019-11-14T09:48:24.3534618Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-14T09:48:24.3534937Z 
2019-11-14T09:48:24.3534999Z 
2019-11-14T09:48:24.3541032Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-14T09:48:24.3541496Z Build completed unsuccessfully in 0:01:33
2019-11-14T09:48:24.3541496Z Build completed unsuccessfully in 0:01:33
2019-11-14T09:48:24.3595306Z == clock drift check ==
2019-11-14T09:48:24.3604109Z   local time: Thu Nov 14 09:48:24 UTC 2019
2019-11-14T09:48:24.5103346Z   network time: Thu, 14 Nov 2019 09:48:24 GMT
2019-11-14T09:48:24.5103946Z == end clock drift check ==
2019-11-14T09:48:25.9957231Z 
2019-11-14T09:48:26.0084103Z ##[error]Bash exited with code '1'.
2019-11-14T09:48:26.0113416Z ##[section]Starting: Checkout
2019-11-14T09:48:26.0115375Z ==============================================================================
2019-11-14T09:48:26.0115431Z Task         : Get sources
2019-11-14T09:48:26.0115500Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
