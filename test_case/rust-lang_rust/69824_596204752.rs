plain
2020-03-08T13:08:29.4857675Z ========================== Starting Command Output ===========================
2020-03-08T13:08:29.4862455Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d19da6c9-bde8-444c-ab75-ad1c6789074f.sh
2020-03-08T13:08:29.4862907Z 
2020-03-08T13:08:29.4866786Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-08T13:08:29.4885525Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T13:08:29.4889062Z Task         : Get sources
2020-03-08T13:08:29.4889356Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T13:08:29.4889639Z Version      : 1.0.0
2020-03-08T13:08:29.4889850Z Author       : Microsoft
---
2020-03-08T13:08:30.4827732Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-08T13:08:30.4833242Z ##[command]git config gc.auto 0
2020-03-08T13:08:30.4836479Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-08T13:08:30.4840856Z ##[command]git config --get-all http.proxy
2020-03-08T13:08:30.4847088Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69824/merge:refs/remotes/pull/69824/merge
---
2020-03-08T13:13:13.4513849Z     Checking backtrace v0.3.44
2020-03-08T13:13:14.5454465Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T13:13:14.5456214Z   --> src/liballoc/raw_vec.rs:77:70
2020-03-08T13:13:14.5457187Z    |
2020-03-08T13:13:14.5458192Z 77 |         let allocation = if zeroed { a.alloc_zeroed(layout) } else { a.alloc(layout) };
2020-03-08T13:13:14.5459603Z    |                                                                      ^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T13:13:14.5464081Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T13:13:14.5464498Z 
2020-03-08T13:13:14.5465088Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T13:13:14.5465786Z   --> src/liballoc/raw_vec.rs:77:38
2020-03-08T13:13:14.5465786Z   --> src/liballoc/raw_vec.rs:77:38
2020-03-08T13:13:14.5466669Z    |
2020-03-08T13:13:14.5467476Z 77 |         let allocation = if zeroed { a.alloc_zeroed(layout) } else { a.alloc(layout) };
2020-03-08T13:13:14.5468559Z    |                                      ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T13:13:14.5469847Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T13:13:14.5470226Z 
2020-03-08T13:13:14.5470850Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T13:13:14.5471446Z    --> src/liballoc/raw_vec.rs:541:38
2020-03-08T13:13:14.5471446Z    --> src/liballoc/raw_vec.rs:541:38
2020-03-08T13:13:14.5471884Z     |
2020-03-08T13:13:14.5472459Z 541 |                 (Allocate, false) => self.a.alloc(new_layout),
2020-03-08T13:13:14.5473290Z     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T13:13:14.5474849Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T13:13:14.5475238Z 
2020-03-08T13:13:14.5475833Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T13:13:14.5476515Z    --> src/liballoc/raw_vec.rs:542:37
2020-03-08T13:13:14.5476515Z    --> src/liballoc/raw_vec.rs:542:37
2020-03-08T13:13:14.5476994Z     |
2020-03-08T13:13:14.5477739Z 542 |                 (Allocate, true) => self.a.alloc_zeroed(new_layout),
2020-03-08T13:13:14.5479232Z     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T13:13:14.5480576Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T13:13:14.5480941Z 
2020-03-08T13:13:14.5539497Z error: aborting due to 4 previous errors
2020-03-08T13:13:14.5539739Z 
---
2020-03-08T13:13:14.5749791Z   local time: Sun Mar  8 13:13:14 UTC 2020
2020-03-08T13:13:14.8556782Z   network time: Sun, 08 Mar 2020 13:13:14 GMT
2020-03-08T13:13:14.8562343Z == end clock drift check ==
2020-03-08T13:13:15.8311664Z 
2020-03-08T13:13:15.8381891Z ##[error]Bash exited with code '1'.
2020-03-08T13:13:15.8395593Z ##[section]Finishing: Run build
2020-03-08T13:13:15.8439490Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T13:13:15.8444495Z Task         : Get sources
2020-03-08T13:13:15.8444815Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T13:13:15.8445093Z Version      : 1.0.0
2020-03-08T13:13:15.8445285Z Author       : Microsoft
2020-03-08T13:13:15.8445285Z Author       : Microsoft
2020-03-08T13:13:15.8445611Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-08T13:13:15.8445969Z ==============================================================================
2020-03-08T13:13:16.1673808Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-08T13:13:16.1725233Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T13:13:16.1813748Z Cleaning up task key
2020-03-08T13:13:16.1815161Z Start cleaning up orphan processes.
2020-03-08T13:13:16.2152565Z Terminate orphan process: pid (4252) (python)
2020-03-08T13:13:16.2198514Z ##[section]Finishing: Finalize Job
