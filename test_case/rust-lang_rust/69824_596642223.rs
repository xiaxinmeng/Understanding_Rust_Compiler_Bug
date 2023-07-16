plain
2020-03-09T16:34:07.8747587Z Prepare build directory.
2020-03-09T16:34:07.9014315Z Set build variables.
2020-03-09T16:34:07.9041775Z Download all required tasks.
2020-03-09T16:34:07.9143013Z Downloading task: Bash (3.163.1)
2020-03-09T16:34:09.5731897Z Checking job knob settings.
2020-03-09T16:34:09.5750848Z Finished checking job knob settings.
2020-03-09T16:34:09.6210732Z ##[section]Finishing: Initialize job
2020-03-09T16:34:09.6503874Z ##[section]Starting: Configure Job Name
2020-03-09T16:34:09.6682250Z ==============================================================================
2020-03-09T16:34:09.6682973Z Task         : Bash
---
2020-03-09T16:34:10.2187232Z ========================== Starting Command Output ===========================
2020-03-09T16:34:10.2191650Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2c24dc4a-8c1f-43c5-b288-e2aa294b0f11.sh
2020-03-09T16:34:10.2191865Z 
2020-03-09T16:34:10.2194698Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T16:34:10.2213392Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-09T16:34:10.2217529Z Task         : Get sources
2020-03-09T16:34:10.2218099Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T16:34:10.2218501Z Version      : 1.0.0
2020-03-09T16:34:10.2218855Z Author       : Microsoft
---
2020-03-09T16:34:11.5614383Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T16:34:11.5620549Z ##[command]git config gc.auto 0
2020-03-09T16:34:11.5623260Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T16:34:11.5625670Z ##[command]git config --get-all http.proxy
2020-03-09T16:34:11.5630971Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69824/merge:refs/remotes/pull/69824/merge
---
2020-03-09T16:38:28.2921630Z     Checking backtrace v0.3.44
2020-03-09T16:38:28.9113300Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-09T16:38:28.9113864Z   --> src/liballoc/raw_vec.rs:69:32
2020-03-09T16:38:28.9114229Z    |
2020-03-09T16:38:28.9114770Z 69 |             AllocInit::Zero => a.alloc_zeroed(layout),
2020-03-09T16:38:28.9115471Z    |                                ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-09T16:38:28.9116656Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-09T16:38:28.9120709Z 
2020-03-09T16:38:28.9125607Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-09T16:38:28.9126146Z   --> src/liballoc/raw_vec.rs:68:39
2020-03-09T16:38:28.9126146Z   --> src/liballoc/raw_vec.rs:68:39
2020-03-09T16:38:28.9126501Z    |
2020-03-09T16:38:28.9127023Z 68 |             AllocInit::Unspecified => a.alloc(layout),
2020-03-09T16:38:28.9127907Z    |                                       ^^^^^^^^^^^^^^^ call to unsafe function
2020-03-09T16:38:28.9128943Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-09T16:38:28.9132555Z 
2020-03-09T16:38:28.9138712Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-09T16:38:28.9139300Z    --> src/liballoc/raw_vec.rs:558:74
2020-03-09T16:38:28.9139300Z    --> src/liballoc/raw_vec.rs:558:74
2020-03-09T16:38:28.9139663Z     |
2020-03-09T16:38:28.9140280Z 558 |                 (AllocPlacement::Unspecified, AllocInit::Unspecified) => self.a.alloc(new_layout),
2020-03-09T16:38:28.9141151Z     |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-09T16:38:28.9142252Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-09T16:38:28.9146919Z 
2020-03-09T16:38:28.9147575Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-09T16:38:28.9148103Z    --> src/liballoc/raw_vec.rs:559:67
2020-03-09T16:38:28.9148103Z    --> src/liballoc/raw_vec.rs:559:67
2020-03-09T16:38:28.9148463Z     |
2020-03-09T16:38:28.9149077Z 559 |                 (AllocPlacement::Unspecified, AllocInit::Zero) => self.a.alloc_zeroed(new_layout),
2020-03-09T16:38:28.9149952Z     |                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-09T16:38:28.9151048Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-09T16:38:28.9151339Z 
2020-03-09T16:38:28.9185716Z error: aborting due to 4 previous errors
2020-03-09T16:38:28.9185921Z 
---
2020-03-09T16:38:28.9338356Z   local time: Mon Mar  9 16:38:28 UTC 2020
2020-03-09T16:38:29.1995605Z   network time: Mon, 09 Mar 2020 16:38:29 GMT
2020-03-09T16:38:29.1998932Z == end clock drift check ==
2020-03-09T16:38:30.1459317Z 
2020-03-09T16:38:30.1523586Z ##[error]Bash exited with code '1'.
2020-03-09T16:38:30.1534966Z ##[section]Finishing: Run build
2020-03-09T16:38:30.1571653Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-09T16:38:30.1575673Z Task         : Get sources
2020-03-09T16:38:30.1575946Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T16:38:30.1576211Z Version      : 1.0.0
2020-03-09T16:38:30.1576386Z Author       : Microsoft
2020-03-09T16:38:30.1576386Z Author       : Microsoft
2020-03-09T16:38:30.1577216Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-09T16:38:30.1577580Z ==============================================================================
2020-03-09T16:38:30.4227236Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-09T16:38:30.4267279Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-09T16:38:30.4345598Z Cleaning up task key
2020-03-09T16:38:30.4346693Z Start cleaning up orphan processes.
2020-03-09T16:38:30.4480149Z Terminate orphan process: pid (4265) (python)
2020-03-09T16:38:30.4590213Z ##[section]Finishing: Finalize Job
