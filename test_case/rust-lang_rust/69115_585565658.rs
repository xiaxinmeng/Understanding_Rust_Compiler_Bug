plain
2020-02-13T03:49:38.9748912Z ========================== Starting Command Output ===========================
2020-02-13T03:49:38.9751407Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/320d40ac-f42f-4763-86e0-6de9b9f73ed2.sh
2020-02-13T03:49:38.9751452Z 
2020-02-13T03:49:38.9754410Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-13T03:49:38.9760373Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69115/merge to s
2020-02-13T03:49:38.9762108Z Task         : Get sources
2020-02-13T03:49:38.9762139Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T03:49:38.9762182Z Version      : 1.0.0
2020-02-13T03:49:38.9762212Z Author       : Microsoft
---
2020-02-13T03:49:39.7466023Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-13T03:49:39.7542246Z ##[command]git config gc.auto 0
2020-02-13T03:49:39.7606403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-13T03:49:39.7636232Z ##[command]git config --get-all http.proxy
2020-02-13T03:49:39.7800343Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69115/merge:refs/remotes/pull/69115/merge
---
2020-02-13T05:28:24.8433972Z expected success, got: exit code: 101
2020-02-13T05:28:24.8434130Z 
2020-02-13T05:28:24.8434201Z 
2020-02-13T05:28:24.8434239Z  finished in 0.002
2020-02-13T05:30:48.5257246Z Timeout for link `https://web.njit.edu/~dingxn/papers/BWS.pdf`
2020-02-13T05:30:48.5280587Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-02-13T05:30:48.5466529Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2020-02-13T05:30:49.9406730Z    Compiling proc-macro2 v0.4.30
2020-02-13T05:30:49.9407724Z    Compiling unicode-xid v0.1.0
---
2020-02-13T05:59:53.2356331Z test [compile-fail] compile-fail/exact_div3.rs ... ok
2020-02-13T05:59:53.3591962Z 
2020-02-13T05:59:53.3592844Z error: compile-fail test compiled successfully!
2020-02-13T05:59:53.3592957Z status: exit code: 0
2020-02-13T05:59:53.3593635Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/exact_div4.rs" "-L" "/tmp/compiletesttwOKuB" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesttwOKuB/exact_div4.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesttwOKuB/exact_div4.stage-id.aux" "-A" "unused"
2020-02-13T05:59:53.3593949Z ------------------------------------------
2020-02-13T05:59:53.3593979Z 
2020-02-13T05:59:53.3594203Z ------------------------------------------
2020-02-13T05:59:53.3594246Z stderr:
---
2020-02-13T06:00:03.5030521Z 
2020-02-13T06:00:03.5030805Z If you do intend to update 'book', please check the error messages above and
2020-02-13T06:00:03.5030862Z commit another update.
2020-02-13T06:00:03.5030892Z 
2020-02-13T06:00:03.5031149Z If you do NOT intend to update 'book', please ensure you did not accidentally
2020-02-13T06:00:03.5031449Z change the submodule at 'src/doc/book'. You may ask your reviewer for the
2020-02-13T06:00:03.5031503Z proper steps.
2020-02-13T06:00:03.5038767Z Build completed unsuccessfully in 0:00:01
2020-02-13T06:00:03.5089962Z == clock drift check ==
2020-02-13T06:00:03.5109938Z   local time: Thu Feb 13 06:00:03 UTC 2020
2020-02-13T06:00:03.7959939Z   network time: Thu, 13 Feb 2020 06:00:03 GMT
2020-02-13T06:00:03.7959939Z   network time: Thu, 13 Feb 2020 06:00:03 GMT
2020-02-13T06:00:03.7962091Z == end clock drift check ==
2020-02-13T06:00:04.1786030Z 
2020-02-13T06:00:04.1893710Z ##[error]Bash exited with code '1'.
2020-02-13T06:00:04.1907386Z ##[section]Finishing: Run build
2020-02-13T06:00:04.1930130Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69115/merge to s
2020-02-13T06:00:04.1932139Z Task         : Get sources
2020-02-13T06:00:04.1932188Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T06:00:04.1932236Z Version      : 1.0.0
2020-02-13T06:00:04.1932298Z Author       : Microsoft
2020-02-13T06:00:04.1932298Z Author       : Microsoft
2020-02-13T06:00:04.1932347Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-13T06:00:04.1932398Z ==============================================================================
2020-02-13T06:00:04.6360459Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-13T06:00:04.6401825Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69115/merge to s
2020-02-13T06:00:04.6516283Z Cleaning up task key
2020-02-13T06:00:04.6517079Z Start cleaning up orphan processes.
2020-02-13T06:00:04.6633336Z Terminate orphan process: pid (4689) (python)
2020-02-13T06:00:04.6883114Z ##[section]Finishing: Finalize Job
