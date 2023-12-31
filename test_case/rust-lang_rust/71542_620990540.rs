plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/585e3c30-e535-4e09-b595-69f203b2fabf.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71542/merge:refs/remotes/pull/71542/merge
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
Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
Looks like docker image is the same as before, not uploading
[CI_JOB_NAME=mingw-check]
[CI_JOB_NAME=mingw-check]
== clock drift check ==
  local time: Wed Apr 29 05:04:20 UTC 2020
  network time: Wed, 29 Apr 2020 05:04:20 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.parallel-compiler := True
---
error: could not compile `core`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
{"reason":"build-finished","success":false}
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:55
Build completed unsuccessfully in 0:01:55
== clock drift check ==
  local time: Wed Apr 29 05:06:15 UTC 2020
  network time: Wed, 29 Apr 2020 05:06:16 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71542/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (7756) (python)
##[section]Finishing: Finalize Job
