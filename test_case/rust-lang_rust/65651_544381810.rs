plain
2019-10-21T06:56:27.0220600Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T06:56:27.0436030Z ##[command]git config gc.auto 0
2019-10-21T06:56:27.0542820Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T06:56:27.0609943Z ##[command]git config --get-all http.proxy
2019-10-21T06:56:27.0750936Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65651/merge:refs/remotes/pull/65651/merge
---
2019-10-21T07:02:57.0149729Z    Compiling serde_json v1.0.40
2019-10-21T07:02:58.9929276Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-21T07:03:11.0805283Z     Finished release [optimized] target(s) in 1m 33s
2019-10-21T07:03:11.0888967Z tidy check
2019-10-21T07:03:11.4391808Z tidy error: /checkout/src/test/ui/intrinsics/intrinsic-alignment.rs:23: tab character
2019-10-21T07:03:11.4512592Z tidy error: /checkout/src/test/ui/structs-enums/rec-align-u64.rs:44: tab character
2019-10-21T07:03:13.3381210Z Found 482 error codes
2019-10-21T07:03:13.3384101Z Found 0 error codes with no tests
2019-10-21T07:03:13.3385518Z Done!
2019-10-21T07:03:13.3386613Z some tidy checks failed
2019-10-21T07:03:13.3386613Z some tidy checks failed
2019-10-21T07:03:13.3386721Z 
2019-10-21T07:03:13.3386751Z 
2019-10-21T07:03:13.3387815Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-21T07:03:13.3387966Z 
2019-10-21T07:03:13.3387999Z 
2019-10-21T07:03:13.3393656Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-21T07:03:13.3393748Z Build completed unsuccessfully in 0:01:36
2019-10-21T07:03:13.3393748Z Build completed unsuccessfully in 0:01:36
2019-10-21T07:03:13.3445354Z == clock drift check ==
2019-10-21T07:03:13.3460832Z   local time: Mon Oct 21 07:03:13 UTC 2019
2019-10-21T07:03:13.4194369Z   network time: Mon, 21 Oct 2019 07:03:13 GMT
2019-10-21T07:03:13.4196183Z == end clock drift check ==
2019-10-21T07:03:14.7543217Z 
2019-10-21T07:03:14.7653023Z ##[error]Bash exited with code '1'.
2019-10-21T07:03:14.7689127Z ##[section]Starting: Checkout
2019-10-21T07:03:14.7691036Z ==============================================================================
2019-10-21T07:03:14.7691100Z Task         : Get sources
2019-10-21T07:03:14.7691168Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
