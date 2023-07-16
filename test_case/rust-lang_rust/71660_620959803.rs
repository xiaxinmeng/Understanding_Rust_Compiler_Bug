plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dea64614-775c-4b33-8ca0-a9001ded4fd4.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71660/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71660/merge:refs/remotes/pull/71660/merge
---
Looks like docker image is the same as before, not uploading
[CI_JOB_NAME=x86_64-gnu-tools]
[CI_JOB_NAME=x86_64-gnu-tools]
== clock drift check ==
  local time: Wed Apr 29 02:35:46 UTC 2020
  network time: Wed, 29 Apr 2020 02:35:46 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.channel         := nightly
---
extracting /checkout/obj/build/cache/2020-04-22/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
error: failed to resolve patches for `https://github.com/rust-lang/cargo`

Caused by:
  patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
Build completed unsuccessfully in 0:00:21
Makefile:67: recipe for target 'prepare' failed
make: *** [prepare] Error 1
Command failed. Attempt 2/5:
Command failed. Attempt 2/5:
error: failed to resolve patches for `https://github.com/rust-lang/cargo`

Caused by:
  patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
Build completed unsuccessfully in 0:00:00
Makefile:67: recipe for target 'prepare' failed
make: *** [prepare] Error 1
Command failed. Attempt 3/5:
Command failed. Attempt 3/5:
error: failed to resolve patches for `https://github.com/rust-lang/cargo`

Caused by:
  patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:67: recipe for target 'prepare' failed
Command failed. Attempt 4/5:
Command failed. Attempt 4/5:
error: failed to resolve patches for `https://github.com/rust-lang/cargo`

Caused by:
  patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
Build completed unsuccessfully in 0:00:00
Makefile:67: recipe for target 'prepare' failed
make: *** [prepare] Error 1
Command failed. Attempt 5/5:
Command failed. Attempt 5/5:
error: failed to resolve patches for `https://github.com/rust-lang/cargo`

Caused by:
  patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
Build completed unsuccessfully in 0:00:00
Makefile:67: recipe for target 'prepare' failed
make: *** [prepare] Error 1
The command has failed after 5 attempts.
The command has failed after 5 attempts.
== clock drift check ==
  local time: Wed Apr 29 02:36:18 UTC 2020
  network time: Wed, 29 Apr 2020 02:36:18 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71660/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71660/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3491) (python)
##[section]Finishing: Finalize Job
