plain
2020-03-18T01:53:34.0708748Z ========================== Starting Command Output ===========================
2020-03-18T01:53:34.0711629Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8234da10-2c46-43e5-9224-84df59cb321e.sh
2020-03-18T01:53:34.0711951Z 
2020-03-18T01:53:34.0716774Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T01:53:34.0737531Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70091/merge to s
2020-03-18T01:53:34.0741868Z Task         : Get sources
2020-03-18T01:53:34.0742201Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T01:53:34.0742515Z Version      : 1.0.0
2020-03-18T01:53:34.0743792Z Author       : Microsoft
---
2020-03-18T01:53:35.2090725Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T01:53:35.2099111Z ##[command]git config gc.auto 0
2020-03-18T01:53:35.2114085Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T01:53:35.2117649Z ##[command]git config --get-all http.proxy
2020-03-18T01:53:35.2133976Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70091/merge:refs/remotes/pull/70091/merge
---
2020-03-18T02:00:27.2129929Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-18T02:00:28.7219048Z error: unused variable: `tokens`
2020-03-18T02:00:28.7220458Z     --> src/librustc_parse/parser/expr.rs:2134:9
2020-03-18T02:00:28.7221567Z      |
2020-03-18T02:00:28.7222538Z 2134 |         tokens: Option<TokenStream>,
2020-03-18T02:00:28.7224281Z      |         ^^^^^^ help: consider prefixing with an underscore: `_tokens`
2020-03-18T02:00:28.7226264Z      = note: `-D unused-variables` implied by `-D warnings`
2020-03-18T02:00:28.7226793Z 
2020-03-18T02:00:29.5294120Z error: aborting due to previous error
2020-03-18T02:00:29.5294879Z 
2020-03-18T02:00:29.5294879Z 
2020-03-18T02:00:29.5374184Z error: could not compile `rustc_parse`.
2020-03-18T02:00:29.5378485Z 
2020-03-18T02:00:29.5379181Z To learn more, run the command again with --verbose.
2020-03-18T02:00:29.5396461Z warning: build failed, waiting for other jobs to finish...
2020-03-18T02:00:30.3367502Z error: build failed
2020-03-18T02:00:30.3370440Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-18T02:00:30.3372442Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-18T02:00:30.3372972Z Build completed unsuccessfully in 0:04:14
2020-03-18T02:00:30.3373389Z == clock drift check ==
2020-03-18T02:00:30.3373779Z   local time: Wed Mar 18 02:00:29 UTC 2020
2020-03-18T02:00:30.3373779Z   local time: Wed Mar 18 02:00:29 UTC 2020
2020-03-18T02:00:30.3374253Z   network time: Wed, 18 Mar 2020 02:00:30 GMT
2020-03-18T02:00:30.3374686Z == end clock drift check ==
2020-03-18T02:00:30.9943242Z 
2020-03-18T02:00:31.0023438Z ##[error]Bash exited with code '1'.
2020-03-18T02:00:31.0049128Z ##[section]Finishing: Run build
2020-03-18T02:00:31.0107013Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70091/merge to s
2020-03-18T02:00:31.0112430Z Task         : Get sources
2020-03-18T02:00:31.0112855Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T02:00:31.0113231Z Version      : 1.0.0
2020-03-18T02:00:31.0113485Z Author       : Microsoft
2020-03-18T02:00:31.0113485Z Author       : Microsoft
2020-03-18T02:00:31.0113909Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T02:00:31.0114401Z ==============================================================================
2020-03-18T02:00:31.3848888Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T02:00:31.3914358Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70091/merge to s
2020-03-18T02:00:31.4042609Z Cleaning up task key
2020-03-18T02:00:31.4044468Z Start cleaning up orphan processes.
2020-03-18T02:00:31.4302879Z Terminate orphan process: pid (3625) (python)
2020-03-18T02:00:31.4533403Z ##[section]Finishing: Finalize Job
