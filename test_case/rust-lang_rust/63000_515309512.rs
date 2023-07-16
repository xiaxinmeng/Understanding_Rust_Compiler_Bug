plain
2019-07-26T04:53:33.2463364Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T04:53:33.2637895Z ##[command]git config gc.auto 0
2019-07-26T04:53:33.2713324Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T04:53:33.2769909Z ##[command]git config --get-all http.proxy
2019-07-26T04:53:33.2898534Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63000/merge:refs/remotes/pull/63000/merge
---
2019-07-26T04:54:07.3526987Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T04:54:07.3527576Z 
2019-07-26T04:54:07.3528133Z   git checkout -b <new-branch-name>
2019-07-26T04:54:07.3528336Z 
2019-07-26T04:54:07.3528510Z HEAD is now at ada23c098 Merge 8b5ce839308ee6716e3b131c7cc5cc32ab00ffe8 into 18630677cf6c7ac50e6786c504b35bc09501dbe2
2019-07-26T04:54:07.3669083Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T04:54:07.3673918Z ==============================================================================
2019-07-26T04:54:07.3673980Z Task         : Bash
2019-07-26T04:54:07.3674046Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T04:59:08.9144728Z    Compiling autocfg v0.1.4
2019-07-26T04:59:10.4998067Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-26T04:59:12.0668261Z    Compiling compiler_builtins v0.1.17
2019-07-26T04:59:14.6048753Z    Compiling cmake v0.1.38
2019-07-26T04:59:15.0101461Z error[E0711]: feature `rust1` is declared stable since 1.38.0, but was previously declared stable since 1.0.0
2019-07-26T04:59:15.0115520Z     |
2019-07-26T04:59:15.0121924Z 603 | #[stable(feature = "rust1", since = "1.38.0")]
2019-07-26T04:59:15.0183414Z     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-26T04:59:15.0183513Z 
2019-07-26T04:59:15.0183513Z 
2019-07-26T04:59:15.5201722Z error: aborting due to previous error
2019-07-26T04:59:15.5208033Z 
2019-07-26T04:59:15.6610354Z error: Could not compile `core`.
2019-07-26T04:59:15.6610667Z warning: build failed, waiting for other jobs to finish...
2019-07-26T04:59:16.8503467Z error: build failed
2019-07-26T04:59:16.8525799Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-26T04:59:16.8536350Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-26T04:59:16.8536656Z Build completed unsuccessfully in 0:02:11
2019-07-26T04:59:16.8536656Z Build completed unsuccessfully in 0:02:11
2019-07-26T04:59:24.2011634Z ##[error]Bash exited with code '1'.
2019-07-26T04:59:24.2044883Z ##[section]Starting: Checkout
2019-07-26T04:59:24.2046490Z ==============================================================================
2019-07-26T04:59:24.2046710Z Task         : Get sources
2019-07-26T04:59:24.2046781Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
