plain
2020-03-16T16:46:32.9564341Z ========================== Starting Command Output ===========================
2020-03-16T16:46:32.9566323Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f8b35c0d-2d2a-4f39-abaf-814c605dcde7.sh
2020-03-16T16:46:32.9566515Z 
2020-03-16T16:46:32.9569290Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T16:46:32.9584233Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70046/merge to s
2020-03-16T16:46:32.9586744Z Task         : Get sources
2020-03-16T16:46:32.9586960Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T16:46:32.9587208Z Version      : 1.0.0
2020-03-16T16:46:32.9587349Z Author       : Microsoft
---
2020-03-16T16:46:34.2449250Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T16:46:34.2455644Z ##[command]git config gc.auto 0
2020-03-16T16:46:34.2466840Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T16:46:34.2475089Z ##[command]git config --get-all http.proxy
2020-03-16T16:46:34.2487343Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70046/merge:refs/remotes/pull/70046/merge
---
2020-03-16T16:50:56.4014062Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-16T16:50:59.0289413Z error[E0614]: type `u8` cannot be dereferenced
2020-03-16T16:50:59.0290174Z    --> src/libstd/sys_common/wtf8.rs:603:71
2020-03-16T16:50:59.0291044Z     |
2020-03-16T16:50:59.0291813Z 603 |             [.., 0xED, b2 @ 0xA0..=0xAF, b3] => Some(decode_surrogate(*b2, *b3)),
2020-03-16T16:50:59.0293291Z 
2020-03-16T16:50:59.0293752Z error[E0614]: type `u8` cannot be dereferenced
2020-03-16T16:50:59.0294260Z    --> src/libstd/sys_common/wtf8.rs:603:76
2020-03-16T16:50:59.0294675Z     |
2020-03-16T16:50:59.0294675Z     |
2020-03-16T16:50:59.0295391Z 603 |             [.., 0xED, b2 @ 0xA0..=0xAF, b3] => Some(decode_surrogate(*b2, *b3)),
2020-03-16T16:50:59.0296631Z 
2020-03-16T16:50:59.0305447Z error[E0614]: type `u8` cannot be dereferenced
2020-03-16T16:50:59.0306062Z    --> src/libstd/sys_common/wtf8.rs:611:71
2020-03-16T16:50:59.0306469Z     |
2020-03-16T16:50:59.0306469Z     |
2020-03-16T16:50:59.0314629Z 611 |             [0xED, b2 @ 0xB0..=0xBF, b3, ..] => Some(decode_surrogate(*b2, *b3)),
2020-03-16T16:50:59.0315684Z 
2020-03-16T16:50:59.0316079Z error[E0614]: type `u8` cannot be dereferenced
2020-03-16T16:50:59.0316590Z    --> src/libstd/sys_common/wtf8.rs:611:76
2020-03-16T16:50:59.0317019Z     |
2020-03-16T16:50:59.0317019Z     |
2020-03-16T16:50:59.0317590Z 611 |             [0xED, b2 @ 0xB0..=0xBF, b3, ..] => Some(decode_surrogate(*b2, *b3)),
2020-03-16T16:50:59.0318604Z 
2020-03-16T16:50:59.8044800Z error: aborting due to 4 previous errors
2020-03-16T16:50:59.8045795Z 
2020-03-16T16:50:59.8046547Z For more information about this error, try `rustc --explain E0614`.
---
2020-03-16T16:50:59.8051893Z   local time: Mon Mar 16 16:50:59 UTC 2020
2020-03-16T16:50:59.8052165Z   network time: Mon, 16 Mar 2020 16:50:59 GMT
2020-03-16T16:50:59.8052381Z == end clock drift check ==
2020-03-16T16:51:00.7745993Z 
2020-03-16T16:51:00.7819399Z ##[error]Bash exited with code '1'.
2020-03-16T16:51:00.7851120Z ##[section]Finishing: Run build
2020-03-16T16:51:00.7890565Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70046/merge to s
2020-03-16T16:51:00.7894411Z Task         : Get sources
2020-03-16T16:51:00.7894812Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T16:51:00.7895068Z Version      : 1.0.0
2020-03-16T16:51:00.7895235Z Author       : Microsoft
2020-03-16T16:51:00.7895235Z Author       : Microsoft
2020-03-16T16:51:00.7895496Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-16T16:51:00.7895817Z ==============================================================================
2020-03-16T16:51:01.0938404Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-16T16:51:01.0986799Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70046/merge to s
2020-03-16T16:51:01.1062505Z Cleaning up task key
2020-03-16T16:51:01.1063697Z Start cleaning up orphan processes.
2020-03-16T16:51:01.1242381Z Terminate orphan process: pid (10921) (python)
2020-03-16T16:51:01.1411253Z ##[section]Finishing: Finalize Job
