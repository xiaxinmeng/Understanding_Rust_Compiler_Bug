plain
2020-01-27T13:02:40.2084404Z ========================== Starting Command Output ===========================
2020-01-27T13:02:40.2085929Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4927cc25-3f67-48d7-af51-3914b91fe529.sh
2020-01-27T13:02:40.2085964Z 
2020-01-27T13:02:40.2088578Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T13:02:40.2094253Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67086/merge to s
2020-01-27T13:02:40.2095966Z Task         : Get sources
2020-01-27T13:02:40.2096003Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T13:02:40.2096038Z Version      : 1.0.0
2020-01-27T13:02:40.2096073Z Author       : Microsoft
---
2020-01-27T13:02:41.0852707Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T13:02:41.0958218Z ##[command]git config gc.auto 0
2020-01-27T13:02:41.1006735Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T13:02:41.1060179Z ##[command]git config --get-all http.proxy
2020-01-27T13:02:41.1193799Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67086/merge:refs/remotes/pull/67086/merge
---
2020-01-27T13:09:23.9763850Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-01-27T13:09:43.3711091Z error: unused variable: `error_code`
2020-01-27T13:09:43.3712445Z    --> src/librustc/infer/error_reporting/need_type_info.rs:218:9
2020-01-27T13:09:43.3712954Z     |
2020-01-27T13:09:43.3713458Z 218 |         error_code: TypeAnnotationNeeded,
2020-01-27T13:09:43.3714015Z     |         ^^^^^^^^^^ help: consider prefixing with an underscore: `_error_code`
2020-01-27T13:09:43.3714984Z     = note: `-D unused-variables` implied by `-D warnings`
2020-01-27T13:09:43.3715210Z 
2020-01-27T13:10:01.0627419Z error: aborting due to previous error
2020-01-27T13:10:01.0628235Z 
2020-01-27T13:10:01.0628235Z 
2020-01-27T13:10:01.0983289Z error: could not compile `rustc`.
2020-01-27T13:10:01.0984101Z 
2020-01-27T13:10:01.0984654Z To learn more, run the command again with --verbose.
2020-01-27T13:10:01.1020212Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-27T13:10:01.1030768Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-27T13:10:01.1031558Z Build completed unsuccessfully in 0:04:17
2020-01-27T13:10:01.1084210Z == clock drift check ==
2020-01-27T13:10:01.1101802Z   local time: Mon Jan 27 13:10:01 UTC 2020
2020-01-27T13:10:01.1101802Z   local time: Mon Jan 27 13:10:01 UTC 2020
2020-01-27T13:10:01.6788722Z   network time: Mon, 27 Jan 2020 13:10:01 GMT
2020-01-27T13:10:01.6790603Z == end clock drift check ==
2020-01-27T13:10:02.0812113Z 
2020-01-27T13:10:02.0902540Z ##[error]Bash exited with code '1'.
2020-01-27T13:10:02.0936347Z ##[section]Finishing: Run build
2020-01-27T13:10:02.0952687Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67086/merge to s
2020-01-27T13:10:02.0954553Z Task         : Get sources
2020-01-27T13:10:02.0954604Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T13:10:02.0954778Z Version      : 1.0.0
2020-01-27T13:10:02.0954822Z Author       : Microsoft
2020-01-27T13:10:02.0954822Z Author       : Microsoft
2020-01-27T13:10:02.0954890Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-27T13:10:02.0954944Z ==============================================================================
2020-01-27T13:10:02.4885320Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-27T13:10:02.4924964Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67086/merge to s
2020-01-27T13:10:02.5039699Z Cleaning up task key
2020-01-27T13:10:02.5040800Z Start cleaning up orphan processes.
2020-01-27T13:10:02.5342040Z Terminate orphan process: pid (3918) (python)
2020-01-27T13:10:02.5364386Z ##[section]Finishing: Finalize Job
