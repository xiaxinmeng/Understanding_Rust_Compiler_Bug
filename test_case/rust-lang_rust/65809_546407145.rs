plain
2019-10-25T15:19:27.0337650Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T15:19:27.0553918Z ##[command]git config gc.auto 0
2019-10-25T15:19:27.0615875Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T15:19:27.0664934Z ##[command]git config --get-all http.proxy
2019-10-25T15:19:27.0806851Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65809/merge:refs/remotes/pull/65809/merge
---
2019-10-25T15:25:50.3121665Z    Compiling serde_json v1.0.40
2019-10-25T15:25:52.2137863Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-25T15:26:04.2054503Z     Finished release [optimized] target(s) in 1m 32s
2019-10-25T15:26:04.2128122Z tidy check
2019-10-25T15:26:04.5940667Z tidy error: /checkout/src/test/codegen/abi-efiapi.rs: missing trailing newline
2019-10-25T15:26:06.5024828Z Found 484 error codes
2019-10-25T15:26:06.5025396Z Found 0 error codes with no tests
2019-10-25T15:26:06.5025499Z Done!
2019-10-25T15:26:06.5025866Z some tidy checks failed
2019-10-25T15:26:06.5025866Z some tidy checks failed
2019-10-25T15:26:06.5030108Z 
2019-10-25T15:26:06.5030936Z 
2019-10-25T15:26:06.5032536Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-25T15:26:06.5033186Z 
2019-10-25T15:26:06.5033269Z 
2019-10-25T15:26:06.5046401Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-25T15:26:06.5046698Z Build completed unsuccessfully in 0:01:35
2019-10-25T15:26:06.5046698Z Build completed unsuccessfully in 0:01:35
2019-10-25T15:26:06.5094737Z == clock drift check ==
2019-10-25T15:26:06.5102926Z   local time: Fri Oct 25 15:26:06 UTC 2019
2019-10-25T15:26:06.7856941Z   network time: Fri, 25 Oct 2019 15:26:06 GMT
2019-10-25T15:26:06.7859554Z == end clock drift check ==
2019-10-25T15:26:08.1101417Z 
2019-10-25T15:26:08.1212095Z ##[error]Bash exited with code '1'.
2019-10-25T15:26:08.1248174Z ##[section]Starting: Checkout
2019-10-25T15:26:08.1249755Z ==============================================================================
2019-10-25T15:26:08.1249831Z Task         : Get sources
2019-10-25T15:26:08.1249875Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
