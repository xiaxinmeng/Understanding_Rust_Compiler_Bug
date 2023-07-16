plain
2019-07-23T07:31:32.8755428Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T07:31:32.8983792Z ##[command]git config gc.auto 0
2019-07-23T07:31:33.5126716Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T07:31:33.5137319Z ##[command]git config --get-all http.proxy
2019-07-23T07:31:33.5140517Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62871/merge:refs/remotes/pull/62871/merge
---
2019-07-23T07:31:46.1694731Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T07:31:46.1694762Z 
2019-07-23T07:31:46.1694964Z   git checkout -b <new-branch-name>
2019-07-23T07:31:46.1695174Z 
2019-07-23T07:31:46.1695244Z HEAD is now at 52c301331 Merge 491307aa1b908424dca72354acbf5643c1875685 into e649e903440bfd919bfc9db848c28df6d795a116
2019-07-23T07:31:46.1844619Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T07:31:46.1849404Z ==============================================================================
2019-07-23T07:31:46.1849464Z Task         : Bash
2019-07-23T07:31:46.1849978Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T07:34:55.8244569Z #################                                                         23.7%
2019-07-23T07:34:55.8251125Z ######################################################################## 100.0%
2019-07-23T07:34:56.2827488Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-23T07:34:56.3190371Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:34:56.3523406Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-07-23T07:34:56.3523585Z Caused by:
2019-07-23T07:34:56.3523585Z Caused by:
2019-07-23T07:34:56.3524178Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:34:56.3537841Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:34:56.3538409Z Build completed unsuccessfully in 0:00:18
2019-07-23T07:34:56.3582584Z make: *** [prepare] Error 1
2019-07-23T07:34:56.3583296Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:34:57.4389913Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:34:57.4744398Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-23T07:34:57.4744589Z 
2019-07-23T07:34:57.4744632Z Caused by:
2019-07-23T07:34:57.4744632Z Caused by:
2019-07-23T07:34:57.4745010Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:34:57.4755467Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:34:57.4755584Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:34:57.4799235Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:34:57.4799362Z make: *** [prepare] Error 1
2019-07-23T07:34:59.5601084Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:34:59.5601084Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:34:59.5985221Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-07-23T07:34:59.5985716Z Caused by:
2019-07-23T07:34:59.5985716Z Caused by:
2019-07-23T07:34:59.5986177Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:34:59.5995775Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:34:59.5995857Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:34:59.6048814Z make: *** [prepare] Error 1
2019-07-23T07:34:59.6050233Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:35:02.6935882Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:35:02.7304177Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-23T07:35:02.7304266Z 
2019-07-23T07:35:02.7304322Z Caused by:
2019-07-23T07:35:02.7304322Z Caused by:
2019-07-23T07:35:02.7304852Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:35:02.7314030Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:35:02.7314198Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:35:02.7363044Z make: *** [prepare] Error 1
2019-07-23T07:35:02.7363540Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:35:06.8194042Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:35:06.8567529Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-23T07:35:06.8567664Z 
2019-07-23T07:35:06.8567721Z Caused by:
2019-07-23T07:35:06.8567721Z Caused by:
2019-07-23T07:35:06.8568400Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:35:06.8576651Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:35:06.8576742Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:35:06.8624834Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:35:06.8624924Z make: *** [prepare] Error 1
2019-07-23T07:35:06.8629182Z The command has failed after 5 attempts.
2019-07-23T07:35:17.7691573Z ##[error]Bash exited with code '1'.
2019-07-23T07:35:17.7741525Z ##[section]Starting: Checkout
2019-07-23T07:35:17.7743155Z ==============================================================================
2019-07-23T07:35:17.7743218Z Task         : Get sources
2019-07-23T07:35:17.7743265Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
