plain
2019-07-23T07:37:55.1075714Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T07:37:55.1304527Z ##[command]git config gc.auto 0
2019-07-23T07:37:55.1374125Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T07:37:55.1443294Z ##[command]git config --get-all http.proxy
2019-07-23T07:37:55.1585534Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62871/merge:refs/remotes/pull/62871/merge
---
2019-07-23T07:38:30.5250067Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T07:38:30.5250203Z 
2019-07-23T07:38:30.5250548Z   git checkout -b <new-branch-name>
2019-07-23T07:38:30.5250706Z 
2019-07-23T07:38:30.5250825Z HEAD is now at 606111d88 Merge d1c15469db07dcbc2e342a76ecefd000d0e1ec05 into e649e903440bfd919bfc9db848c28df6d795a116
2019-07-23T07:38:30.5380605Z ##[section]Finishing: Checkout
2019-07-23T07:38:30.5387423Z ##[section]Starting: Decide whether to run this job
2019-07-23T07:38:30.5389756Z Task         : Bash
2019-07-23T07:38:30.5389810Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T07:38:30.5389847Z Version      : 3.151.3
2019-07-23T07:38:30.5389893Z Author       : Microsoft Corporation
2019-07-23T07:38:30.5389893Z Author       : Microsoft Corporation
2019-07-23T07:38:30.5389951Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-23T07:38:30.5389991Z ==============================================================================
2019-07-23T07:38:30.6760794Z Generating script.
2019-07-23T07:38:30.6797579Z ========================== Starting Command Output ===========================
2019-07-23T07:38:30.6823799Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d35b9fd4-244a-4c68-ab47-1731d2089be3.sh
2019-07-23T07:38:30.9341911Z Executing the job since submodules are updated
2019-07-23T07:38:30.9431305Z ##[section]Finishing: Decide whether to run this job
2019-07-23T07:38:30.9441706Z ==============================================================================
2019-07-23T07:38:30.9441767Z Task         : Bash
2019-07-23T07:38:30.9441862Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T07:38:30.9441937Z Version      : 3.151.3
---
2019-07-23T07:41:11.2602948Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-23T07:41:11.2971452Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:41:11.3304501Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-23T07:41:11.3304664Z 
2019-07-23T07:41:11.3304710Z Caused by:
2019-07-23T07:41:11.3305083Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:41:11.3313162Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:41:11.3313257Z Build completed unsuccessfully in 0:00:18
2019-07-23T07:41:11.3359068Z make: *** [prepare] Error 1
2019-07-23T07:41:11.3360753Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:41:12.4080074Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:41:12.4404663Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-23T07:41:12.4405437Z 
2019-07-23T07:41:12.4405672Z Caused by:
2019-07-23T07:41:12.4405672Z Caused by:
2019-07-23T07:41:12.4495167Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:41:12.4495567Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:41:12.4495889Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:41:12.4496145Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:41:12.4496188Z make: *** [prepare] Error 1
2019-07-23T07:41:14.5200389Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:41:14.5200389Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:41:14.5522760Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-07-23T07:41:14.5522902Z Caused by:
2019-07-23T07:41:14.5522902Z Caused by:
2019-07-23T07:41:14.5523298Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:41:14.5528848Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:41:14.5528954Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:41:14.5574140Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:41:14.5574210Z make: *** [prepare] Error 1
2019-07-23T07:41:17.6296437Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:41:17.6296437Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:41:17.6612544Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-07-23T07:41:17.6613505Z Caused by:
2019-07-23T07:41:17.6613505Z Caused by:
2019-07-23T07:41:17.6614085Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:41:17.6618062Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:41:17.6618462Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:41:17.6661496Z make: *** [prepare] Error 1
2019-07-23T07:41:17.6662415Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:41:21.7391247Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:41:21.7391247Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:41:21.7706303Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-07-23T07:41:21.7707482Z Caused by:
2019-07-23T07:41:21.7707482Z Caused by:
2019-07-23T07:41:21.7708308Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:41:21.7717689Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:41:21.7718075Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:41:21.7762106Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:41:21.7764909Z make: *** [prepare] Error 1
2019-07-23T07:41:21.7769747Z The command has failed after 5 attempts.
2019-07-23T07:41:32.7405521Z ##[error]Bash exited with code '1'.
2019-07-23T07:41:32.7439901Z ##[section]Starting: Checkout
2019-07-23T07:41:32.7441583Z ==============================================================================
2019-07-23T07:41:32.7441637Z Task         : Get sources
2019-07-23T07:41:32.7441707Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
