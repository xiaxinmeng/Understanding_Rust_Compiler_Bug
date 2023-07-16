plain
2020-01-25T08:59:47.5238058Z ========================== Starting Command Output ===========================
2020-01-25T08:59:47.5240903Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ca4e60ee-8a03-4928-99ca-d8efdf4d67d0.sh
2020-01-25T08:59:47.5993712Z 
2020-01-25T08:59:47.6069594Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T08:59:47.6076080Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T08:59:47.6077792Z Task         : Get sources
2020-01-25T08:59:47.6077830Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T08:59:47.6077912Z Version      : 1.0.0
2020-01-25T08:59:47.6077951Z Author       : Microsoft
---
2020-01-25T08:59:51.8107604Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T08:59:51.8443218Z ##[command]git config gc.auto 0
2020-01-25T08:59:51.8501033Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T08:59:51.8561125Z ##[command]git config --get-all http.proxy
2020-01-25T08:59:51.8710913Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67330/merge:refs/remotes/pull/67330/merge
---
2020-01-25T09:04:42.5911456Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T09:04:42.5931040Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T09:04:42.8566674Z    Compiling cc v1.0.50
2020-01-25T09:04:42.8578537Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-25T09:04:45.2262259Z error[E0425]: cannot find value `idx_opt` in this scope
2020-01-25T09:04:45.2262907Z      |
2020-01-25T09:04:45.2262907Z      |
2020-01-25T09:04:45.2263250Z 3788 |         let idx = idx_opt.map(|idx| idx + 1).unwrap_or(self.v.len());
2020-01-25T09:04:45.2263700Z 
2020-01-25T09:04:45.2263700Z 
2020-01-25T09:04:45.2378611Z error[E0425]: cannot find value `idx_opt` in this scope
2020-01-25T09:04:45.2379228Z      |
2020-01-25T09:04:45.2379228Z      |
2020-01-25T09:04:45.2379564Z 3813 |         let idx = idx_opt.map(|idx| idx + 1).unwrap_or(self.v.len());
2020-01-25T09:04:45.2379970Z 
2020-01-25T09:04:50.5543880Z    Compiling libc v0.2.66
2020-01-25T09:04:51.4935351Z    Compiling autocfg v0.1.7
2020-01-25T09:04:52.3254944Z error: aborting due to 2 previous errors
---
2020-01-25T09:04:52.8748551Z   local time: Sat Jan 25 09:04:52 UTC 2020
2020-01-25T09:04:53.1596624Z   network time: Sat, 25 Jan 2020 09:04:53 GMT
2020-01-25T09:04:53.1598978Z == end clock drift check ==
2020-01-25T09:05:00.9707985Z 
2020-01-25T09:05:00.9779202Z ##[error]Bash exited with code '1'.
2020-01-25T09:05:00.9792703Z ##[section]Finishing: Run build
2020-01-25T09:05:00.9808586Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T09:05:00.9810457Z Task         : Get sources
2020-01-25T09:05:00.9810513Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T09:05:00.9810583Z Version      : 1.0.0
2020-01-25T09:05:00.9810633Z Author       : Microsoft
2020-01-25T09:05:00.9810633Z Author       : Microsoft
2020-01-25T09:05:00.9810805Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T09:05:00.9810877Z ==============================================================================
2020-01-25T09:05:01.4140718Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T09:05:01.4184244Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T09:05:01.4329556Z Cleaning up task key
2020-01-25T09:05:01.4330542Z Start cleaning up orphan processes.
2020-01-25T09:05:01.4440047Z Terminate orphan process: pid (4385) (python)
2020-01-25T09:05:01.5164415Z ##[section]Finishing: Finalize Job
