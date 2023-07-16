plain
2020-03-21T11:45:39.6842155Z ========================== Starting Command Output ===========================
2020-03-21T11:45:39.6846511Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dbf72a65-9cf8-461b-8bb2-04a58ae8d0db.sh
2020-03-21T11:45:39.6847086Z 
2020-03-21T11:45:39.6852208Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-21T11:45:39.6872696Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69864/merge to s
2020-03-21T11:45:39.6876997Z Task         : Get sources
2020-03-21T11:45:39.6877299Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T11:45:39.6878938Z Version      : 1.0.0
2020-03-21T11:45:39.6879235Z Author       : Microsoft
---
2020-03-21T11:45:40.6753504Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-21T11:45:40.6766497Z ##[command]git config gc.auto 0
2020-03-21T11:45:40.6770134Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-21T11:45:40.6773729Z ##[command]git config --get-all http.proxy
2020-03-21T11:45:40.6780398Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69864/merge:refs/remotes/pull/69864/merge
---
2020-03-21T11:54:26.8911241Z configure: build.locked-deps    := True
2020-03-21T11:54:26.8911541Z configure: llvm.ccache          := sccache
2020-03-21T11:54:26.8912033Z configure: build.cargo-native-static := True
2020-03-21T11:54:26.8912514Z configure: dist.missing-tools   := True
2020-03-21T11:54:26.8913110Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-21T11:54:26.8913828Z configure: writing `config.toml` in current directory
2020-03-21T11:54:26.8914059Z configure: 
2020-03-21T11:54:26.8915925Z configure: run `python /checkout/x.py --help`
2020-03-21T11:54:26.8916198Z configure: 
---
2020-03-21T12:02:21.4559627Z    Compiling cargo_metadata v0.9.1
2020-03-21T12:02:26.2884867Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-21T12:02:37.8012963Z     Finished release [optimized] target(s) in 26.90s
2020-03-21T12:02:37.8116049Z tidy check
2020-03-21T12:02:44.2020451Z tidy error: /checkout/src/libstd/sys/unix/ext/net.rs: too many lines (3050) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-03-21T12:02:47.4937501Z some tidy checks failed
2020-03-21T12:02:47.4943203Z Found 489 error codes
2020-03-21T12:02:47.4944261Z Found 0 error codes with no tests
2020-03-21T12:02:47.4944489Z Done!
2020-03-21T12:02:47.4944489Z Done!
2020-03-21T12:02:47.4944945Z 
2020-03-21T12:02:47.4945051Z 
2020-03-21T12:02:47.4946473Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-21T12:02:47.4947463Z 
2020-03-21T12:02:47.4947559Z 
2020-03-21T12:02:47.4956945Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-21T12:02:47.4957338Z Build completed unsuccessfully in 0:00:37
2020-03-21T12:02:47.4957338Z Build completed unsuccessfully in 0:00:37
2020-03-21T12:02:47.5010198Z == clock drift check ==
2020-03-21T12:02:47.5027523Z   local time: Sat Mar 21 12:02:47 UTC 2020
2020-03-21T12:02:47.7799003Z   network time: Sat, 21 Mar 2020 12:02:47 GMT
2020-03-21T12:02:47.7812248Z == end clock drift check ==
2020-03-21T12:02:49.3025755Z 
2020-03-21T12:02:49.3110021Z ##[error]Bash exited with code '1'.
2020-03-21T12:02:49.3123726Z ##[section]Finishing: Run build
2020-03-21T12:02:49.3210472Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69864/merge to s
2020-03-21T12:02:49.3216402Z Task         : Get sources
2020-03-21T12:02:49.3216755Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T12:02:49.3217078Z Version      : 1.0.0
2020-03-21T12:02:49.3217322Z Author       : Microsoft
2020-03-21T12:02:49.3217322Z Author       : Microsoft
2020-03-21T12:02:49.3217680Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-21T12:02:49.3218097Z ==============================================================================
2020-03-21T12:02:49.6852394Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-21T12:02:49.6898613Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69864/merge to s
2020-03-21T12:02:49.6989749Z Cleaning up task key
2020-03-21T12:02:49.6991038Z Start cleaning up orphan processes.
2020-03-21T12:02:49.7398010Z Terminate orphan process: pid (3906) (python)
2020-03-21T12:02:49.7441936Z ##[section]Finishing: Finalize Job
