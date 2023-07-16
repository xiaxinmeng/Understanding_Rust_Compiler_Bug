plain
2019-09-13T03:09:28.3310029Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-13T03:09:28.3533061Z ##[command]git config gc.auto 0
2019-09-13T03:09:28.3621341Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-13T03:09:28.3677027Z ##[command]git config --get-all http.proxy
2019-09-13T03:09:28.3811492Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64419/merge:refs/remotes/pull/64419/merge
---
2019-09-13T03:16:17.3988134Z    Compiling serde_json v1.0.40
2019-09-13T03:16:20.1888981Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-13T03:16:30.0441083Z     Finished release [optimized] target(s) in 1m 29s
2019-09-13T03:16:30.0523955Z tidy check
2019-09-13T03:16:30.4611708Z tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:572: line longer than 100 chars
2019-09-13T03:16:30.4612088Z tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:576: line longer than 100 chars
2019-09-13T03:16:31.9496502Z some tidy checks failed
2019-09-13T03:16:31.9501289Z 
2019-09-13T03:16:31.9501289Z 
2019-09-13T03:16:31.9502192Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-13T03:16:31.9502316Z 
2019-09-13T03:16:31.9502340Z 
2019-09-13T03:16:31.9564131Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-13T03:16:31.9564825Z Build completed unsuccessfully in 0:01:32
2019-09-13T03:16:31.9564825Z Build completed unsuccessfully in 0:01:32
2019-09-13T03:16:31.9572553Z == clock drift check ==
2019-09-13T03:16:31.9600607Z   local time: Fri Sep 13 03:16:31 UTC 2019
2019-09-13T03:16:32.1106246Z   network time: Fri, 13 Sep 2019 03:16:32 GMT
2019-09-13T03:16:32.1107978Z == end clock drift check ==
2019-09-13T03:16:33.6654950Z ##[error]Bash exited with code '1'.
2019-09-13T03:16:33.6686067Z ##[section]Starting: Checkout
2019-09-13T03:16:33.6688082Z ==============================================================================
2019-09-13T03:16:33.6688142Z Task         : Get sources
2019-09-13T03:16:33.6688179Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
