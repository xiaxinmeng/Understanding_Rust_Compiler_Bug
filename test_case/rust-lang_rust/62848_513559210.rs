plain
2019-07-21T14:20:23.5596994Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T14:20:23.5767454Z ##[command]git config gc.auto 0
2019-07-21T14:20:23.5830742Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T14:20:23.5883742Z ##[command]git config --get-all http.proxy
2019-07-21T14:20:23.6018705Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62848/merge:refs/remotes/pull/62848/merge
---
2019-07-21T14:20:56.8300515Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T14:20:56.8301706Z 
2019-07-21T14:20:56.8302789Z   git checkout -b <new-branch-name>
2019-07-21T14:20:56.8304296Z 
2019-07-21T14:20:56.8304915Z HEAD is now at e025b9e49 Merge c273da301a627202f5a9f294ddb0c6dee32cc9c5 into 83dfe7b27cf2debecebedd3b038f9a1c2e05e051
2019-07-21T14:20:56.8446049Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T14:20:56.8450201Z ==============================================================================
2019-07-21T14:20:56.8450283Z Task         : Bash
2019-07-21T14:20:56.8450330Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-21T14:26:17.5438295Z     Checking hashbrown v0.4.0
2019-07-21T14:26:22.9165185Z error[E0635]: unknown feature `rustc_private`
2019-07-21T14:26:22.9165639Z    --> src/libstd/lib.rs:291:12
2019-07-21T14:26:22.9165814Z     |
2019-07-21T14:26:22.9166041Z 291 | #![feature(rustc_private)]
2019-07-21T14:26:22.9166460Z 
2019-07-21T14:26:23.0230887Z error: aborting due to previous error
2019-07-21T14:26:23.0231016Z 
2019-07-21T14:26:23.0231323Z For more information about this error, try `rustc --explain E0635`.
2019-07-21T14:26:23.0231323Z For more information about this error, try `rustc --explain E0635`.
2019-07-21T14:26:23.0744958Z error: Could not compile `std`.
2019-07-21T14:26:23.0745035Z 
2019-07-21T14:26:23.0745306Z To learn more, run the command again with --verbose.
2019-07-21T14:26:23.0784168Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-21T14:26:23.0787599Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-21T14:26:23.0787655Z Build completed unsuccessfully in 0:02:37
2019-07-21T14:26:23.0787655Z Build completed unsuccessfully in 0:02:37
2019-07-21T14:26:24.8523697Z ##[error]Bash exited with code '1'.
2019-07-21T14:26:24.8558308Z ##[section]Starting: Checkout
2019-07-21T14:26:24.8560448Z ==============================================================================
2019-07-21T14:26:24.8560506Z Task         : Get sources
2019-07-21T14:26:24.8560596Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
