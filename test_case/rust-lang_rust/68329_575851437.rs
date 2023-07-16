plain
2020-01-18T00:35:34.1298453Z ========================== Starting Command Output ===========================
2020-01-18T00:35:34.1301303Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/edb7ce94-8e7b-4011-a45e-68c89a6a219a.sh
2020-01-18T00:35:34.1301504Z 
2020-01-18T00:35:34.1306667Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T00:35:34.1312510Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68329/merge to s
2020-01-18T00:35:34.1314261Z Task         : Get sources
2020-01-18T00:35:34.1314295Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T00:35:34.1314344Z Version      : 1.0.0
2020-01-18T00:35:34.1314377Z Author       : Microsoft
---
2020-01-18T00:35:35.2092662Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T00:35:35.2107176Z ##[command]git config gc.auto 0
2020-01-18T00:35:35.2111819Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T00:35:35.2117997Z ##[command]git config --get-all http.proxy
2020-01-18T00:35:35.2128537Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68329/merge:refs/remotes/pull/68329/merge
---
2020-01-18T01:17:13.6270691Z   local time: Sat Jan 18 01:17:13 UTC 2020
2020-01-18T01:17:13.7328320Z   network time: Sat, 18 Jan 2020 01:17:13 GMT
2020-01-18T01:17:13.7329928Z == end clock drift check ==
2020-01-18T01:17:15.2428712Z 
2020-01-18T01:17:15.2528051Z ##[error]Bash exited with code '1'.
2020-01-18T01:17:15.2540294Z ##[section]Finishing: Run build
2020-01-18T01:17:15.2562824Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68329/merge to s
2020-01-18T01:17:15.2564733Z Task         : Get sources
2020-01-18T01:17:15.2564782Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T01:17:15.2564864Z Version      : 1.0.0
2020-01-18T01:17:15.2564906Z Author       : Microsoft
2020-01-18T01:17:15.2564906Z Author       : Microsoft
2020-01-18T01:17:15.2564954Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T01:17:15.2565021Z ==============================================================================
2020-01-18T01:17:15.7305008Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T01:17:15.7373916Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68329/merge to s
2020-01-18T01:17:15.7519115Z Cleaning up task key
2020-01-18T01:17:15.7520049Z Start cleaning up orphan processes.
2020-01-18T01:17:15.7634210Z Terminate orphan process: pid (3594) (python)
2020-01-18T01:17:15.7957523Z ##[section]Finishing: Finalize Job
