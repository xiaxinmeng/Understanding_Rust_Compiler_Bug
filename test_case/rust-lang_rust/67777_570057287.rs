plain
2020-01-01T14:13:39.3981272Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-01T14:13:40.0784711Z ##[command]git config gc.auto 0
2020-01-01T14:13:40.0787610Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-01T14:13:40.0790076Z ##[command]git config --get-all http.proxy
2020-01-01T14:13:40.0793112Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67777/merge:refs/remotes/pull/67777/merge
---
2020-01-01T14:20:50.2725825Z 
2020-01-01T14:20:50.2784396Z error: could not compile `rustc_data_structures`.
2020-01-01T14:20:50.2797751Z warning: build failed, waiting for other jobs to finish...
2020-01-01T14:20:59.3168270Z error: build failed
2020-01-01T14:20:59.3196170Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-01T14:20:59.3209685Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-01T14:20:59.3210084Z Build completed unsuccessfully in 0:04:00
2020-01-01T14:20:59.3263271Z == clock drift check ==
2020-01-01T14:20:59.3280555Z   local time: Wed Jan  1 14:20:59 UTC 2020
2020-01-01T14:20:59.3280555Z   local time: Wed Jan  1 14:20:59 UTC 2020
2020-01-01T14:20:59.6077734Z   network time: Wed, 01 Jan 2020 14:20:59 GMT
2020-01-01T14:20:59.6081421Z == end clock drift check ==
2020-01-01T14:21:00.7439182Z 
2020-01-01T14:21:00.7550484Z ##[error]Bash exited with code '1'.
2020-01-01T14:21:00.7577798Z ##[section]Starting: Checkout
2020-01-01T14:21:00.7579488Z ==============================================================================
2020-01-01T14:21:00.7579544Z Task         : Get sources
2020-01-01T14:21:00.7579589Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
