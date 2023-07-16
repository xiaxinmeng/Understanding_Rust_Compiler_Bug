plain
2019-07-23T07:12:48.6108557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T07:12:48.6315052Z ##[command]git config gc.auto 0
2019-07-23T07:12:48.6388598Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T07:12:48.6437568Z ##[command]git config --get-all http.proxy
2019-07-23T07:12:48.6578087Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62871/merge:refs/remotes/pull/62871/merge
---
2019-07-23T07:13:22.5131224Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T07:13:22.5131248Z 
2019-07-23T07:13:22.5131416Z   git checkout -b <new-branch-name>
2019-07-23T07:13:22.5131438Z 
2019-07-23T07:13:22.5131475Z HEAD is now at d562263f6 Merge b39315eba5a5f60fa511a3931994dd4d8641d9de into e649e903440bfd919bfc9db848c28df6d795a116
2019-07-23T07:13:22.5255068Z ##[section]Finishing: Checkout
2019-07-23T07:13:22.5261924Z ##[section]Starting: Decide whether to run this job
2019-07-23T07:13:22.5264156Z Task         : Bash
2019-07-23T07:13:22.5264191Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T07:13:22.5264243Z Version      : 3.151.3
2019-07-23T07:13:22.5264277Z Author       : Microsoft Corporation
2019-07-23T07:13:22.5264277Z Author       : Microsoft Corporation
2019-07-23T07:13:22.5264317Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-23T07:13:22.5264384Z ==============================================================================
2019-07-23T07:13:22.6495999Z Generating script.
2019-07-23T07:13:22.6528804Z ========================== Starting Command Output ===========================
2019-07-23T07:13:22.6546656Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3198d723-ce2f-475e-ab5f-e0bd123aa667.sh
2019-07-23T07:13:22.8907243Z Executing the job since submodules are updated
2019-07-23T07:13:22.8990863Z ##[section]Finishing: Decide whether to run this job
2019-07-23T07:13:22.9001006Z ==============================================================================
2019-07-23T07:13:22.9001093Z Task         : Bash
2019-07-23T07:13:22.9001134Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T07:13:22.9001171Z Version      : 3.151.3
---
2019-07-23T07:16:12.2824860Z ################################                                          45.3%
2019-07-23T07:16:12.2825372Z ######################################################################## 100.0%
2019-07-23T07:16:13.6735508Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-23T07:16:13.7086247Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:16:13.7375024Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-07-23T07:16:13.7375147Z Caused by:
2019-07-23T07:16:13.7375147Z Caused by:
2019-07-23T07:16:13.7375627Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:16:13.7381858Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:16:13.7381942Z Build completed unsuccessfully in 0:00:17
2019-07-23T07:16:13.7425328Z make: *** [prepare] Error 1
2019-07-23T07:16:13.7426096Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:16:14.8094600Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:16:14.8094600Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:16:14.8372763Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-07-23T07:16:14.8373677Z Caused by:
2019-07-23T07:16:14.8373677Z Caused by:
2019-07-23T07:16:14.8374591Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:16:14.8382085Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:16:14.8382545Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:16:14.8416829Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:16:14.8417245Z make: *** [prepare] Error 1
2019-07-23T07:16:17.4155686Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:16:17.4156287Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-23T07:16:17.4156590Z 
2019-07-23T07:16:17.4157043Z Caused by:
2019-07-23T07:16:17.4157043Z Caused by:
2019-07-23T07:16:17.4158043Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:16:17.4158697Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:16:17.4159005Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:16:17.4159523Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:16:17.4159814Z make: *** [prepare] Error 1
2019-07-23T07:16:20.0193126Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:16:20.0484893Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-23T07:16:20.0485283Z 
2019-07-23T07:16:20.0485630Z Caused by:
2019-07-23T07:16:20.0485630Z Caused by:
2019-07-23T07:16:20.0486071Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:16:20.0489893Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:16:20.0490747Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:16:20.0519218Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:16:20.0519702Z make: *** [prepare] Error 1
2019-07-23T07:16:24.1196480Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-23T07:16:24.1474541Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-23T07:16:24.1475218Z 
2019-07-23T07:16:24.1475434Z Caused by:
2019-07-23T07:16:24.1475434Z Caused by:
2019-07-23T07:16:24.1476131Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-23T07:16:24.1481905Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-23T07:16:24.1482405Z Build completed unsuccessfully in 0:00:00
2019-07-23T07:16:24.1515464Z Makefile:69: recipe for target 'prepare' failed
2019-07-23T07:16:24.1517100Z make: *** [prepare] Error 1
2019-07-23T07:16:24.1517467Z The command has failed after 5 attempts.
2019-07-23T07:16:35.1315217Z ##[error]Bash exited with code '1'.
2019-07-23T07:16:35.1346163Z ##[section]Starting: Checkout
2019-07-23T07:16:35.1347832Z ==============================================================================
2019-07-23T07:16:35.1347876Z Task         : Get sources
2019-07-23T07:16:35.1347914Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
