plain
2020-03-12T19:01:49.4170462Z ========================== Starting Command Output ===========================
2020-03-12T19:01:49.4172936Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cc2ab8e7-f25e-4843-91fa-afc40354c8c7.sh
2020-03-12T19:01:49.4173187Z 
2020-03-12T19:01:49.4176962Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-12T19:01:49.4194955Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69955/merge to s
2020-03-12T19:01:49.4198077Z Task         : Get sources
2020-03-12T19:01:49.4198362Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-12T19:01:49.4198653Z Version      : 1.0.0
2020-03-12T19:01:49.4198842Z Author       : Microsoft
---
2020-03-12T19:01:50.4508927Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-12T19:01:50.4553515Z ##[command]git config gc.auto 0
2020-03-12T19:01:50.4591087Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-12T19:01:50.4621294Z ##[command]git config --get-all http.proxy
2020-03-12T19:01:50.4708398Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69955/merge:refs/remotes/pull/69955/merge
---
2020-03-12T19:10:30.7349529Z configure: build.locked-deps    := True
2020-03-12T19:10:30.7350001Z configure: llvm.ccache          := sccache
2020-03-12T19:10:30.7350938Z configure: build.cargo-native-static := True
2020-03-12T19:10:30.7351627Z configure: dist.missing-tools   := True
2020-03-12T19:10:30.7352423Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-12T19:10:30.7353274Z configure: writing `config.toml` in current directory
2020-03-12T19:10:30.7353627Z configure: 
2020-03-12T19:10:30.7354196Z configure: run `python /checkout/x.py --help`
2020-03-12T19:10:30.7354597Z configure: 
---
2020-03-12T19:12:21.6678985Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-12T19:12:25.9001649Z error: variable does not need to be mutable
2020-03-12T19:12:25.9003359Z    --> src/libstd/sys/windows/mutex.rs:112:13
2020-03-12T19:12:25.9004330Z     |
2020-03-12T19:12:25.9005383Z 112 |         let mut re = box ReentrantMutex::uninitialized();
2020-03-12T19:12:25.9007526Z     |             |
2020-03-12T19:12:25.9008888Z     |             help: remove this `mut`
2020-03-12T19:12:25.9009845Z     |
2020-03-12T19:12:25.9010762Z     = note: `-D unused-mut` implied by `-D warnings`
---
2020-03-12T19:12:26.0688823Z   local time: Thu Mar 12 19:12:26 UTC 2020
2020-03-12T19:12:26.3591487Z   network time: Thu, 12 Mar 2020 19:12:26 GMT
2020-03-12T19:12:26.3591924Z == end clock drift check ==
2020-03-12T19:12:31.5232103Z 
2020-03-12T19:12:31.5306086Z ##[error]Bash exited with code '1'.
2020-03-12T19:12:31.5319656Z ##[section]Finishing: Run build
2020-03-12T19:12:31.5367031Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69955/merge to s
2020-03-12T19:12:31.5372209Z Task         : Get sources
2020-03-12T19:12:31.5372544Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-12T19:12:31.5372836Z Version      : 1.0.0
2020-03-12T19:12:31.5373038Z Author       : Microsoft
2020-03-12T19:12:31.5373038Z Author       : Microsoft
2020-03-12T19:12:31.5373379Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-12T19:12:31.5373756Z ==============================================================================
2020-03-12T19:12:31.8767727Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-12T19:12:31.8811871Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69955/merge to s
2020-03-12T19:12:31.8898367Z Cleaning up task key
2020-03-12T19:12:31.8899608Z Start cleaning up orphan processes.
2020-03-12T19:12:31.9074452Z Terminate orphan process: pid (3977) (python)
2020-03-12T19:12:31.9235171Z ##[section]Finishing: Finalize Job
