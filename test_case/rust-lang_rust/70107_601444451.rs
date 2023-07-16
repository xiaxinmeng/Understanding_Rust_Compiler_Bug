plain
2020-03-19T22:12:21.6271122Z ========================== Starting Command Output ===========================
2020-03-19T22:12:21.6273833Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d36fe671-ac73-4be4-b6a5-d0277f12c8e4.sh
2020-03-19T22:12:21.6274107Z 
2020-03-19T22:12:21.6277078Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T22:12:21.6294938Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-19T22:12:21.6298252Z Task         : Get sources
2020-03-19T22:12:21.6298496Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T22:12:21.6299387Z Version      : 1.0.0
2020-03-19T22:12:21.6299562Z Author       : Microsoft
---
2020-03-19T22:12:23.9264878Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T22:12:23.9272203Z ##[command]git config gc.auto 0
2020-03-19T22:12:23.9277154Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T22:12:23.9281688Z ##[command]git config --get-all http.proxy
2020-03-19T22:12:23.9290040Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70107/merge:refs/remotes/pull/70107/merge
---
2020-03-19T22:19:46.9550711Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-19T22:19:48.6316578Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-19T22:19:49.2491129Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-19T22:19:51.0305780Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-19T22:19:51.0306919Z error: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
2020-03-19T22:19:51.0312275Z     --> src/librustc/ty/print/pretty.rs:2034:52
2020-03-19T22:19:51.0312912Z      |
2020-03-19T22:19:51.0313757Z 2034 |                 p!(print(c), write(" well-formed")),
2020-03-19T22:19:51.0314655Z      |                                                    ^ expected one of `.`, `;`, `?`, `}`, or an operator
2020-03-19T22:20:01.8081524Z error: aborting due to previous error
2020-03-19T22:20:01.8082413Z 
2020-03-19T22:20:01.8305374Z error: could not compile `rustc`.
2020-03-19T22:20:01.8305973Z 
2020-03-19T22:20:01.8305973Z 
2020-03-19T22:20:01.8306586Z To learn more, run the command again with --verbose.
2020-03-19T22:20:01.8327758Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-19T22:20:01.8342493Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-19T22:20:01.8342765Z Build completed unsuccessfully in 0:03:33
2020-03-19T22:20:01.8395255Z == clock drift check ==
2020-03-19T22:20:01.8419599Z   local time: Thu Mar 19 22:20:01 UTC 2020
2020-03-19T22:20:01.8419599Z   local time: Thu Mar 19 22:20:01 UTC 2020
2020-03-19T22:20:02.1062440Z   network time: Thu, 19 Mar 2020 22:20:02 GMT
2020-03-19T22:20:02.1066565Z == end clock drift check ==
2020-03-19T22:20:03.3136564Z 
2020-03-19T22:20:03.3169849Z ##[error]Bash exited with code '1'.
2020-03-19T22:20:03.3182636Z ##[section]Finishing: Run build
2020-03-19T22:20:03.3226596Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-19T22:20:03.3230664Z Task         : Get sources
2020-03-19T22:20:03.3230921Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T22:20:03.3231174Z Version      : 1.0.0
2020-03-19T22:20:03.3231343Z Author       : Microsoft
2020-03-19T22:20:03.3231343Z Author       : Microsoft
2020-03-19T22:20:03.3231611Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T22:20:03.3231931Z ==============================================================================
2020-03-19T22:20:03.5965930Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T22:20:03.6007177Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-19T22:20:03.6082483Z Cleaning up task key
2020-03-19T22:20:03.6083532Z Start cleaning up orphan processes.
2020-03-19T22:20:03.6367178Z Terminate orphan process: pid (9461) (python)
2020-03-19T22:20:03.6396282Z ##[section]Finishing: Finalize Job
