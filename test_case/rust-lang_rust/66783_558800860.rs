plain
2019-11-26T17:48:33.6823365Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T17:48:34.4627274Z ##[command]git config gc.auto 0
2019-11-26T17:48:34.4632125Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T17:48:34.4634259Z ##[command]git config --get-all http.proxy
2019-11-26T17:48:34.4636456Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66783/merge:refs/remotes/pull/66783/merge
---
2019-11-26T17:51:53.9179708Z #######################################                                   54.3%
2019-11-26T17:51:53.9655007Z ##############################################################            87.2%
2019-11-26T17:51:53.9655709Z ######################################################################## 100.0%
2019-11-26T17:51:54.3436536Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-26T17:51:54.3835979Z warning: patch for the non root package will be ignored, specify patch at the workspace root:
2019-11-26T17:51:54.3836237Z package:   /checkout/src/tools/cargo/Cargo.toml
2019-11-26T17:51:54.3838389Z workspace: /checkout/Cargo.toml
2019-11-26T17:51:54.4062713Z     Updating git repository `https://github.com/Mark-Simulacrum/jobserver-rs`
2019-11-26T17:51:54.8941809Z     Updating git repository `https://github.com/spastorino/rustc-rayon`
---
2019-11-26T17:52:33.9348576Z    Compiling serde_json v1.0.40
2019-11-26T17:52:34.2746193Z    Compiling toml v0.5.3
2019-11-26T17:52:37.0771600Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-11-26T17:53:10.1364351Z     Finished dev [unoptimized] target(s) in 1m 15s
2019-11-26T17:53:10.2525492Z warning: patch for the non root package will be ignored, specify patch at the workspace root:
2019-11-26T17:53:10.2525632Z package:   /checkout/src/tools/cargo/Cargo.toml
2019-11-26T17:53:10.2526509Z workspace: /checkout/Cargo.toml
---
2019-11-26T17:54:40.5825810Z * highest error code: E0745
2019-11-26T17:54:40.9475834Z * 273 features
2019-11-26T17:54:41.5785877Z Dependencies not on the whitelist:
2019-11-26T17:54:41.5787216Z * crossbeam-channel 
2019-11-26T17:54:41.5791296Z invalid source: "git+https://github.com/Mark-Simulacrum/jobserver-rs?branch=sem#3ec169d99340c3c41fd383a537f225d44f99e374"
2019-11-26T17:54:41.5793850Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#dd85f978633ae564515b7d1c82ba3d1cff76b1f1"
2019-11-26T17:54:41.5794625Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#dd85f978633ae564515b7d1c82ba3d1cff76b1f1"
2019-11-26T17:54:41.8258483Z some tidy checks failed
2019-11-26T17:54:41.8259425Z Found 486 error codes
2019-11-26T17:54:41.8259884Z Found 0 error codes with no tests
2019-11-26T17:54:41.8259962Z Done!
2019-11-26T17:54:41.8259962Z Done!
2019-11-26T17:54:41.8259990Z 
2019-11-26T17:54:41.8260125Z 
2019-11-26T17:54:41.8261172Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-26T17:54:41.8262937Z 
2019-11-26T17:54:41.8263012Z 
2019-11-26T17:54:41.8263237Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-26T17:54:41.8263322Z Build completed unsuccessfully in 0:01:28
2019-11-26T17:54:41.8263322Z Build completed unsuccessfully in 0:01:28
2019-11-26T17:54:41.8314639Z == clock drift check ==
2019-11-26T17:54:41.8323597Z   local time: Tue Nov 26 17:54:41 UTC 2019
2019-11-26T17:54:41.9884827Z   network time: Tue, 26 Nov 2019 17:54:41 GMT
2019-11-26T17:54:41.9890008Z == end clock drift check ==
2019-11-26T17:54:43.2991916Z 
2019-11-26T17:54:43.3092650Z ##[error]Bash exited with code '1'.
2019-11-26T17:54:43.3118576Z ##[section]Starting: Checkout
2019-11-26T17:54:43.3120449Z ==============================================================================
2019-11-26T17:54:43.3120499Z Task         : Get sources
2019-11-26T17:54:43.3120540Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
