plain
2019-10-02T09:50:14.5794917Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-02T09:50:14.6016994Z ##[command]git config gc.auto 0
2019-10-02T09:50:14.6066896Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-02T09:50:14.6135138Z ##[command]git config --get-all http.proxy
2019-10-02T09:50:14.6263286Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64990/merge:refs/remotes/pull/64990/merge
---
2019-10-02T09:57:15.2365168Z tidy check
2019-10-02T09:57:15.9764357Z * 579 error codes
2019-10-02T09:57:15.9764504Z * highest error code: E0734
2019-10-02T09:57:16.3216334Z * 263 features
2019-10-02T09:57:16.5300122Z tidy error: /checkout/src/libcore/num/f32.rs:12: platform-specific cfg: cfg(target_os = "uefi")
2019-10-02T09:57:16.9944824Z some tidy checks failed
2019-10-02T09:57:16.9945760Z 
2019-10-02T09:57:16.9945760Z 
2019-10-02T09:57:16.9946845Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-02T09:57:16.9947244Z 
2019-10-02T09:57:16.9947555Z 
2019-10-02T09:57:16.9951859Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-02T09:57:16.9952064Z Build completed unsuccessfully in 0:01:30
2019-10-02T09:57:16.9952064Z Build completed unsuccessfully in 0:01:30
2019-10-02T09:57:17.0005378Z == clock drift check ==
2019-10-02T09:57:17.0022141Z   local time: Wed Oct  2 09:57:17 UTC 2019
2019-10-02T09:57:17.1005504Z   network time: Wed, 02 Oct 2019 09:57:17 GMT
2019-10-02T09:57:17.1006145Z == end clock drift check ==
2019-10-02T09:57:18.4771770Z ##[error]Bash exited with code '1'.
2019-10-02T09:57:18.4803541Z ##[section]Starting: Checkout
2019-10-02T09:57:18.4805718Z ==============================================================================
2019-10-02T09:57:18.4805774Z Task         : Get sources
2019-10-02T09:57:18.4805844Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
