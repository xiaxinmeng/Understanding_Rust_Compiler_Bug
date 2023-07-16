plain
2020-03-31T16:30:41.4590685Z ========================== Starting Command Output ===========================
2020-03-31T16:30:41.4593902Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/26ef4fb0-cd82-4ea2-bf25-b3e91c70482f.sh
2020-03-31T16:30:41.4594143Z 
2020-03-31T16:30:41.4597593Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T16:30:41.4615966Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70597/merge to s
2020-03-31T16:30:41.4619072Z Task         : Get sources
2020-03-31T16:30:41.4619384Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T16:30:41.4619640Z Version      : 1.0.0
2020-03-31T16:30:41.4619814Z Author       : Microsoft
---
2020-03-31T16:30:42.6660851Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T16:30:42.6666690Z ##[command]git config gc.auto 0
2020-03-31T16:30:42.6670306Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T16:30:42.6673166Z ##[command]git config --get-all http.proxy
2020-03-31T16:30:42.6679766Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70597/merge:refs/remotes/pull/70597/merge
---
2020-03-31T16:32:52.0161745Z  ---> 3fc1b512c57b
2020-03-31T16:32:52.0161939Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-31T16:32:52.0162444Z  ---> Using cache
2020-03-31T16:32:52.0162986Z  ---> 5ee4295733f4
2020-03-31T16:32:52.0165916Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-31T16:32:52.0167541Z  ---> 3d07a0fa42fe
2020-03-31T16:32:52.0170713Z Successfully built 3d07a0fa42fe
2020-03-31T16:32:52.0195090Z Successfully tagged rust-ci:latest
2020-03-31T16:32:52.0468904Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-31T16:34:53.4112016Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-03-31T16:34:53.5805505Z     Checking backtrace v0.3.46
2020-03-31T16:34:54.2271213Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-03-31T16:34:54.2293375Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-31T16:34:57.1887056Z error[E0606]: casting `&mut alloc_crate::boxed::Box<dyn core::ops::FnOnce()>` as `*mut libc::c_void` is invalid
2020-03-31T16:34:57.1888337Z    |
2020-03-31T16:34:57.1888337Z    |
2020-03-31T16:34:57.1895547Z 72 |             &mut *p as &mut Box<dyn FnOnce()> as *mut _,
2020-03-31T16:34:57.1897047Z 
2020-03-31T16:34:57.2336388Z error: aborting due to previous error
2020-03-31T16:34:57.2336649Z 
2020-03-31T16:34:57.2337424Z For more information about this error, try `rustc --explain E0606`.
---
2020-03-31T16:34:57.2553344Z   local time: Tue Mar 31 16:34:57 UTC 2020
2020-03-31T16:34:57.5214896Z   network time: Tue, 31 Mar 2020 16:34:57 GMT
2020-03-31T16:34:57.5215366Z == end clock drift check ==
2020-03-31T16:34:58.7530913Z 
2020-03-31T16:34:58.7608659Z ##[error]Bash exited with code '1'.
2020-03-31T16:34:58.7663954Z ##[section]Finishing: Run build
2020-03-31T16:34:58.7715373Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70597/merge to s
2020-03-31T16:34:58.7722060Z Task         : Get sources
2020-03-31T16:34:58.7722416Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T16:34:58.7722766Z Version      : 1.0.0
2020-03-31T16:34:58.7722992Z Author       : Microsoft
2020-03-31T16:34:58.7722992Z Author       : Microsoft
2020-03-31T16:34:58.7723363Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T16:34:58.7723822Z ==============================================================================
2020-03-31T16:34:59.1040829Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T16:34:59.1091769Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70597/merge to s
2020-03-31T16:34:59.1178576Z Cleaning up task key
2020-03-31T16:34:59.1179922Z Start cleaning up orphan processes.
2020-03-31T16:34:59.1375881Z Terminate orphan process: pid (6118) (python)
2020-03-31T16:34:59.1545308Z ##[section]Finishing: Finalize Job
