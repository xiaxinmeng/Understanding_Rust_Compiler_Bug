plain
2020-01-18T00:04:04.7005596Z ========================== Starting Command Output ===========================
2020-01-18T00:04:04.7008183Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2d063e57-e297-49a4-a9f1-ac5288a4fd93.sh
2020-01-18T00:04:04.7008226Z 
2020-01-18T00:04:04.7011182Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T00:04:04.7016961Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68267/merge to s
2020-01-18T00:04:04.7019132Z Task         : Get sources
2020-01-18T00:04:04.7019166Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T00:04:04.7019246Z Version      : 1.0.0
2020-01-18T00:04:04.7019280Z Author       : Microsoft
---
2020-01-18T00:04:05.5022695Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T00:04:05.5114932Z ##[command]git config gc.auto 0
2020-01-18T00:04:05.5169846Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T00:04:05.5223436Z ##[command]git config --get-all http.proxy
2020-01-18T00:04:05.5363202Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68267/merge:refs/remotes/pull/68267/merge
---
2020-01-18T00:48:05.7769778Z    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-01-18T00:48:06.1658028Z error[E0658]: subslice patterns are unstable
2020-01-18T00:48:06.1658376Z     --> src/librustc_resolve/lifetimes.rs:1837:33
2020-01-18T00:48:06.1658919Z      |
2020-01-18T00:48:06.1659285Z 1837 |                         [param, ..] => (param.span.shrink_to_lo(), format!("{}, ", lifetime_ref)),
2020-01-18T00:48:06.1659843Z      |
2020-01-18T00:48:06.1659843Z      |
2020-01-18T00:48:06.1660278Z      = note: for more information, see ***/issues/62254
2020-01-18T00:48:06.1664456Z 
2020-01-18T00:48:06.1703482Z error[E0658]: subslice patterns are unstable
2020-01-18T00:48:06.1703759Z     --> src/librustc_resolve/lifetimes.rs:2932:29
2020-01-18T00:48:06.1703934Z      |
2020-01-18T00:48:06.1703934Z      |
2020-01-18T00:48:06.1704218Z 2932 |                     [param, ..] => (param.span.shrink_to_lo(), "'lifetime, ".to_string()),
2020-01-18T00:48:06.1704693Z      |
2020-01-18T00:48:06.1704693Z      |
2020-01-18T00:48:06.1705027Z      = note: for more information, see ***/issues/62254
2020-01-18T00:48:06.1709621Z 
2020-01-18T00:48:07.8447134Z error: aborting due to 2 previous errors
2020-01-18T00:48:07.8452140Z 
2020-01-18T00:48:07.8462990Z For more information about this error, try `rustc --explain E0658`.
---
2020-01-18T00:48:27.7141641Z   local time: Sat Jan 18 00:48:27 UTC 2020
2020-01-18T00:48:27.9860737Z   network time: Sat, 18 Jan 2020 00:48:27 GMT
2020-01-18T00:48:27.9861201Z == end clock drift check ==
2020-01-18T00:48:28.5848474Z 
2020-01-18T00:48:28.5953545Z ##[error]Bash exited with code '1'.
2020-01-18T00:48:28.5964922Z ##[section]Finishing: Run build
2020-01-18T00:48:28.5982349Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68267/merge to s
2020-01-18T00:48:28.5984304Z Task         : Get sources
2020-01-18T00:48:28.5984340Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T00:48:28.5984374Z Version      : 1.0.0
2020-01-18T00:48:28.5984422Z Author       : Microsoft
2020-01-18T00:48:28.5984422Z Author       : Microsoft
2020-01-18T00:48:28.5984458Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T00:48:28.5984495Z ==============================================================================
2020-01-18T00:48:29.0054820Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T00:48:29.0093648Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68267/merge to s
2020-01-18T00:48:29.0217246Z Cleaning up task key
2020-01-18T00:48:29.0217908Z Start cleaning up orphan processes.
2020-01-18T00:48:29.0317708Z Terminate orphan process: pid (3644) (python)
2020-01-18T00:48:29.0606140Z ##[section]Finishing: Finalize Job
