plain
2020-03-09T08:30:14.6346656Z ========================== Starting Command Output ===========================
2020-03-09T08:30:14.6352507Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/50537a77-6f5a-4db2-bcb5-85e842f856e7.sh
2020-03-09T08:30:14.6353029Z 
2020-03-09T08:30:14.6357945Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T08:30:14.6379604Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69167/merge to s
2020-03-09T08:30:14.6383813Z Task         : Get sources
2020-03-09T08:30:14.6384140Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T08:30:14.6384455Z Version      : 1.0.0
2020-03-09T08:30:14.6384668Z Author       : Microsoft
---
2020-03-09T08:30:16.2322874Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T08:30:16.2330361Z ##[command]git config gc.auto 0
2020-03-09T08:30:16.2334766Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T08:30:16.2339024Z ##[command]git config --get-all http.proxy
2020-03-09T08:30:16.2347906Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69167/merge:refs/remotes/pull/69167/merge
---
2020-03-09T08:39:37.6954935Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-03-09T08:39:37.9675252Z error: named argument never used
2020-03-09T08:39:37.9678097Z     --> src/librustdoc/html/render.rs:1629:29
2020-03-09T08:39:37.9679074Z      |
2020-03-09T08:39:37.9680032Z 1627 |                     "{root}/{path}",
2020-03-09T08:39:37.9682504Z 1628 |                     root = source_code_external_url,
2020-03-09T08:39:37.9683546Z 1629 |                     krate = krate,
2020-03-09T08:39:37.9685130Z      |                             ^^^^^ named argument never used
2020-03-09T08:39:37.9685802Z 
---
2020-03-09T08:39:41.0014982Z   local time: Mon Mar  9 08:39:40 UTC 2020
2020-03-09T08:39:41.2861693Z   network time: Mon, 09 Mar 2020 08:39:41 GMT
2020-03-09T08:39:41.2866543Z == end clock drift check ==
2020-03-09T08:39:41.7427270Z 
2020-03-09T08:39:41.7482563Z ##[error]Bash exited with code '1'.
2020-03-09T08:39:41.7497921Z ##[section]Finishing: Run build
2020-03-09T08:39:41.7553708Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69167/merge to s
2020-03-09T08:39:41.7559366Z Task         : Get sources
2020-03-09T08:39:41.7559756Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T08:39:41.7560115Z Version      : 1.0.0
2020-03-09T08:39:41.7560406Z Author       : Microsoft
2020-03-09T08:39:41.7560406Z Author       : Microsoft
2020-03-09T08:39:41.7560804Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-09T08:39:41.7561266Z ==============================================================================
2020-03-09T08:39:42.1192906Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-09T08:39:42.1242140Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69167/merge to s
2020-03-09T08:39:42.1332882Z Cleaning up task key
2020-03-09T08:39:42.1334723Z Start cleaning up orphan processes.
2020-03-09T08:39:42.1539067Z Terminate orphan process: pid (3554) (python)
2020-03-09T08:39:42.1682949Z ##[section]Finishing: Finalize Job
