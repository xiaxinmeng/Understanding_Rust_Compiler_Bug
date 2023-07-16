plain
2020-01-28T08:55:38.7909426Z ========================== Starting Command Output ===========================
2020-01-28T08:55:38.7912803Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c4510b0d-f0a2-4729-ad5b-66972f9898b9.sh
2020-01-28T08:55:38.7912955Z 
2020-01-28T08:55:38.7917267Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T08:55:38.7929321Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68594/merge to s
2020-01-28T08:55:38.7930979Z Task         : Get sources
2020-01-28T08:55:38.7931016Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T08:55:38.7931051Z Version      : 1.0.0
2020-01-28T08:55:38.7931132Z Author       : Microsoft
---
2020-01-28T08:55:43.1237123Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T08:55:43.1255448Z ##[command]git config gc.auto 0
2020-01-28T08:55:43.1260748Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T08:55:43.1264374Z ##[command]git config --get-all http.proxy
2020-01-28T08:55:43.1273392Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68594/merge:refs/remotes/pull/68594/merge
---
2020-01-28T09:00:46.8462529Z curl: (22) The requested URL returned error: 404 Not Found
2020-01-28T09:00:46.8477180Z 
2020-01-28T09:00:46.8477790Z spurious failure, trying again
2020-01-28T09:00:46.9924025Z curl: (22) The requested URL returned error: 404 Not Found
2020-01-28T09:00:46.9952774Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpRipBqv.sha256 https://dev-static.rust-lang.org/dist/2019-12-18/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-01-28T09:00:47.0008991Z == clock drift check ==
2020-01-28T09:00:47.0020015Z   local time: Tue Jan 28 09:00:47 UTC 2020
2020-01-28T09:00:47.0924315Z   network time: Tue, 28 Jan 2020 09:00:47 GMT
2020-01-28T09:00:47.0928208Z == end clock drift check ==
2020-01-28T09:00:47.0928208Z == end clock drift check ==
2020-01-28T09:00:54.4686277Z 
2020-01-28T09:00:54.4769804Z ##[error]Bash exited with code '1'.
2020-01-28T09:00:54.4783193Z ##[section]Finishing: Run build
2020-01-28T09:00:54.4837864Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68594/merge to s
2020-01-28T09:00:54.4839792Z Task         : Get sources
2020-01-28T09:00:54.4839844Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T09:00:54.4839939Z Version      : 1.0.0
2020-01-28T09:00:54.4839990Z Author       : Microsoft
2020-01-28T09:00:54.4839990Z Author       : Microsoft
2020-01-28T09:00:54.4840041Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-28T09:00:54.4840141Z ==============================================================================
2020-01-28T09:00:54.9005566Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-28T09:00:54.9050657Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68594/merge to s
2020-01-28T09:00:54.9165973Z Cleaning up task key
2020-01-28T09:00:54.9166864Z Start cleaning up orphan processes.
2020-01-28T09:00:54.9278392Z Terminate orphan process: pid (3809) (python)
2020-01-28T09:00:54.9639349Z ##[section]Finishing: Finalize Job
