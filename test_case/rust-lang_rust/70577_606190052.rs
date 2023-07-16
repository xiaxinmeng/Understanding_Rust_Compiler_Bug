plain
2020-03-30T19:12:15.7854324Z ========================== Starting Command Output ===========================
2020-03-30T19:12:15.7859030Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7a0ce29d-b519-4235-8c3e-53da839d932a.sh
2020-03-30T19:12:15.7859403Z 
2020-03-30T19:12:15.7863801Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-30T19:12:15.7891477Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70577/merge to s
2020-03-30T19:12:15.7896815Z Task         : Get sources
2020-03-30T19:12:15.7897136Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T19:12:15.7897448Z Version      : 1.0.0
2020-03-30T19:12:15.7897669Z Author       : Microsoft
---
2020-03-30T19:12:17.7814305Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-30T19:12:17.7821862Z ##[command]git config gc.auto 0
2020-03-30T19:12:17.7826971Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-30T19:12:18.7325070Z ##[command]git config --get-all http.proxy
2020-03-30T19:12:18.7336344Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70577/merge:refs/remotes/pull/70577/merge
---
2020-03-30T19:15:51.6830048Z  ---> 3fc1b512c57b
2020-03-30T19:15:51.6832129Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-30T19:15:51.6834993Z  ---> Using cache
2020-03-30T19:15:51.6836827Z  ---> 5ee4295733f4
2020-03-30T19:15:51.6855518Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-30T19:15:51.6858981Z  ---> 3d07a0fa42fe
2020-03-30T19:15:51.6893138Z Successfully built 3d07a0fa42fe
2020-03-30T19:15:51.6980280Z Successfully tagged rust-ci:latest
2020-03-30T19:15:51.7729411Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-30T19:17:40.1504274Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-03-30T19:17:40.3123077Z     Checking backtrace v0.3.46
2020-03-30T19:17:40.7795799Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-03-30T19:17:40.7849139Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-30T19:17:42.6423772Z error[E0063]: missing field `inner_pos` in initializer of `io::buffered::BufReader<_>`
2020-03-30T19:17:42.6426267Z     |
2020-03-30T19:17:42.6426267Z     |
2020-03-30T19:17:42.6428095Z 105 |             BufReader { inner, buf: buffer.into_boxed_slice(), pos: 0, cap: 0 }
2020-03-30T19:17:42.6429746Z     |             ^^^^^^^^^ missing `inner_pos`
2020-03-30T19:17:43.4369532Z error: aborting due to previous error
2020-03-30T19:17:43.4370001Z 
2020-03-30T19:17:43.4370608Z For more information about this error, try `rustc --explain E0063`.
2020-03-30T19:17:43.4479563Z error: could not compile `std`.
---
2020-03-30T19:17:43.4611663Z   local time: Mon Mar 30 19:17:43 UTC 2020
2020-03-30T19:17:43.6244760Z   network time: Mon, 30 Mar 2020 19:17:43 GMT
2020-03-30T19:17:43.6246847Z == end clock drift check ==
2020-03-30T19:17:44.6372878Z 
2020-03-30T19:17:44.6434271Z ##[error]Bash exited with code '1'.
2020-03-30T19:17:44.6447799Z ##[section]Finishing: Run build
2020-03-30T19:17:44.6491585Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70577/merge to s
2020-03-30T19:17:44.6496035Z Task         : Get sources
2020-03-30T19:17:44.6496361Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T19:17:44.6496666Z Version      : 1.0.0
2020-03-30T19:17:44.6496887Z Author       : Microsoft
2020-03-30T19:17:44.6496887Z Author       : Microsoft
2020-03-30T19:17:44.6497226Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-30T19:17:44.6497616Z ==============================================================================
2020-03-30T19:17:44.9660377Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-30T19:17:44.9703617Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70577/merge to s
2020-03-30T19:17:44.9787127Z Cleaning up task key
2020-03-30T19:17:44.9788276Z Start cleaning up orphan processes.
2020-03-30T19:17:44.9964487Z Terminate orphan process: pid (3699) (python)
2020-03-30T19:17:45.0121035Z ##[section]Finishing: Finalize Job
