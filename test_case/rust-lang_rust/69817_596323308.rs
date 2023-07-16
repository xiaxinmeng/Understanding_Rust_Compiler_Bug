plain
2020-03-09T03:57:13.1180071Z ========================== Starting Command Output ===========================
2020-03-09T03:57:13.1184644Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ef31e75b-8a9e-43c1-829e-d06c2f76d051.sh
2020-03-09T03:57:13.1185092Z 
2020-03-09T03:57:13.1189100Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T03:57:13.1208117Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69817/merge to s
2020-03-09T03:57:13.1212452Z Task         : Get sources
2020-03-09T03:57:13.1212743Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T03:57:13.1213026Z Version      : 1.0.0
2020-03-09T03:57:13.1213236Z Author       : Microsoft
---
2020-03-09T03:57:15.9388691Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T03:57:15.9398637Z ##[command]git config gc.auto 0
2020-03-09T03:57:15.9405686Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T03:57:15.9412192Z ##[command]git config --get-all http.proxy
2020-03-09T03:57:15.9424092Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69817/merge:refs/remotes/pull/69817/merge
---
2020-03-09T04:04:03.5260159Z    Compiling serde_json v1.0.40
2020-03-09T04:04:05.1345566Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-09T04:04:14.9238008Z     Finished release [optimized] target(s) in 1m 20s
2020-03-09T04:04:14.9334817Z tidy check
2020-03-09T04:04:15.0870752Z tidy error: /checkout/src/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:182: line longer than 100 chars
2020-03-09T04:04:15.0871765Z tidy error: /checkout/src/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:194: line longer than 100 chars
2020-03-09T04:04:15.0872754Z tidy error: /checkout/src/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:206: line longer than 100 chars
2020-03-09T04:04:17.6106540Z some tidy checks failed
2020-03-09T04:04:17.6113621Z Found 489 error codes
2020-03-09T04:04:17.6113878Z Found 0 error codes with no tests
2020-03-09T04:04:17.6114105Z Done!
2020-03-09T04:04:17.6114105Z Done!
2020-03-09T04:04:17.6114228Z 
2020-03-09T04:04:17.6114336Z 
2020-03-09T04:04:17.6115698Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-09T04:04:17.6116518Z 
2020-03-09T04:04:17.6116624Z 
2020-03-09T04:04:17.6121564Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-09T04:04:17.6122243Z Build completed unsuccessfully in 0:01:31
2020-03-09T04:04:17.6122243Z Build completed unsuccessfully in 0:01:31
2020-03-09T04:04:17.6173230Z == clock drift check ==
2020-03-09T04:04:17.6183338Z   local time: Mon Mar  9 04:04:17 UTC 2020
2020-03-09T04:04:17.8974701Z   network time: Mon, 09 Mar 2020 04:04:17 GMT
2020-03-09T04:04:17.8978024Z == end clock drift check ==
2020-03-09T04:04:18.8094243Z 
2020-03-09T04:04:18.8143159Z ##[error]Bash exited with code '1'.
2020-03-09T04:04:18.8155462Z ##[section]Finishing: Run build
2020-03-09T04:04:18.8207969Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69817/merge to s
2020-03-09T04:04:18.8212982Z Task         : Get sources
2020-03-09T04:04:18.8213370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T04:04:18.8213705Z Version      : 1.0.0
2020-03-09T04:04:18.8213927Z Author       : Microsoft
2020-03-09T04:04:18.8213927Z Author       : Microsoft
2020-03-09T04:04:18.8214275Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-09T04:04:18.8214706Z ==============================================================================
2020-03-09T04:04:19.1790815Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-09T04:04:19.1839326Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69817/merge to s
2020-03-09T04:04:19.1934127Z Cleaning up task key
2020-03-09T04:04:19.1935421Z Start cleaning up orphan processes.
2020-03-09T04:04:19.2136179Z Terminate orphan process: pid (4082) (python)
2020-03-09T04:04:19.2303642Z ##[section]Finishing: Finalize Job
