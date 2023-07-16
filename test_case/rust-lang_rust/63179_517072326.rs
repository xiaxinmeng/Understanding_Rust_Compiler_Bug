plain
2019-08-01T00:15:49.4663622Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T00:15:49.4842538Z ##[command]git config gc.auto 0
2019-08-01T00:15:49.4927113Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T00:15:49.4999291Z ##[command]git config --get-all http.proxy
2019-08-01T00:15:49.5144161Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63179/merge:refs/remotes/pull/63179/merge
---
2019-08-01T00:16:24.8730645Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T00:16:24.8732227Z 
2019-08-01T00:16:24.8733707Z   git checkout -b <new-branch-name>
2019-08-01T00:16:24.8734906Z 
2019-08-01T00:16:24.8735989Z HEAD is now at 471b0f1a7 Merge b7bd2ea3a094cfff46a440d6c02561cdac3e0f5a into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-08-01T00:16:24.8864136Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T00:16:24.8867245Z ==============================================================================
2019-08-01T00:16:24.8867311Z Task         : Bash
2019-08-01T00:16:24.8867362Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T00:22:44.1228442Z    Compiling serde_json v1.0.40
2019-08-01T00:22:48.5616635Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-01T00:22:57.4514876Z     Finished release [optimized] target(s) in 1m 32s
2019-08-01T00:22:57.4589053Z tidy check
2019-08-01T00:22:57.9451258Z tidy error: /checkout/src/test/ui/issues/issue-2214.rs:34: trailing whitespace
2019-08-01T00:22:57.9451695Z tidy error: /checkout/src/test/ui/issues/issue-2214.rs:36: trailing whitespace
2019-08-01T00:22:59.3271986Z some tidy checks failed
2019-08-01T00:22:59.3272155Z 
2019-08-01T00:22:59.3272155Z 
2019-08-01T00:22:59.3279856Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-01T00:22:59.3280072Z 
2019-08-01T00:22:59.3280102Z 
2019-08-01T00:22:59.3280992Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-01T00:22:59.3281081Z Build completed unsuccessfully in 0:01:35
2019-08-01T00:22:59.3281081Z Build completed unsuccessfully in 0:01:35
2019-08-01T00:23:00.6209501Z ##[error]Bash exited with code '1'.
2019-08-01T00:23:00.6240443Z ##[section]Starting: Checkout
2019-08-01T00:23:00.6243552Z ==============================================================================
2019-08-01T00:23:00.6243607Z Task         : Get sources
2019-08-01T00:23:00.6243654Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
