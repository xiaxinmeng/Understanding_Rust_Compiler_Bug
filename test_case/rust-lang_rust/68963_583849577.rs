plain
2020-02-09T14:00:04.3825204Z ========================== Starting Command Output ===========================
2020-02-09T14:00:04.3842645Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/72277fbf-513c-4685-8925-fad5e34dd2c9.sh
2020-02-09T14:00:05.2009631Z 
2020-02-09T14:00:05.2083336Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T14:00:05.2093277Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-09T14:00:05.2095025Z Task         : Get sources
2020-02-09T14:00:05.2095059Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T14:00:05.2095089Z Version      : 1.0.0
2020-02-09T14:00:05.2095119Z Author       : Microsoft
---
2020-02-09T14:00:10.8373376Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T14:00:10.8716872Z ##[command]git config gc.auto 0
2020-02-09T14:00:10.8783031Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T14:00:10.8832930Z ##[command]git config --get-all http.proxy
2020-02-09T14:00:10.8988152Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68963/merge:refs/remotes/pull/68963/merge
---
2020-02-09T14:09:15.4320605Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-02-09T14:09:16.4944235Z error[E0308]: mismatched types
2020-02-09T14:09:16.4944734Z    --> src/librustc_resolve/diagnostics.rs:825:67
2020-02-09T14:09:16.4944971Z     |
2020-02-09T14:09:16.4945445Z 825 |                 .map(|span| Ident::new(suggestion.candidate.name, span))
2020-02-09T14:09:16.4946340Z     |                                                                   |
2020-02-09T14:09:16.4946851Z     |                                                                   expected struct `rustc_span::span_encoding::Span`, found `&rustc_span::span_encoding::Span`
2020-02-09T14:09:16.4946851Z     |                                                                   expected struct `rustc_span::span_encoding::Span`, found `&rustc_span::span_encoding::Span`
2020-02-09T14:09:16.4947263Z     |                                                                   help: consider dereferencing the borrow: `*span`
2020-02-09T14:09:17.3081474Z error: aborting due to previous error
2020-02-09T14:09:17.3081573Z 
2020-02-09T14:09:17.3089480Z For more information about this error, try `rustc --explain E0308`.
2020-02-09T14:09:17.3147691Z error: could not compile `rustc_resolve`.
2020-02-09T14:09:17.3147691Z error: could not compile `rustc_resolve`.
2020-02-09T14:09:17.3164176Z warning: build failed, waiting for other jobs to finish...
2020-02-09T14:09:26.9514129Z error: build failed
2020-02-09T14:09:26.9540032Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-09T14:09:26.9550117Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-09T14:09:26.9550339Z Build completed unsuccessfully in 0:06:10
2020-02-09T14:09:26.9610606Z == clock drift check ==
2020-02-09T14:09:26.9627842Z   local time: Sun Feb  9 14:09:26 UTC 2020
2020-02-09T14:09:26.9627842Z   local time: Sun Feb  9 14:09:26 UTC 2020
2020-02-09T14:09:27.2514877Z   network time: Sun, 09 Feb 2020 14:09:27 GMT
2020-02-09T14:09:27.2516503Z == end clock drift check ==
2020-02-09T14:09:28.0937194Z 
2020-02-09T14:09:28.1046832Z ##[error]Bash exited with code '1'.
2020-02-09T14:09:28.1059858Z ##[section]Finishing: Run build
2020-02-09T14:09:28.1078174Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-09T14:09:28.1080520Z Task         : Get sources
2020-02-09T14:09:28.1080564Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T14:09:28.1080624Z Version      : 1.0.0
2020-02-09T14:09:28.1080664Z Author       : Microsoft
2020-02-09T14:09:28.1080664Z Author       : Microsoft
2020-02-09T14:09:28.1080707Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-09T14:09:28.1080769Z ==============================================================================
2020-02-09T14:09:28.5560825Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-09T14:09:28.5603472Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-09T14:09:28.5723373Z Cleaning up task key
2020-02-09T14:09:28.5724117Z Start cleaning up orphan processes.
2020-02-09T14:09:28.5828468Z Terminate orphan process: pid (4272) (python)
2020-02-09T14:09:28.6373509Z ##[section]Finishing: Finalize Job
