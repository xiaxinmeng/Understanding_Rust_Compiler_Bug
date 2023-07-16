plain
[00:03:12]  Downloading rustc-ap-rustc_cratesio_shim v128.0.0
[00:03:12]  Downloading ena v0.9.3
[00:03:13]  Downloading parking_lot v0.5.5
[00:03:13]  Downloading byteorder v1.2.2
[00:03:13]  Downloading chalk-engine v0.4.0
[00:03:13]  Downloading rustc-ap-rustc_errors v128.0.0
[00:03:13]  Downloading owning_ref v0.3.3
[00:03:13]  Downloading schannel v0.1.12
[00:03:13]  Downloading globset v0.3.0
---
[00:03:16]  Downloading crossbeam-epoch v0.3.1
[00:03:16]  Downloading itertools v0.6.5
[00:03:16]  Downloading chalk-macros v0.1.0
[00:03:16]  Downloading docopt v0.8.3
[00:03:16]  Downloading lalrpop-intern v0.14.0
[00:03:17]  Downloading stacker v0.1.3
[00:03:17]  Downloading rustyline v1.0.0
[00:03:18]  Downloading fxhash v0.2.1
[00:03:18]  Downloading bitflags v0.9.1
[00:03:18]  Downloading unreachable v1.0.0
[00:03:18]  Downloading utf-8 v0.7.2
---

[00:05:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:45] tidy error: /checkout/src/librustc_traits/chalk_context.rs: incorrect license
[00:05:45] tidy error: /checkout/src/librustc/ty/context.rs:1651: line longer than 100 chars
[00:05:45] tidy error: /checkout/src/librustc/ty/context.rs:1666: line longer than 100 chars
[00:05:46] Dependencies not on the whitelist:
[00:05:46] * chalk-engine 
[00:05:46] * chalk-macros 
[00:05:46] * diff 
[00:05:46] * encode_unicode 
[00:05:46] * error-chain 
[00:05:46] * fixedbitset 
[00:05:46] * fxhash 
[00:05:46] * fxhash 
[00:05:46] * gcc 
[00:05:46] * itertools 
[00:05:46] * lalrpop-intern 
[00:05:46] * nix 
[00:05:46] * petgraph 
[00:05:46] * proc-macro2 
[00:05:46] * quote 
[00:05:46] * rustyline 
---
[00:05:46] * unicode-xid 
[00:05:46] some tidy checks failed
[00:05:46] 
[00:05:46] 
[00:05:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:46] 
[00:05:46] 
[00:05:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:46] Build completed unsuccessfully in 0:02:26
[00:05:46] Build completed unsuccessfully in 0:02:26
[00:05:46] make: *** [tidy] Error 1
[00:05:46] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d706a48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
