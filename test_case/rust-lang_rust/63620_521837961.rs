plain
2019-08-15T23:42:21.9271480Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-15T23:42:21.9436438Z ##[command]git config gc.auto 0
2019-08-15T23:42:21.9519986Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-15T23:42:21.9581961Z ##[command]git config --get-all http.proxy
2019-08-15T23:42:21.9728646Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63620/merge:refs/remotes/pull/63620/merge
---
2019-08-15T23:42:56.2223126Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T23:42:56.2223299Z 
2019-08-15T23:42:56.2223807Z   git checkout -b <new-branch-name>
2019-08-15T23:42:56.2223869Z 
2019-08-15T23:42:56.2223958Z HEAD is now at 737562ccb Merge 4c0cf881d87b87c98515cf928604ad0b58fe0161 into f7af19c279b8b7ea3d2c21fcbd67164af8d5d968
2019-08-15T23:42:56.2386938Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-15T23:42:56.2390214Z ==============================================================================
2019-08-15T23:42:56.2390272Z Task         : Bash
2019-08-15T23:42:56.2390336Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-15T23:49:28.0950652Z    Compiling serde_json v1.0.40
2019-08-15T23:49:29.9232106Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-15T23:49:40.7148719Z     Finished release [optimized] target(s) in 1m 30s
2019-08-15T23:49:40.7220859Z tidy check
2019-08-15T23:49:41.6768864Z tidy error: /checkout/src/librustc/traits/fulfill.rs:186: line longer than 100 chars
2019-08-15T23:49:42.8161817Z some tidy checks failed
2019-08-15T23:49:42.8163801Z 
2019-08-15T23:49:42.8163801Z 
2019-08-15T23:49:42.8165664Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-15T23:49:42.8166086Z 
2019-08-15T23:49:42.8166230Z 
2019-08-15T23:49:42.8178461Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-15T23:49:42.8178668Z Build completed unsuccessfully in 0:01:34
2019-08-15T23:49:42.8178668Z Build completed unsuccessfully in 0:01:34
2019-08-15T23:49:42.8231480Z == clock drift check ==
2019-08-15T23:49:42.8249074Z   local time: Thu Aug 15 23:49:42 UTC 2019
2019-08-15T23:49:42.9758524Z   network time: Thu, 15 Aug 2019 23:49:42 GMT
2019-08-15T23:49:42.9760385Z == end clock drift check ==
2019-08-15T23:49:44.4400599Z ##[error]Bash exited with code '1'.
2019-08-15T23:49:44.4430523Z ##[section]Starting: Checkout
2019-08-15T23:49:44.4431973Z ==============================================================================
2019-08-15T23:49:44.4432020Z Task         : Get sources
2019-08-15T23:49:44.4432074Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
