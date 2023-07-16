plain
2019-08-21T07:04:31.8441094Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-21T07:04:31.8658331Z ##[command]git config gc.auto 0
2019-08-21T07:04:31.8710476Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-21T07:04:31.8761884Z ##[command]git config --get-all http.proxy
2019-08-21T07:04:31.8889792Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63347/merge:refs/remotes/pull/63347/merge
---
2019-08-21T07:05:07.1661630Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-21T07:05:07.1661654Z 
2019-08-21T07:05:07.1661805Z   git checkout -b <new-branch-name>
2019-08-21T07:05:07.1661843Z 
2019-08-21T07:05:07.1661879Z HEAD is now at bac726845 Merge 42ebadb55a633ca6921ee884443cc5311c622351 into bea0372a1a7a31b81f28cc4d9a83a2dc9a79d008
2019-08-21T07:05:07.1800452Z ##[section]Finishing: Checkout
2019-08-21T07:05:07.1806308Z ##[section]Starting: Decide whether to run this job
2019-08-21T07:05:07.1810042Z Task         : Bash
2019-08-21T07:05:07.1810080Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-21T07:05:07.1810145Z Version      : 3.151.3
2019-08-21T07:05:07.1810180Z Author       : Microsoft Corporation
2019-08-21T07:05:07.1810180Z Author       : Microsoft Corporation
2019-08-21T07:05:07.1810218Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-08-21T07:05:07.1810276Z ==============================================================================
2019-08-21T07:05:07.2964716Z Generating script.
2019-08-21T07:05:07.2993248Z ========================== Starting Command Output ===========================
2019-08-21T07:05:07.3014292Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/07629fb4-f683-4fb5-bb68-94d9149d589b.sh
2019-08-21T07:05:07.3931402Z Executing the job since submodules are updated
2019-08-21T07:05:07.4010418Z ##[section]Finishing: Decide whether to run this job
2019-08-21T07:05:07.4018724Z ==============================================================================
2019-08-21T07:05:07.4018832Z Task         : Bash
2019-08-21T07:05:07.4019053Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-21T07:05:07.4019101Z Version      : 3.151.3
---
2019-08-21T07:07:51.7828431Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-21T07:07:51.7828674Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-08-21T07:07:51.7828857Z 
2019-08-21T07:07:51.7829029Z Caused by:
2019-08-21T07:07:51.7829390Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-08-21T07:07:51.7829680Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-21T07:07:51.7829730Z Build completed unsuccessfully in 0:00:20
2019-08-21T07:07:51.7830154Z Makefile:67: recipe for target 'prepare' failed
2019-08-21T07:07:52.3909397Z Command failed. Attempt 2/5:
2019-08-21T07:07:52.4742461Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-08-21T07:07:52.4742966Z 
2019-08-21T07:07:52.4743210Z Caused by:
2019-08-21T07:07:52.4743210Z Caused by:
2019-08-21T07:07:52.4744517Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-08-21T07:07:52.4750708Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-21T07:07:52.4751219Z Build completed unsuccessfully in 0:00:00
2019-08-21T07:07:52.4781714Z Makefile:67: recipe for target 'prepare' failed
2019-08-21T07:07:54.4798206Z Command failed. Attempt 3/5:
2019-08-21T07:07:54.5622200Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-08-21T07:07:54.5622343Z 
2019-08-21T07:07:54.5622397Z Caused by:
2019-08-21T07:07:54.5622397Z Caused by:
2019-08-21T07:07:54.5622811Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-08-21T07:07:54.5627425Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-21T07:07:54.5654712Z make: *** [prepare] Error 1
2019-08-21T07:07:54.5654712Z make: *** [prepare] Error 1
2019-08-21T07:07:54.5655305Z Makefile:67: recipe for target 'prepare' failed
2019-08-21T07:07:57.6485507Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-08-21T07:07:57.6485639Z 
2019-08-21T07:07:57.6485733Z Caused by:
2019-08-21T07:07:57.6485733Z Caused by:
2019-08-21T07:07:57.6486075Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-08-21T07:07:57.6496040Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-21T07:07:57.6528947Z make: *** [prepare] Error 1
2019-08-21T07:07:57.6528947Z make: *** [prepare] Error 1
2019-08-21T07:07:57.6530122Z Makefile:67: recipe for target 'prepare' failed
2019-08-21T07:08:01.7378842Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-08-21T07:08:01.7379412Z 
2019-08-21T07:08:01.7379988Z Caused by:
2019-08-21T07:08:01.7379988Z Caused by:
2019-08-21T07:08:01.7380781Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-08-21T07:08:01.7385306Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-08-21T07:08:01.7421711Z make: *** [prepare] Error 1
2019-08-21T07:08:01.7421711Z make: *** [prepare] Error 1
2019-08-21T07:08:01.7422595Z Makefile:67: recipe for target 'prepare' failed
2019-08-21T07:08:01.7426938Z == clock drift check ==
2019-08-21T07:08:01.7431406Z   local time: Wed Aug 21 07:08:01 UTC 2019
2019-08-21T07:08:01.7949548Z   network time: Wed, 21 Aug 2019 07:08:01 GMT
2019-08-21T07:08:01.7953515Z == end clock drift check ==
2019-08-21T07:08:01.7953515Z == end clock drift check ==
2019-08-21T07:08:17.9728639Z ##[error]Bash exited with code '1'.
2019-08-21T07:08:17.9781448Z ##[section]Starting: Checkout
2019-08-21T07:08:17.9783071Z ==============================================================================
2019-08-21T07:08:17.9783127Z Task         : Get sources
2019-08-21T07:08:17.9783185Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
