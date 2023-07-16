plain
2019-08-25T23:26:58.9031804Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-25T23:26:58.9238611Z ##[command]git config gc.auto 0
2019-08-25T23:26:58.9324471Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-25T23:26:58.9375356Z ##[command]git config --get-all http.proxy
2019-08-25T23:26:58.9518002Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63825/merge:refs/remotes/pull/63825/merge
---
2019-08-25T23:27:34.0738372Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-25T23:27:34.0738404Z 
2019-08-25T23:27:34.0738605Z   git checkout -b <new-branch-name>
2019-08-25T23:27:34.0738635Z 
2019-08-25T23:27:34.0738705Z HEAD is now at 1132f1b32 Merge 3dc3cf30a45108c45df0492900224515d03b79ea into 521d78407471cb78e9bbf47160f6aa23047ac499
2019-08-25T23:27:34.0910922Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-25T23:27:34.0914541Z ==============================================================================
2019-08-25T23:27:34.0914601Z Task         : Bash
2019-08-25T23:27:34.0914650Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-25T23:33:54.7659866Z    Compiling serde_json v1.0.40
2019-08-25T23:33:56.5941106Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-25T23:34:07.6290144Z     Finished release [optimized] target(s) in 1m 32s
2019-08-25T23:34:07.6364208Z tidy check
2019-08-25T23:34:08.1676169Z tidy error: /checkout/src/tools/compiletest/src/runtest.rs:4: line longer than 100 chars
2019-08-25T23:34:08.1677953Z tidy error: /checkout/src/tools/compiletest/src/runtest.rs:2942: line longer than 100 chars
2019-08-25T23:34:09.6212119Z some tidy checks failed
2019-08-25T23:34:09.6213546Z 
2019-08-25T23:34:09.6213546Z 
2019-08-25T23:34:09.6214853Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-25T23:34:09.6215467Z 
2019-08-25T23:34:09.6215653Z 
2019-08-25T23:34:09.6220405Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-25T23:34:09.6220891Z Build completed unsuccessfully in 0:01:35
2019-08-25T23:34:09.6220891Z Build completed unsuccessfully in 0:01:35
2019-08-25T23:34:09.6269335Z == clock drift check ==
2019-08-25T23:34:09.6283445Z   local time: Sun Aug 25 23:34:09 UTC 2019
2019-08-25T23:34:09.7768637Z   network time: Sun, 25 Aug 2019 23:34:09 GMT
2019-08-25T23:34:09.7776388Z == end clock drift check ==
2019-08-25T23:34:11.1112099Z ##[error]Bash exited with code '1'.
2019-08-25T23:34:11.1145239Z ##[section]Starting: Checkout
2019-08-25T23:34:11.1146981Z ==============================================================================
2019-08-25T23:34:11.1147033Z Task         : Get sources
2019-08-25T23:34:11.1147094Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
