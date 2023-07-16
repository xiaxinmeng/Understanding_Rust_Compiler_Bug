plain
2020-02-23T22:20:54.7074819Z ========================== Starting Command Output ===========================
2020-02-23T22:20:54.7077606Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2f30868a-1a1b-4308-a747-4e655eb6383f.sh
2020-02-23T22:20:54.7077905Z 
2020-02-23T22:20:54.7081478Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-23T22:20:54.7102256Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69412/merge to s
2020-02-23T22:20:54.7105743Z Task         : Get sources
2020-02-23T22:20:54.7106092Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T22:20:54.7106403Z Version      : 1.0.0
2020-02-23T22:20:54.7106614Z Author       : Microsoft
---
2020-02-23T22:20:55.7188805Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-23T22:20:55.7194464Z ##[command]git config gc.auto 0
2020-02-23T22:20:55.7198172Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-23T22:20:55.7201639Z ##[command]git config --get-all http.proxy
2020-02-23T22:20:55.7212978Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69412/merge:refs/remotes/pull/69412/merge
---
2020-02-23T22:57:30.5829132Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-02-23T22:57:46.4009667Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-02-23T22:57:51.8220480Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-23T22:58:15.4674001Z error: unused attribute
2020-02-23T22:58:15.4676144Z    --> src/librustc/ty/query/plumbing.rs:254:5
2020-02-23T22:58:15.4677525Z 254 |     #[cold]
2020-02-23T22:58:15.4678222Z     |     ^^^^^^^
2020-02-23T22:58:15.4678813Z     |
2020-02-23T22:58:15.4679558Z     = note: `-D unused-attributes` implied by `-D warnings`
---
2020-02-23T22:58:30.1032931Z   local time: Sun Feb 23 22:58:30 UTC 2020
2020-02-23T22:58:30.3999003Z   network time: Sun, 23 Feb 2020 22:58:30 GMT
2020-02-23T22:58:30.3999340Z == end clock drift check ==
2020-02-23T22:58:30.8098915Z 
2020-02-23T22:58:30.8196907Z ##[error]Bash exited with code '1'.
2020-02-23T22:58:30.8213875Z ##[section]Finishing: Run build
2020-02-23T22:58:30.8303192Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69412/merge to s
2020-02-23T22:58:30.8309121Z Task         : Get sources
2020-02-23T22:58:30.8309514Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T22:58:30.8309892Z Version      : 1.0.0
2020-02-23T22:58:30.8310143Z Author       : Microsoft
2020-02-23T22:58:30.8310143Z Author       : Microsoft
2020-02-23T22:58:30.8310551Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-23T22:58:30.8311034Z ==============================================================================
2020-02-23T22:58:31.1670444Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-23T22:58:31.1714420Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69412/merge to s
2020-02-23T22:58:31.1803763Z Cleaning up task key
2020-02-23T22:58:31.1805107Z Start cleaning up orphan processes.
2020-02-23T22:58:31.1961479Z Terminate orphan process: pid (4777) (python)
2020-02-23T22:58:31.2109603Z ##[section]Finishing: Finalize Job
