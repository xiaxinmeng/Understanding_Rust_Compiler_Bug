plain
2020-03-18T23:10:18.5707966Z ========================== Starting Command Output ===========================
2020-03-18T23:10:18.5711078Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/39878a8c-65cf-4950-92a9-fa856c5a37ef.sh
2020-03-18T23:10:18.5711488Z 
2020-03-18T23:10:18.5716138Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T23:10:18.5735399Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70122/merge to s
2020-03-18T23:10:18.5738748Z Task         : Get sources
2020-03-18T23:10:18.5739067Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T23:10:18.5739376Z Version      : 1.0.0
2020-03-18T23:10:18.5739579Z Author       : Microsoft
---
2020-03-18T23:10:19.5612796Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T23:10:19.5618137Z ##[command]git config gc.auto 0
2020-03-18T23:10:19.5621703Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T23:10:19.5624993Z ##[command]git config --get-all http.proxy
2020-03-18T23:10:19.5631287Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70122/merge:refs/remotes/pull/70122/merge
---
2020-03-18T23:19:13.8461779Z configure: build.locked-deps    := True
2020-03-18T23:19:13.8462229Z configure: llvm.ccache          := sccache
2020-03-18T23:19:13.8462844Z configure: build.cargo-native-static := True
2020-03-18T23:19:13.8463441Z configure: dist.missing-tools   := True
2020-03-18T23:19:13.8464170Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-18T23:19:13.8464965Z configure: writing `config.toml` in current directory
2020-03-18T23:19:13.8465327Z configure: 
2020-03-18T23:19:13.8465853Z configure: run `python /checkout/x.py --help`
2020-03-18T23:19:13.8508419Z configure: 
---
2020-03-18T23:26:34.2356852Z     Finished release [optimized] target(s) in 8.66s
2020-03-18T23:26:34.2462044Z tidy check
2020-03-18T23:26:39.9038822Z * 591 error codes
2020-03-18T23:26:39.9039158Z * highest error code: E0748
2020-03-18T23:26:40.2572115Z tidy error: Found 1 features without a gate test.
2020-03-18T23:26:40.2573431Z Expected a gate test for the feature 'const_generics'.
2020-03-18T23:26:40.2574192Z Hint: create a failing test file named 'feature-gate-const_generics.rs'
2020-03-18T23:26:40.2574857Z       in the 'ui' test suite, with its failures due to
2020-03-18T23:26:40.2575310Z       missing usage of `#![feature(const_generics)]`.
2020-03-18T23:26:40.2575985Z Hint: If you already have such a test and don't want to rename it,
2020-03-18T23:26:40.2576690Z       you can also add a // gate-test-const_generics line to the test file.
2020-03-18T23:26:41.1660359Z Found 489 error codes
2020-03-18T23:26:41.1660664Z Found 0 error codes with no tests
2020-03-18T23:26:41.1660849Z Done!
2020-03-18T23:26:41.1661014Z some tidy checks failed
2020-03-18T23:26:41.1661014Z some tidy checks failed
2020-03-18T23:26:41.1661171Z 
2020-03-18T23:26:41.1661269Z 
2020-03-18T23:26:41.1663445Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-18T23:26:41.1666061Z 
2020-03-18T23:26:41.1666160Z 
2020-03-18T23:26:41.1666491Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-18T23:26:41.1667250Z Build completed unsuccessfully in 0:00:26
2020-03-18T23:26:41.1667250Z Build completed unsuccessfully in 0:00:26
2020-03-18T23:26:41.1720381Z == clock drift check ==
2020-03-18T23:26:41.1734207Z   local time: Wed Mar 18 23:26:41 UTC 2020
2020-03-18T23:26:41.4618997Z   network time: Wed, 18 Mar 2020 23:26:41 GMT
2020-03-18T23:26:41.4624199Z == end clock drift check ==
2020-03-18T23:26:41.9528400Z 
2020-03-18T23:26:41.9599441Z ##[error]Bash exited with code '1'.
2020-03-18T23:26:41.9614627Z ##[section]Finishing: Run build
2020-03-18T23:26:41.9662549Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70122/merge to s
2020-03-18T23:26:41.9668209Z Task         : Get sources
2020-03-18T23:26:41.9668562Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T23:26:41.9668896Z Version      : 1.0.0
2020-03-18T23:26:41.9669131Z Author       : Microsoft
2020-03-18T23:26:41.9669131Z Author       : Microsoft
2020-03-18T23:26:41.9669492Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T23:26:41.9669915Z ==============================================================================
2020-03-18T23:26:42.2995085Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T23:26:42.3042371Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70122/merge to s
2020-03-18T23:26:42.3131658Z Cleaning up task key
2020-03-18T23:26:42.3133069Z Start cleaning up orphan processes.
2020-03-18T23:26:42.3310319Z Terminate orphan process: pid (4408) (python)
2020-03-18T23:26:42.3573343Z ##[section]Finishing: Finalize Job
