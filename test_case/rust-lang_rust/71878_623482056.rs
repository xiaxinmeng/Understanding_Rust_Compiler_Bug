plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0d465b97-228d-48cb-a38b-d5ec698e215f.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71878/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71878/merge:refs/remotes/pull/71878/merge
---
 ---> f7353ccad5b1
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> ed38efbaa060
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
 ---> c5008ef7ae8e
Successfully built c5008ef7ae8e
Successfully tagged rust-ci:latest
Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
---
    Checking alloc v0.0.0 (/checkout/src/liballoc)
    Checking rustc-demangle v0.1.16
    Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
    Checking backtrace v0.3.46
error[E0596]: cannot borrow `unlinked_node` as mutable, as it is not declared as mutable
     |
     |
1508 |         let unlinked_node = self.current?;
     |             ------------- help: consider changing this to be mutable: `mut unlinked_node`
...
1513 |             unlinked_node.as_mut().prev = None;


error[E0596]: cannot borrow `unlinked_node` as mutable, as it is not declared as mutable
     |
     |
1508 |         let unlinked_node = self.current?;
     |             ------------- help: consider changing this to be mutable: `mut unlinked_node`
1514 |             unlinked_node.as_mut().next = None;
     |             ^^^^^^^^^^^^^ cannot borrow as mutable

error: aborting due to 2 previous errors
---
  local time: Mon May  4 14:00:47 UTC 2020
  network time: Mon, 04 May 2020 14:00:47 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71878/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71878/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (5621) (python)
##[section]Finishing: Finalize Job
