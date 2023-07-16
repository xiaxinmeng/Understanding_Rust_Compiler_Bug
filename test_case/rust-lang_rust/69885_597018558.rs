plain
2020-03-10T10:34:03.5884011Z ========================== Starting Command Output ===========================
2020-03-10T10:34:03.5888059Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1011d629-6c69-4d7b-8a98-10ded316184f.sh
2020-03-10T10:34:03.5888571Z 
2020-03-10T10:34:03.5893964Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T10:34:03.5913617Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69885/merge to s
2020-03-10T10:34:03.5917858Z Task         : Get sources
2020-03-10T10:34:03.5918136Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T10:34:03.5918382Z Version      : 1.0.0
2020-03-10T10:34:03.5918551Z Author       : Microsoft
---
2020-03-10T10:34:04.5708481Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T10:34:04.5715325Z ##[command]git config gc.auto 0
2020-03-10T10:34:04.5719223Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T10:34:04.5722628Z ##[command]git config --get-all http.proxy
2020-03-10T10:34:04.5728858Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69885/merge:refs/remotes/pull/69885/merge
---
2020-03-10T10:40:10.9005603Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T10:40:10.9016951Z 
2020-03-10T10:40:10.9017270Z spurious failure, trying again
2020-03-10T10:40:11.0253951Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T10:40:11.0272738Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpDZcT7P.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T10:40:11.0323686Z == clock drift check ==
2020-03-10T10:40:11.0342993Z   local time: Tue Mar 10 10:40:11 UTC 2020
2020-03-10T10:40:11.0979197Z   network time: Tue, 10 Mar 2020 10:40:11 GMT
2020-03-10T10:40:11.0979768Z == end clock drift check ==
2020-03-10T10:40:11.0979768Z == end clock drift check ==
2020-03-10T10:40:18.3189460Z 
2020-03-10T10:40:18.3278368Z ##[error]Bash exited with code '1'.
2020-03-10T10:40:18.3328667Z ##[section]Finishing: Run build
2020-03-10T10:40:18.3391813Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69885/merge to s
2020-03-10T10:40:18.3397424Z Task         : Get sources
2020-03-10T10:40:18.3397815Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T10:40:18.3398175Z Version      : 1.0.0
2020-03-10T10:40:18.3398442Z Author       : Microsoft
2020-03-10T10:40:18.3398442Z Author       : Microsoft
2020-03-10T10:40:18.3398835Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T10:40:18.3399299Z ==============================================================================
2020-03-10T10:40:18.7249829Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T10:40:18.7297240Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69885/merge to s
2020-03-10T10:40:18.7396247Z Cleaning up task key
2020-03-10T10:40:18.7397854Z Start cleaning up orphan processes.
2020-03-10T10:40:18.7605839Z Terminate orphan process: pid (4320) (python)
2020-03-10T10:40:18.7766696Z ##[section]Finishing: Finalize Job
