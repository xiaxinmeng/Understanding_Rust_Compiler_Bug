plain
2019-10-22T03:32:31.7569944Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T03:32:31.7759921Z ##[command]git config gc.auto 0
2019-10-22T03:32:31.7856649Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T03:32:31.7926401Z ##[command]git config --get-all http.proxy
2019-10-22T03:32:31.8083924Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-22T03:39:32.1110174Z Found 0 error codes with no tests
2019-10-22T03:39:32.1110502Z Done!
2019-10-22T03:39:32.1110865Z 
2019-10-22T03:39:32.1111173Z 
2019-10-22T03:39:32.1112380Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-22T03:39:32.1113061Z 
2019-10-22T03:39:32.1113296Z 
2019-10-22T03:39:32.1117713Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-22T03:39:32.1118202Z Build completed unsuccessfully in 0:01:38
2019-10-22T03:39:32.1118202Z Build completed unsuccessfully in 0:01:38
2019-10-22T03:39:32.1175115Z == clock drift check ==
2019-10-22T03:39:32.1199074Z   local time: Tue Oct 22 03:39:32 UTC 2019
2019-10-22T03:39:32.2174429Z   network time: Tue, 22 Oct 2019 03:39:32 GMT
2019-10-22T03:39:32.2185890Z == end clock drift check ==
2019-10-22T03:39:33.6069495Z 
2019-10-22T03:39:33.6186749Z ##[error]Bash exited with code '1'.
2019-10-22T03:39:33.6236041Z ##[section]Starting: Checkout
2019-10-22T03:39:33.6238104Z ==============================================================================
2019-10-22T03:39:33.6238531Z Task         : Get sources
2019-10-22T03:39:33.6238611Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
