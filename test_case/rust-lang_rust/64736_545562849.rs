plain
2019-10-23T17:47:05.8627357Z ##[section]Starting: Linux x86_64-gnu-llvm-6.0
2019-10-23T17:47:06.0785561Z ##[section]Starting: Initialize job
2019-10-23T17:47:06.0785800Z Agent name: 'Azure Pipelines 54'
2019-10-23T17:47:06.0785894Z Agent machine name: 'fv-az578'
2019-10-23T17:47:06.0785938Z Current agent version: '2.159.2'
2019-10-23T17:47:06.0814850Z Agent running as: 'vsts'
2019-10-23T17:47:06.1333952Z Set build variables.
2019-10-23T17:47:06.1390605Z Download all required tasks.
2019-10-23T17:47:06.1580565Z Downloading task: Bash (3.159.3)
2019-10-23T17:47:07.0958072Z Downloading task: CmdLine (2.151.2)
---
2019-10-23T17:47:12.5000456Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-23T17:47:12.5263257Z ##[command]git config gc.auto 0
2019-10-23T17:47:12.5328441Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-23T17:47:12.5396558Z ##[command]git config --get-all http.proxy
2019-10-23T17:47:12.5571405Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-10-23T17:53:33.2061551Z    Compiling serde_json v1.0.40
2019-10-23T17:53:35.1868186Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-23T17:53:47.2048176Z     Finished release [optimized] target(s) in 1m 35s
2019-10-23T17:53:47.2154308Z tidy check
2019-10-23T17:53:47.9460221Z tidy error: /checkout/src/librustc/mir/cache.rs:83: line longer than 100 chars
2019-10-23T17:53:47.9460416Z tidy error: /checkout/src/librustc/mir/cache.rs:106: line longer than 100 chars
2019-10-23T17:53:47.9460474Z tidy error: /checkout/src/librustc/mir/cache.rs: missing trailing newline
2019-10-23T17:53:48.0381719Z tidy error: /checkout/src/librustc_codegen_ssa/base.rs:375: TODO is deprecated; use FIXME
2019-10-23T17:53:48.0401685Z tidy error: /checkout/src/librustc_codegen_ssa/mir/place.rs:589: line longer than 100 chars
2019-10-23T17:53:49.7259039Z Found 483 error codes
2019-10-23T17:53:49.7259218Z Found 0 error codes with no tests
2019-10-23T17:53:49.7259267Z Done!
2019-10-23T17:53:49.7259312Z some tidy checks failed
2019-10-23T17:53:49.7259312Z some tidy checks failed
2019-10-23T17:53:49.7259362Z 
2019-10-23T17:53:49.7259390Z 
2019-10-23T17:53:49.7260371Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-23T17:53:49.7260814Z 
2019-10-23T17:53:49.7260841Z 
2019-10-23T17:53:49.7265066Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-23T17:53:49.7265167Z Build completed unsuccessfully in 0:01:39
2019-10-23T17:53:49.7265167Z Build completed unsuccessfully in 0:01:39
2019-10-23T17:53:49.7331722Z == clock drift check ==
2019-10-23T17:53:49.7372511Z   local time: Wed Oct 23 17:53:49 UTC 2019
2019-10-23T17:53:50.0119596Z   network time: Wed, 23 Oct 2019 17:53:50 GMT
2019-10-23T17:53:50.0127430Z == end clock drift check ==
2019-10-23T17:53:51.3701655Z 
2019-10-23T17:53:51.3834749Z ##[error]Bash exited with code '1'.
2019-10-23T17:53:51.3889574Z ##[section]Starting: Checkout
2019-10-23T17:53:51.3891333Z ==============================================================================
2019-10-23T17:53:51.3891401Z Task         : Get sources
2019-10-23T17:53:51.3891466Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
