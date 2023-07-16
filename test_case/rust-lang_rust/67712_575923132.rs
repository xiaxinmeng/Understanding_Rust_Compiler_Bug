plain
2020-01-18T17:18:56.2376232Z ========================== Starting Command Output ===========================
2020-01-18T17:18:56.2422987Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/71bf218b-b85b-4c1d-8796-0217c2141007.sh
2020-01-18T17:18:56.2423027Z 
2020-01-18T17:18:56.2426740Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T17:18:56.2434197Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67712/merge to s
2020-01-18T17:18:56.2436092Z Task         : Get sources
2020-01-18T17:18:56.2436120Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T17:18:56.2436148Z Version      : 1.0.0
2020-01-18T17:18:56.2436181Z Author       : Microsoft
---
2020-01-18T17:18:56.9987585Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T17:18:57.0095717Z ##[command]git config gc.auto 0
2020-01-18T17:18:57.0163732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T17:18:57.0222650Z ##[command]git config --get-all http.proxy
2020-01-18T17:18:57.0339746Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67712/merge:refs/remotes/pull/67712/merge
---
2020-01-18T17:55:13.1472901Z   local time: Sat Jan 18 17:55:13 UTC 2020
2020-01-18T17:55:13.7005401Z   network time: Sat, 18 Jan 2020 17:55:13 GMT
2020-01-18T17:55:13.7005841Z == end clock drift check ==
2020-01-18T17:55:15.1998610Z 
2020-01-18T17:55:15.2087196Z ##[error]Bash exited with code '1'.
2020-01-18T17:55:15.2097409Z ##[section]Finishing: Run build
2020-01-18T17:55:15.2121237Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67712/merge to s
2020-01-18T17:55:15.2123001Z Task         : Get sources
2020-01-18T17:55:15.2123045Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T17:55:15.2123105Z Version      : 1.0.0
2020-01-18T17:55:15.2123145Z Author       : Microsoft
2020-01-18T17:55:15.2123145Z Author       : Microsoft
2020-01-18T17:55:15.2123187Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T17:55:15.2123252Z ==============================================================================
2020-01-18T17:55:15.6551194Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T17:55:15.6600428Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67712/merge to s
2020-01-18T17:55:15.6707886Z Cleaning up task key
2020-01-18T17:55:15.6708914Z Start cleaning up orphan processes.
2020-01-18T17:55:15.6818452Z Terminate orphan process: pid (3438) (python)
2020-01-18T17:55:15.7144749Z ##[section]Finishing: Finalize Job
