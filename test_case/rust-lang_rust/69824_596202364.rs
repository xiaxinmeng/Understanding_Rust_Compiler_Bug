plain
2020-03-08T12:45:08.5516399Z ========================== Starting Command Output ===========================
2020-03-08T12:45:08.5525082Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dc8ebb8d-778a-449f-a181-90c3ff3c12bb.sh
2020-03-08T12:45:08.5525800Z 
2020-03-08T12:45:08.5531503Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-08T12:45:08.5553760Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T12:45:08.5557658Z Task         : Get sources
2020-03-08T12:45:08.5558029Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T12:45:08.5558349Z Version      : 1.0.0
2020-03-08T12:45:08.5558567Z Author       : Microsoft
---
2020-03-08T12:45:09.5369734Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-08T12:45:09.5375520Z ##[command]git config gc.auto 0
2020-03-08T12:45:09.5379557Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-08T12:45:09.5383232Z ##[command]git config --get-all http.proxy
2020-03-08T12:45:09.5390168Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69824/merge:refs/remotes/pull/69824/merge
---
2020-03-08T12:49:47.0916046Z     Checking backtrace v0.3.44
2020-03-08T12:49:47.6228635Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T12:49:47.6229506Z   --> src/liballoc/raw_vec.rs:77:70
2020-03-08T12:49:47.6230102Z    |
2020-03-08T12:49:47.6230918Z 77 |         let allocation = if zeroed { a.alloc_zeroed(layout) } else { a.alloc(layout) };
2020-03-08T12:49:47.6232328Z    |                                                                      ^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T12:49:47.6234177Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T12:49:47.6234642Z 
2020-03-08T12:49:47.6235312Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T12:49:47.6236083Z   --> src/liballoc/raw_vec.rs:77:38
2020-03-08T12:49:47.6236083Z   --> src/liballoc/raw_vec.rs:77:38
2020-03-08T12:49:47.6236643Z    |
2020-03-08T12:49:47.6237448Z 77 |         let allocation = if zeroed { a.alloc_zeroed(layout) } else { a.alloc(layout) };
2020-03-08T12:49:47.6238580Z    |                                      ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T12:49:47.6240141Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T12:49:47.6240601Z 
2020-03-08T12:49:47.6241264Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T12:49:47.6242041Z    --> src/liballoc/raw_vec.rs:544:38
2020-03-08T12:49:47.6242041Z    --> src/liballoc/raw_vec.rs:544:38
2020-03-08T12:49:47.6242606Z     |
2020-03-08T12:49:47.6243339Z 544 |                 (Allocate, false) => self.a.alloc(new_layout),
2020-03-08T12:49:47.6244408Z     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T12:49:47.6246689Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T12:49:47.6247156Z 
2020-03-08T12:49:47.6257826Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T12:49:47.6259042Z    --> src/liballoc/raw_vec.rs:545:37
2020-03-08T12:49:47.6259042Z    --> src/liballoc/raw_vec.rs:545:37
2020-03-08T12:49:47.6259890Z     |
2020-03-08T12:49:47.6260861Z 545 |                 (Allocate, true) => self.a.alloc_zeroed(new_layout),
2020-03-08T12:49:47.6262153Z     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T12:49:47.6264188Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T12:49:47.6264787Z 
2020-03-08T12:49:47.6348450Z error: aborting due to 4 previous errors
2020-03-08T12:49:47.6348780Z 
---
2020-03-08T12:49:48.7200676Z   local time: Sun Mar  8 12:49:47 UTC 2020
2020-03-08T12:49:48.7200966Z   network time: Sun, 08 Mar 2020 12:49:47 GMT
2020-03-08T12:49:48.7201214Z == end clock drift check ==
2020-03-08T12:49:48.8863246Z 
2020-03-08T12:49:48.8959697Z ##[error]Bash exited with code '1'.
2020-03-08T12:49:48.8974904Z ##[section]Finishing: Run build
2020-03-08T12:49:48.9025018Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T12:49:48.9030836Z Task         : Get sources
2020-03-08T12:49:48.9031234Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T12:49:48.9031620Z Version      : 1.0.0
2020-03-08T12:49:48.9031874Z Author       : Microsoft
2020-03-08T12:49:48.9031874Z Author       : Microsoft
2020-03-08T12:49:48.9032298Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-08T12:49:48.9032784Z ==============================================================================
2020-03-08T12:49:49.2165934Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-08T12:49:49.2210812Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T12:49:49.2289168Z Cleaning up task key
2020-03-08T12:49:49.2290294Z Start cleaning up orphan processes.
2020-03-08T12:49:49.2452061Z Terminate orphan process: pid (3954) (python)
2020-03-08T12:49:49.2571841Z ##[section]Finishing: Finalize Job
