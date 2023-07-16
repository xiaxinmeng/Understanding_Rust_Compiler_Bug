plain
2019-11-04T13:25:35.3092483Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-04T13:25:35.3293810Z ##[command]git config gc.auto 0
2019-11-04T13:25:35.3372696Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-04T13:25:35.3425322Z ##[command]git config --get-all http.proxy
2019-11-04T13:25:35.3602390Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66072/merge:refs/remotes/pull/66072/merge
---
2019-11-04T13:31:52.5313822Z    Compiling serde_json v1.0.40
2019-11-04T13:31:54.1786324Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-04T13:32:05.0048737Z     Finished release [optimized] target(s) in 1m 24s
2019-11-04T13:32:05.0128416Z tidy check
2019-11-04T13:32:05.4803052Z tidy error: /checkout/src/librustc_interface/pretty.rs: empty file
2019-11-04T13:32:05.4803125Z tidy error: /checkout/src/librustc_interface/pretty.rs: leading newline
2019-11-04T13:32:05.6027670Z tidy error: /checkout/src/librustc/session/config.rs:2625: line longer than 100 chars
2019-11-04T13:32:05.6028523Z tidy error: /checkout/src/librustc/session/config.rs: too many lines (3040) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-11-04T13:32:07.2757884Z some tidy checks failed
2019-11-04T13:32:07.2758681Z Found 485 error codes
2019-11-04T13:32:07.2758955Z Found 0 error codes with no tests
2019-11-04T13:32:07.2759253Z Done!
2019-11-04T13:32:07.2759253Z Done!
2019-11-04T13:32:07.2788202Z 
2019-11-04T13:32:07.2788561Z 
2019-11-04T13:32:07.2789614Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-04T13:32:07.2790048Z 
2019-11-04T13:32:07.2790208Z 
2019-11-04T13:32:07.2790437Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-04T13:32:07.2790606Z Build completed unsuccessfully in 0:01:28
2019-11-04T13:32:07.2790606Z Build completed unsuccessfully in 0:01:28
2019-11-04T13:32:07.2820340Z == clock drift check ==
2019-11-04T13:32:07.2828978Z   local time: Mon Nov  4 13:32:07 UTC 2019
2019-11-04T13:32:07.3668688Z   network time: Mon, 04 Nov 2019 13:32:07 GMT
2019-11-04T13:32:07.3670727Z == end clock drift check ==
2019-11-04T13:32:08.7502682Z 
2019-11-04T13:32:08.7603267Z ##[error]Bash exited with code '1'.
2019-11-04T13:32:08.7629184Z ##[section]Starting: Checkout
2019-11-04T13:32:08.7630978Z ==============================================================================
2019-11-04T13:32:08.7631043Z Task         : Get sources
2019-11-04T13:32:08.7631082Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
