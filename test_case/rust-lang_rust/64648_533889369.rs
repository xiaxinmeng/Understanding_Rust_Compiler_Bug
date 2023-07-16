plain
2019-09-22T14:42:01.2529528Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-22T14:42:01.2715292Z ##[command]git config gc.auto 0
2019-09-22T14:42:01.2793714Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-22T14:42:01.2835788Z ##[command]git config --get-all http.proxy
2019-09-22T14:42:01.2970973Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64648/merge:refs/remotes/pull/64648/merge
---
2019-09-22T14:48:54.7289938Z     Finished release [optimized] target(s) in 1m 30s
2019-09-22T14:48:54.7369254Z tidy check
2019-09-22T14:48:55.8360410Z * 578 error codes
2019-09-22T14:48:55.8361397Z * highest error code: E0733
2019-09-22T14:48:55.8745605Z tidy error: /checkout/src/libsyntax/feature_gate/active.rs:177: feature omit_gdb_pretty_printer_section is not sorted by since
2019-09-22T14:48:56.9061231Z some tidy checks failed
2019-09-22T14:48:56.9066813Z 
2019-09-22T14:48:56.9066813Z 
2019-09-22T14:48:56.9067725Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-22T14:48:56.9074813Z 
2019-09-22T14:48:56.9074869Z 
2019-09-22T14:48:56.9108178Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-22T14:48:56.9108254Z Build completed unsuccessfully in 0:01:33
2019-09-22T14:48:56.9108254Z Build completed unsuccessfully in 0:01:33
2019-09-22T14:48:56.9124109Z == clock drift check ==
2019-09-22T14:48:56.9139156Z   local time: Sun Sep 22 14:48:56 UTC 2019
2019-09-22T14:48:57.1917902Z   network time: Sun, 22 Sep 2019 14:48:57 GMT
2019-09-22T14:48:57.1923829Z == end clock drift check ==
2019-09-22T14:48:58.5308649Z ##[error]Bash exited with code '1'.
2019-09-22T14:48:58.5340012Z ##[section]Starting: Checkout
2019-09-22T14:48:58.5341686Z ==============================================================================
2019-09-22T14:48:58.5341750Z Task         : Get sources
2019-09-22T14:48:58.5341790Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
