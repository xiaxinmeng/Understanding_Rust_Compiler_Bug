plain
2019-11-26T11:56:12.2565737Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T11:56:12.8975064Z ##[command]git config gc.auto 0
2019-11-26T11:56:12.8979338Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T11:56:12.8984832Z ##[command]git config --get-all http.proxy
2019-11-26T11:56:12.8990072Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66275/merge:refs/remotes/pull/66275/merge
---
2019-11-26T12:02:29.3916347Z Done!
2019-11-26T12:02:29.3916618Z some tidy checks failed
2019-11-26T12:02:29.3916958Z 
2019-11-26T12:02:29.3917668Z 
2019-11-26T12:02:29.3921006Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-26T12:02:29.3921937Z 
2019-11-26T12:02:29.3922152Z 
2019-11-26T12:02:29.3922492Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-26T12:02:29.3922759Z Build completed unsuccessfully in 0:01:32
2019-11-26T12:02:29.3922759Z Build completed unsuccessfully in 0:01:32
2019-11-26T12:02:29.3969802Z == clock drift check ==
2019-11-26T12:02:29.3979062Z   local time: Tue Nov 26 12:02:29 UTC 2019
2019-11-26T12:02:29.4817846Z   network time: Tue, 26 Nov 2019 12:02:29 GMT
2019-11-26T12:02:29.4818916Z == end clock drift check ==
2019-11-26T12:02:30.7859288Z 
2019-11-26T12:02:30.7967878Z ##[error]Bash exited with code '1'.
2019-11-26T12:02:30.8024999Z ##[section]Starting: Checkout
2019-11-26T12:02:30.8026789Z ==============================================================================
2019-11-26T12:02:30.8026850Z Task         : Get sources
2019-11-26T12:02:30.8026904Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
