plain
2020-02-06T01:37:00.6053131Z ========================== Starting Command Output ===========================
2020-02-06T01:37:00.6055500Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/401bcf38-52fc-4ae7-a7b8-384883083d2c.sh
2020-02-06T01:37:00.6055696Z 
2020-02-06T01:37:00.6093004Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-06T01:37:00.6099163Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-06T01:37:00.6100717Z Task         : Get sources
2020-02-06T01:37:00.6100751Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T01:37:00.6100825Z Version      : 1.0.0
2020-02-06T01:37:00.6100857Z Author       : Microsoft
---
2020-02-06T01:37:01.4078679Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-06T01:37:01.4170218Z ##[command]git config gc.auto 0
2020-02-06T01:37:01.4244313Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-06T01:37:01.4295486Z ##[command]git config --get-all http.proxy
2020-02-06T01:37:01.4427608Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68751/merge:refs/remotes/pull/68751/merge
---
2020-02-06T01:44:15.6307514Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2020-02-06T01:44:16.8025820Z error[E0308]: mismatched types
2020-02-06T01:44:16.8026167Z    --> src/librustc_lint/unused.rs:594:16
2020-02-06T01:44:16.8026400Z     |
2020-02-06T01:44:16.8027072Z 594 |         if let &Const(.., ref expr) | &Static(.., ref expr) = item.kind {
2020-02-06T01:44:16.8027712Z     |
2020-02-06T01:44:16.8028289Z     = note:   expected enum `syntax::ast::ItemKind`
2020-02-06T01:44:16.8028564Z             found reference `&_`
2020-02-06T01:44:16.8028603Z 
2020-02-06T01:44:16.8028603Z 
2020-02-06T01:44:16.8056901Z error[E0308]: mismatched types
2020-02-06T01:44:16.8057208Z    --> src/librustc_lint/unused.rs:594:39
2020-02-06T01:44:16.8057466Z     |
2020-02-06T01:44:16.8058001Z 594 |         if let &Const(.., ref expr) | &Static(.., ref expr) = item.kind {
2020-02-06T01:44:16.8058657Z     |
2020-02-06T01:44:16.8058971Z     = note:   expected enum `syntax::ast::ItemKind`
2020-02-06T01:44:16.8059239Z             found reference `&_`
2020-02-06T01:44:16.8059277Z 
2020-02-06T01:44:16.8059277Z 
2020-02-06T01:44:16.8271819Z error: aborting due to 2 previous errors
2020-02-06T01:44:16.8271900Z 
2020-02-06T01:44:16.8278707Z For more information about this error, try `rustc --explain E0308`.
2020-02-06T01:44:16.8327022Z error: could not compile `rustc_lint`.
2020-02-06T01:44:16.8327372Z warning: build failed, waiting for other jobs to finish...
2020-02-06T01:44:22.6983158Z error: build failed
2020-02-06T01:44:22.7008699Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-06T01:44:22.7020290Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-06T01:44:22.7020806Z Build completed unsuccessfully in 0:05:05
2020-02-06T01:44:22.7070360Z == clock drift check ==
2020-02-06T01:44:22.7078165Z   local time: Thu Feb  6 01:44:22 UTC 2020
2020-02-06T01:44:22.7078165Z   local time: Thu Feb  6 01:44:22 UTC 2020
2020-02-06T01:44:22.8637200Z   network time: Thu, 06 Feb 2020 01:44:22 GMT
2020-02-06T01:44:22.8638972Z == end clock drift check ==
2020-02-06T01:44:23.4453508Z 
2020-02-06T01:44:23.4543624Z ##[error]Bash exited with code '1'.
2020-02-06T01:44:23.4555282Z ##[section]Finishing: Run build
2020-02-06T01:44:23.4573254Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-06T01:44:23.4575156Z Task         : Get sources
2020-02-06T01:44:23.4575208Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T01:44:23.4575275Z Version      : 1.0.0
2020-02-06T01:44:23.4575321Z Author       : Microsoft
2020-02-06T01:44:23.4575321Z Author       : Microsoft
2020-02-06T01:44:23.4575371Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-06T01:44:23.4575439Z ==============================================================================
2020-02-06T01:44:23.8442253Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-06T01:44:23.8483084Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-06T01:44:23.8590397Z Cleaning up task key
2020-02-06T01:44:23.8591232Z Start cleaning up orphan processes.
2020-02-06T01:44:23.8847388Z Terminate orphan process: pid (4361) (python)
2020-02-06T01:44:23.8868708Z ##[section]Finishing: Finalize Job
