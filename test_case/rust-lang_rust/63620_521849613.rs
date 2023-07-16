plain
2019-08-16T00:55:02.9750902Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T00:55:02.9949845Z ##[command]git config gc.auto 0
2019-08-16T00:55:03.0023598Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T00:55:03.0074235Z ##[command]git config --get-all http.proxy
2019-08-16T00:55:03.0202664Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63620/merge:refs/remotes/pull/63620/merge
---
2019-08-16T00:55:37.7062571Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T00:55:37.7062603Z 
2019-08-16T00:55:37.7063014Z   git checkout -b <new-branch-name>
2019-08-16T00:55:37.7063039Z 
2019-08-16T00:55:37.7063079Z HEAD is now at 217b50d07 Merge e695c785936179713c5c42e12247c9ce6aa17991 into f7af19c279b8b7ea3d2c21fcbd67164af8d5d968
2019-08-16T00:55:37.7224620Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T00:55:37.7227142Z ==============================================================================
2019-08-16T00:55:37.7227186Z Task         : Bash
2019-08-16T00:55:37.7227223Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T01:02:10.5053797Z    Compiling serde_json v1.0.40
2019-08-16T01:02:12.4159482Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-16T01:02:23.6706947Z     Finished release [optimized] target(s) in 1m 34s
2019-08-16T01:02:23.6785524Z tidy check
2019-08-16T01:02:24.3591988Z tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs: ignoring line length unnecessarily
2019-08-16T01:02:25.8195501Z some tidy checks failed
2019-08-16T01:02:25.8197588Z 
2019-08-16T01:02:25.8197588Z 
2019-08-16T01:02:25.8198824Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-16T01:02:25.8198964Z 
2019-08-16T01:02:25.8198989Z 
2019-08-16T01:02:25.8215972Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-16T01:02:25.8216053Z Build completed unsuccessfully in 0:01:37
2019-08-16T01:02:25.8216053Z Build completed unsuccessfully in 0:01:37
2019-08-16T01:02:25.8271111Z == clock drift check ==
2019-08-16T01:02:25.8288399Z   local time: Fri Aug 16 01:02:25 UTC 2019
2019-08-16T01:02:25.9124562Z   network time: Fri, 16 Aug 2019 01:02:25 GMT
2019-08-16T01:02:25.9131216Z == end clock drift check ==
2019-08-16T01:02:27.2276606Z ##[error]Bash exited with code '1'.
2019-08-16T01:02:27.2307780Z ##[section]Starting: Checkout
2019-08-16T01:02:27.2309586Z ==============================================================================
2019-08-16T01:02:27.2309651Z Task         : Get sources
2019-08-16T01:02:27.2309692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
