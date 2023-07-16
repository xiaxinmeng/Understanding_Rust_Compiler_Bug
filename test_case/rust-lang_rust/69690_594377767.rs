plain
2020-03-04T07:39:20.6476841Z ========================== Starting Command Output ===========================
2020-03-04T07:39:20.6479653Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f4cedfd7-558a-4cff-8422-ea808490d991.sh
2020-03-04T07:39:20.6480259Z 
2020-03-04T07:39:20.6484131Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-04T07:39:20.6503503Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69690/merge to s
2020-03-04T07:39:20.6507198Z Task         : Get sources
2020-03-04T07:39:20.6507516Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T07:39:20.6507798Z Version      : 1.0.0
2020-03-04T07:39:20.6507986Z Author       : Microsoft
---
2020-03-04T07:39:22.8246262Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-04T07:39:22.8494398Z ##[command]git config gc.auto 0
2020-03-04T07:39:22.8500383Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-04T07:39:22.8505449Z ##[command]git config --get-all http.proxy
2020-03-04T07:39:22.8533432Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69690/merge:refs/remotes/pull/69690/merge
---
2020-03-04T07:46:10.6646543Z    Compiling serde_json v1.0.40
2020-03-04T07:46:12.2639094Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-04T07:46:21.9648787Z     Finished release [optimized] target(s) in 1m 27s
2020-03-04T07:46:21.9736883Z tidy check
2020-03-04T07:46:22.2405170Z tidy error: /checkout/src/test/ui/or-patterns/slice-patterns.rs:49: line longer than 100 chars
2020-03-04T07:46:24.4931279Z some tidy checks failed
2020-03-04T07:46:24.4932111Z Found 489 error codes
2020-03-04T07:46:24.4932465Z Found 0 error codes with no tests
2020-03-04T07:46:24.4932777Z Done!
2020-03-04T07:46:24.4932777Z Done!
2020-03-04T07:46:24.4935401Z 
2020-03-04T07:46:24.4935731Z 
2020-03-04T07:46:24.4937304Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-04T07:46:24.4938402Z 
2020-03-04T07:46:24.4938618Z 
2020-03-04T07:46:24.4944780Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-04T07:46:24.4945298Z Build completed unsuccessfully in 0:01:37
2020-03-04T07:46:24.4945298Z Build completed unsuccessfully in 0:01:37
2020-03-04T07:46:24.4995654Z == clock drift check ==
2020-03-04T07:46:24.5004758Z   local time: Wed Mar  4 07:46:24 UTC 2020
2020-03-04T07:46:25.0482135Z   network time: Wed, 04 Mar 2020 07:46:25 GMT
2020-03-04T07:46:25.0482451Z == end clock drift check ==
2020-03-04T07:46:25.7935500Z 
2020-03-04T07:46:25.7978562Z ##[error]Bash exited with code '1'.
2020-03-04T07:46:25.7996027Z ##[section]Finishing: Run build
2020-03-04T07:46:25.8040441Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69690/merge to s
2020-03-04T07:46:25.8045826Z Task         : Get sources
2020-03-04T07:46:25.8046189Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T07:46:25.8046507Z Version      : 1.0.0
2020-03-04T07:46:25.8046728Z Author       : Microsoft
2020-03-04T07:46:25.8046728Z Author       : Microsoft
2020-03-04T07:46:25.8047104Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-04T07:46:25.8047510Z ==============================================================================
2020-03-04T07:46:26.1321437Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-04T07:46:26.1361624Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69690/merge to s
2020-03-04T07:46:26.1445833Z Cleaning up task key
2020-03-04T07:46:26.1447075Z Start cleaning up orphan processes.
2020-03-04T07:46:26.1617798Z Terminate orphan process: pid (3842) (python)
2020-03-04T07:46:26.1799272Z ##[section]Finishing: Finalize Job
