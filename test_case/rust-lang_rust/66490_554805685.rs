plain
2019-11-18T00:22:06.0893739Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T00:22:06.1109470Z ##[command]git config gc.auto 0
2019-11-18T00:22:06.1205465Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T00:22:06.1250572Z ##[command]git config --get-all http.proxy
2019-11-18T00:22:06.1394103Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66490/merge:refs/remotes/pull/66490/merge
---
2019-11-18T00:25:18.3647812Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-18T00:25:18.4315056Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:18.4315255Z 
2019-11-18T00:25:18.4315298Z Caused by:
2019-11-18T00:25:18.4315651Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T00:25:18.4328481Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T00:25:18.4382187Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T00:25:18.4382317Z make: *** [prepare] Error 1
2019-11-18T00:25:19.4406392Z Command failed. Attempt 2/5:
2019-11-18T00:25:19.5399071Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:19.5399071Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:19.5399178Z 
2019-11-18T00:25:19.5399233Z Caused by:
2019-11-18T00:25:19.5399725Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T00:25:19.5408380Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T00:25:19.5452588Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T00:25:19.5452690Z make: *** [prepare] Error 1
2019-11-18T00:25:21.5472298Z Command failed. Attempt 3/5:
2019-11-18T00:25:21.6488350Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:21.6488350Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:21.6489443Z 
2019-11-18T00:25:21.6489694Z Caused by:
2019-11-18T00:25:21.6490704Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T00:25:21.6495970Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T00:25:21.6543937Z make: *** [prepare] Error 1
2019-11-18T00:25:21.6544537Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T00:25:25.0712771Z Command failed. Attempt 4/5:
2019-11-18T00:25:25.0714607Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:25.0714607Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:25.0714808Z 
2019-11-18T00:25:25.0714977Z Caused by:
2019-11-18T00:25:25.0715890Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T00:25:25.0716382Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T00:25:25.0716685Z make: *** [prepare] Error 1
2019-11-18T00:25:25.0717020Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T00:25:28.7674699Z Command failed. Attempt 5/5:
2019-11-18T00:25:28.8770819Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:28.8770819Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T00:25:28.8772140Z 
2019-11-18T00:25:28.8772604Z Caused by:
2019-11-18T00:25:28.8773392Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T00:25:28.8780262Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T00:25:28.8817264Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T00:25:28.8821086Z make: *** [prepare] Error 1
2019-11-18T00:25:28.8829907Z The command has failed after 5 attempts.
2019-11-18T00:25:28.8844759Z == clock drift check ==
2019-11-18T00:25:28.8844759Z == clock drift check ==
2019-11-18T00:25:28.8845105Z   local time: Mon Nov 18 00:25:28 UTC 2019
2019-11-18T00:25:28.9689009Z   network time: Mon, 18 Nov 2019 00:25:28 GMT
2019-11-18T00:25:28.9689998Z == end clock drift check ==
2019-11-18T00:25:41.3243032Z 
2019-11-18T00:25:41.3350402Z ##[error]Bash exited with code '1'.
2019-11-18T00:25:41.3379415Z ##[section]Starting: Checkout
2019-11-18T00:25:41.3381177Z ==============================================================================
2019-11-18T00:25:41.3381233Z Task         : Get sources
2019-11-18T00:25:41.3381303Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
