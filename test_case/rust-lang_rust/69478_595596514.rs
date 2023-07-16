plain
2020-03-06T04:35:42.7722848Z ========================== Starting Command Output ===========================
2020-03-06T04:35:42.7725166Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cc71bdd4-8cc8-441e-a9e8-10f70e615455.sh
2020-03-06T04:35:42.7725416Z 
2020-03-06T04:35:42.7728961Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-06T04:35:42.7745232Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-06T04:35:42.7747926Z Task         : Get sources
2020-03-06T04:35:42.7748202Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T04:35:42.7748449Z Version      : 1.0.0
2020-03-06T04:35:42.7748616Z Author       : Microsoft
---
2020-03-06T04:35:43.7761557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-06T04:35:43.7767647Z ##[command]git config gc.auto 0
2020-03-06T04:35:43.7771418Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-06T04:35:43.7775031Z ##[command]git config --get-all http.proxy
2020-03-06T04:35:43.7781590Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
---
2020-03-06T04:38:45.4572943Z   local time: Fri Mar  6 04:38:45 UTC 2020
2020-03-06T04:38:45.6215401Z   network time: Fri, 06 Mar 2020 04:38:45 GMT
2020-03-06T04:38:45.6219260Z == end clock drift check ==
2020-03-06T04:38:46.5907019Z 
2020-03-06T04:38:46.5968950Z ##[error]Bash exited with code '1'.
2020-03-06T04:38:46.5983872Z ##[section]Finishing: Run build
2020-03-06T04:38:46.6019741Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-06T04:38:46.6024103Z Task         : Get sources
2020-03-06T04:38:46.6024363Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T04:38:46.6024586Z Version      : 1.0.0
2020-03-06T04:38:46.6024745Z Author       : Microsoft
2020-03-06T04:38:46.6024745Z Author       : Microsoft
2020-03-06T04:38:46.6025015Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-06T04:38:46.6025301Z ==============================================================================
2020-03-06T04:38:46.8438171Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-06T04:38:46.8477284Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-06T04:38:46.8539937Z Cleaning up task key
2020-03-06T04:38:46.8540878Z Start cleaning up orphan processes.
2020-03-06T04:38:46.8665961Z Terminate orphan process: pid (3683) (python)
2020-03-06T04:38:46.8771084Z ##[section]Finishing: Finalize Job
