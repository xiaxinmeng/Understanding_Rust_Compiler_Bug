plain
2020-03-22T19:26:01.2417430Z ========================== Starting Command Output ===========================
2020-03-22T19:26:01.2419880Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3f23690c-ce8f-4342-a363-dcb68ee94b28.sh
2020-03-22T19:26:01.2420153Z 
2020-03-22T19:26:01.2424331Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T19:26:01.2443279Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70267/merge to s
2020-03-22T19:26:01.2447455Z Task         : Get sources
2020-03-22T19:26:01.2447768Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T19:26:01.2448068Z Version      : 1.0.0
2020-03-22T19:26:01.2448271Z Author       : Microsoft
---
2020-03-22T19:26:02.4977335Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T19:26:02.4985204Z ##[command]git config gc.auto 0
2020-03-22T19:26:02.4988948Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T19:26:02.4992443Z ##[command]git config --get-all http.proxy
2020-03-22T19:26:02.4999546Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70267/merge:refs/remotes/pull/70267/merge
---
2020-03-22T19:32:41.4685863Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-22T19:32:42.4847725Z error: unused import: `TypeId`
2020-03-22T19:32:42.4848801Z   --> src/librustc/mir/interpret/error.rs:18:16
2020-03-22T19:32:42.4849535Z    |
2020-03-22T19:32:42.4850399Z 18 |     any::{Any, TypeId},
2020-03-22T19:32:42.4851998Z    |
2020-03-22T19:32:42.4852900Z    = note: `-D unused-imports` implied by `-D warnings`
2020-03-22T19:32:42.4856573Z 
2020-03-22T19:32:51.5869243Z error: aborting due to previous error
2020-03-22T19:32:51.5869243Z error: aborting due to previous error
2020-03-22T19:32:51.5870206Z 
2020-03-22T19:32:51.6143277Z error: could not compile `rustc`.
2020-03-22T19:32:51.6144251Z 
2020-03-22T19:32:51.6145660Z To learn more, run the command again with --verbose.
2020-03-22T19:32:51.6170205Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-22T19:32:51.6181874Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-22T19:32:51.6182623Z Build completed unsuccessfully in 0:04:01
2020-03-22T19:32:51.6238253Z == clock drift check ==
2020-03-22T19:32:51.6254316Z   local time: Sun Mar 22 19:32:51 UTC 2020
2020-03-22T19:32:51.6254316Z   local time: Sun Mar 22 19:32:51 UTC 2020
2020-03-22T19:32:51.9152318Z   network time: Sun, 22 Mar 2020 19:32:51 GMT
2020-03-22T19:32:51.9154350Z == end clock drift check ==
2020-03-22T19:32:52.7854956Z 
2020-03-22T19:32:52.7901832Z ##[error]Bash exited with code '1'.
2020-03-22T19:32:52.7914266Z ##[section]Finishing: Run build
2020-03-22T19:32:52.8032743Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70267/merge to s
2020-03-22T19:32:52.8037799Z Task         : Get sources
2020-03-22T19:32:52.8038151Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T19:32:52.8038471Z Version      : 1.0.0
2020-03-22T19:32:52.8038712Z Author       : Microsoft
2020-03-22T19:32:52.8038712Z Author       : Microsoft
2020-03-22T19:32:52.8039067Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T19:32:52.8039490Z ==============================================================================
2020-03-22T19:32:53.1315092Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T19:32:53.1359867Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70267/merge to s
2020-03-22T19:32:53.1451429Z Cleaning up task key
2020-03-22T19:32:53.1452691Z Start cleaning up orphan processes.
2020-03-22T19:32:53.1651679Z Terminate orphan process: pid (3617) (python)
2020-03-22T19:32:53.1866254Z ##[section]Finishing: Finalize Job
