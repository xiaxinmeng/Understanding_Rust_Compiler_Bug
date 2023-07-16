plain
2020-03-02T23:27:06.6318163Z ========================== Starting Command Output ===========================
2020-03-02T23:27:06.6320547Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/98769562-fa75-4057-bce4-2b4c4751943a.sh
2020-03-02T23:27:06.6320737Z 
2020-03-02T23:27:06.6323510Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-02T23:27:06.6340529Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69649/merge to s
2020-03-02T23:27:06.6343358Z Task         : Get sources
2020-03-02T23:27:06.6343630Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T23:27:06.6343849Z Version      : 1.0.0
2020-03-02T23:27:06.6343998Z Author       : Microsoft
---
2020-03-02T23:27:07.9675834Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-02T23:27:07.9737306Z ##[command]git config gc.auto 0
2020-03-02T23:27:07.9740680Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-02T23:27:07.9743601Z ##[command]git config --get-all http.proxy
2020-03-02T23:27:07.9750363Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69649/merge:refs/remotes/pull/69649/merge
---
2020-03-02T23:35:28.6115543Z   local time: Mon Mar  2 23:35:28 UTC 2020
2020-03-02T23:35:28.8735544Z   network time: Mon, 02 Mar 2020 23:35:28 GMT
2020-03-02T23:35:28.8740322Z == end clock drift check ==
2020-03-02T23:35:29.3144523Z 
2020-03-02T23:35:29.3206632Z ##[error]Bash exited with code '1'.
2020-03-02T23:35:29.3229813Z ##[section]Finishing: Run build
2020-03-02T23:35:29.3269774Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69649/merge to s
2020-03-02T23:35:29.3275086Z Task         : Get sources
2020-03-02T23:35:29.3275392Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T23:35:29.3275653Z Version      : 1.0.0
2020-03-02T23:35:29.3275830Z Author       : Microsoft
2020-03-02T23:35:29.3275830Z Author       : Microsoft
2020-03-02T23:35:29.3276134Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-02T23:35:29.3276473Z ==============================================================================
2020-03-02T23:35:29.5944520Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-02T23:35:29.5979652Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69649/merge to s
2020-03-02T23:35:29.6051522Z Cleaning up task key
2020-03-02T23:35:29.6052951Z Start cleaning up orphan processes.
2020-03-02T23:35:29.6228080Z Terminate orphan process: pid (7732) (python)
2020-03-02T23:35:29.6352622Z ##[section]Finishing: Finalize Job
