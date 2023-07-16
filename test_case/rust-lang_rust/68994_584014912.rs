plain
2020-02-10T08:36:29.2506312Z ========================== Starting Command Output ===========================
2020-02-10T08:36:29.2796011Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/820f4414-1a5f-4a85-90e5-750a565ef38c.sh
2020-02-10T08:36:29.2796061Z 
2020-02-10T08:36:29.2799622Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T08:36:29.2805323Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68994/merge to s
2020-02-10T08:36:29.2806820Z Task         : Get sources
2020-02-10T08:36:29.2806854Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T08:36:29.2806900Z Version      : 1.0.0
2020-02-10T08:36:29.2806931Z Author       : Microsoft
---
2020-02-10T08:36:30.1914576Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T08:36:30.2023786Z ##[command]git config gc.auto 0
2020-02-10T08:36:30.2085215Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T08:36:30.2121233Z ##[command]git config --get-all http.proxy
2020-02-10T08:36:30.2293755Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68994/merge:refs/remotes/pull/68994/merge
---
2020-02-10T08:45:16.3737789Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-02-10T08:45:19.5452620Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-02-10T08:45:19.7425029Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-02-10T08:45:21.2498188Z error[E0599]: no method named `map_or_default` found for enum `std::option::Option<&str>` in the current scope
2020-02-10T08:45:21.2498556Z    --> src/librustc_codegen_ssa/back/link.rs:768:54
2020-02-10T08:45:21.2498841Z     |
2020-02-10T08:45:21.2499176Z 768 |     let channel = option_env!("CFG_RELEASE_CHANNEL").map_or_default(|channel| format!("-{}", channel));
2020-02-10T08:45:21.2499655Z 
2020-02-10T08:45:22.7096242Z error: aborting due to previous error
2020-02-10T08:45:22.7096401Z 
2020-02-10T08:45:22.7102053Z For more information about this error, try `rustc --explain E0599`.
2020-02-10T08:45:22.7102053Z For more information about this error, try `rustc --explain E0599`.
2020-02-10T08:45:22.7146008Z error: could not compile `rustc_codegen_ssa`.
2020-02-10T08:45:22.7146331Z warning: build failed, waiting for other jobs to finish...
2020-02-10T08:45:26.5501215Z error: build failed
2020-02-10T08:45:26.5539053Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-10T08:45:26.5549655Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-10T08:45:26.5549950Z Build completed unsuccessfully in 0:05:51
2020-02-10T08:45:26.5599824Z == clock drift check ==
2020-02-10T08:45:26.5618823Z   local time: Mon Feb 10 08:45:26 UTC 2020
2020-02-10T08:45:26.5618823Z   local time: Mon Feb 10 08:45:26 UTC 2020
2020-02-10T08:45:26.8389611Z   network time: Mon, 10 Feb 2020 08:45:26 GMT
2020-02-10T08:45:26.8394892Z == end clock drift check ==
2020-02-10T08:45:27.6828023Z 
2020-02-10T08:45:27.6922090Z ##[error]Bash exited with code '1'.
2020-02-10T08:45:27.6935001Z ##[section]Finishing: Run build
2020-02-10T08:45:27.6952720Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68994/merge to s
2020-02-10T08:45:27.6954398Z Task         : Get sources
2020-02-10T08:45:27.6954442Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T08:45:27.6954486Z Version      : 1.0.0
2020-02-10T08:45:27.6954543Z Author       : Microsoft
2020-02-10T08:45:27.6954543Z Author       : Microsoft
2020-02-10T08:45:27.6954587Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-10T08:45:27.6954648Z ==============================================================================
2020-02-10T08:45:28.1144824Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-10T08:45:28.1192447Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68994/merge to s
2020-02-10T08:45:28.1346782Z Cleaning up task key
2020-02-10T08:45:28.1347600Z Start cleaning up orphan processes.
2020-02-10T08:45:28.1457309Z Terminate orphan process: pid (3674) (python)
2020-02-10T08:45:28.1644046Z ##[section]Finishing: Finalize Job
