plain
2020-03-08T14:11:08.2515507Z ========================== Starting Command Output ===========================
2020-03-08T14:11:08.2517946Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/eb0e3b30-35a4-4417-bfbc-295cddfd89f8.sh
2020-03-08T14:11:08.2518207Z 
2020-03-08T14:11:08.2521503Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-08T14:11:08.2540369Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T14:11:08.2543493Z Task         : Get sources
2020-03-08T14:11:08.2543799Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T14:11:08.2544072Z Version      : 1.0.0
2020-03-08T14:11:08.2544258Z Author       : Microsoft
---
2020-03-08T14:11:09.2531286Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-08T14:11:09.2537028Z ##[command]git config gc.auto 0
2020-03-08T14:11:09.2541585Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-08T14:11:09.2545404Z ##[command]git config --get-all http.proxy
2020-03-08T14:11:09.2552145Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69824/merge:refs/remotes/pull/69824/merge
---
2020-03-08T14:17:11.9782414Z     Checking backtrace v0.3.44
2020-03-08T14:17:12.5020051Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T14:17:12.5020945Z   --> src/liballoc/raw_vec.rs:67:70
2020-03-08T14:17:12.5021506Z    |
2020-03-08T14:17:12.5022319Z 67 |         let allocation = if zeroed { a.alloc_zeroed(layout) } else { a.alloc(layout) };
2020-03-08T14:17:12.5023518Z    |                                                                      ^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T14:17:12.5025167Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T14:17:12.5025614Z 
2020-03-08T14:17:12.5026272Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T14:17:12.5027068Z   --> src/liballoc/raw_vec.rs:67:38
2020-03-08T14:17:12.5027068Z   --> src/liballoc/raw_vec.rs:67:38
2020-03-08T14:17:12.5027605Z    |
2020-03-08T14:17:12.5028405Z 67 |         let allocation = if zeroed { a.alloc_zeroed(layout) } else { a.alloc(layout) };
2020-03-08T14:17:12.5029534Z    |                                      ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T14:17:12.5031102Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T14:17:12.5031540Z 
2020-03-08T14:17:12.5032194Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T14:17:12.5032992Z    --> src/liballoc/raw_vec.rs:536:38
2020-03-08T14:17:12.5032992Z    --> src/liballoc/raw_vec.rs:536:38
2020-03-08T14:17:12.5033540Z     |
2020-03-08T14:17:12.5034272Z 536 |                 (Allocate, false) => self.a.alloc(new_layout),
2020-03-08T14:17:12.5035355Z     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T14:17:12.5036930Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T14:17:12.5037390Z 
2020-03-08T14:17:12.5050259Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-03-08T14:17:12.5051123Z    --> src/liballoc/raw_vec.rs:537:37
2020-03-08T14:17:12.5051123Z    --> src/liballoc/raw_vec.rs:537:37
2020-03-08T14:17:12.5051709Z     |
2020-03-08T14:17:12.5052472Z 537 |                 (Allocate, true) => self.a.alloc_zeroed(new_layout),
2020-03-08T14:17:12.5053576Z     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-03-08T14:17:12.5055372Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2020-03-08T14:17:12.5060706Z 
2020-03-08T14:17:12.5107877Z error: aborting due to 4 previous errors
2020-03-08T14:17:12.5113088Z 
---
2020-03-08T14:17:12.5307230Z   local time: Sun Mar  8 14:17:12 UTC 2020
2020-03-08T14:17:12.6210603Z   network time: Sun, 08 Mar 2020 14:17:12 GMT
2020-03-08T14:17:12.6210961Z == end clock drift check ==
2020-03-08T14:17:13.7035326Z 
2020-03-08T14:17:13.7085379Z ##[error]Bash exited with code '1'.
2020-03-08T14:17:13.7108643Z ##[section]Finishing: Run build
2020-03-08T14:17:13.7157655Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T14:17:13.7163343Z Task         : Get sources
2020-03-08T14:17:13.7163696Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T14:17:13.7164015Z Version      : 1.0.0
2020-03-08T14:17:13.7164262Z Author       : Microsoft
2020-03-08T14:17:13.7164262Z Author       : Microsoft
2020-03-08T14:17:13.7164622Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-08T14:17:13.7165032Z ==============================================================================
2020-03-08T14:17:14.0525044Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-08T14:17:14.0569078Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69824/merge to s
2020-03-08T14:17:14.0660221Z Cleaning up task key
2020-03-08T14:17:14.0661479Z Start cleaning up orphan processes.
2020-03-08T14:17:14.0867875Z Terminate orphan process: pid (4838) (python)
2020-03-08T14:17:14.1012227Z ##[section]Finishing: Finalize Job
