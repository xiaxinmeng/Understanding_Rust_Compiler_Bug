plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9be20770-6cda-4585-924a-7d5f48fb8332.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71994/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71994/merge:refs/remotes/pull/71994/merge
---
 ---> 3adb0605cc65
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 28dbc326cb7f
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 537a01811900
Successfully built 537a01811900
Successfully tagged rust-ci:latest
Built container sha256:537a018119009dc218456238dec90b5530050db1e2a1e166550c218003f6159d
---
configure: 
Updating only changed submodules
Submodules updated in 0.02 seconds
Traceback (most recent call last):
  File "../x.py", line 11, in <module>
    bootstrap.main()
  File "/checkout/src/bootstrap/bootstrap.py", line 961, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 927, in bootstrap
    build.download_stage0()
  File "/checkout/src/bootstrap/bootstrap.py", line 377, in download_stage0
    self._download_stage0_helper(filename, pattern, tarball_suffix)
  File "/checkout/src/bootstrap/bootstrap.py", line 426, in _download_stage0_helper
    os.makedirs(rustc_cache)
  File "/usr/lib/python3.5/os.py", line 231, in makedirs
    makedirs(head, mode, exist_ok)
  File "/usr/lib/python3.5/os.py", line 231, in makedirs
    makedirs(head, mode, exist_ok)
  File "/usr/lib/python3.5/os.py", line 241, in makedirs
    mkdir(name, mode)
OSError: [Errno 30] Read-only file system: '/checkout/build'
  local time: Thu May  7 20:59:14 UTC 2020
  network time: Thu, 07 May 2020 20:59:14 GMT
== end clock drift check ==


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71994/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71994/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4378) (python)
##[section]Finishing: Finalize Job
