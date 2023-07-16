plain
2019-10-24T13:23:14.4194865Z     Finished release [optimized] target(s) in 3m 03s
2019-10-24T13:23:14.4323317Z [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 183.387
2019-10-24T13:23:14.6351171Z     Finished release [optimized] target(s) in 0.19s
2019-10-24T13:23:14.6403958Z      Running `build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup`
2019-10-24T13:23:14.6435548Z fatal error: Your xargo is too old; please upgrade to the latest version
2019-10-24T13:23:14.6440757Z 
2019-10-24T13:23:14.6445884Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
2019-10-24T13:23:14.6446125Z expected success, got: exit code: 1
2019-10-24T13:23:14.6446170Z 
---
2019-10-24T13:23:14.6622009Z Verifying status of clippy-driver...
2019-10-24T13:23:14.6633628Z Verifying status of miri...
2019-10-24T13:23:14.6646651Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-10-24T13:23:14.6657089Z 
2019-10-24T13:23:14.6657874Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-10-24T13:23:14.6658461Z 
2019-10-24T13:23:14.6659857Z If you do intend to update 'miri', please check the error messages above and
2019-10-24T13:23:14.6659982Z commit another update.
2019-10-24T13:23:14.6660049Z 
2019-10-24T13:23:14.6660387Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-10-24T13:23:14.6660700Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-10-24T13:23:14.6660789Z proper steps.
2019-10-24T13:23:14.6677761Z   local time: Thu Oct 24 13:23:14 UTC 2019
2019-10-24T13:23:14.8242052Z   network time: Thu, 24 Oct 2019 13:23:14 GMT
2019-10-24T13:23:14.8245925Z == end clock drift check ==
2019-10-24T13:23:15.8217457Z 
2019-10-24T13:23:15.8217457Z 
2019-10-24T13:23:15.8278461Z ##[error]Bash exited with code '3'.
2019-10-24T13:23:15.8313767Z ##[section]Starting: Upload CPU usage statistics
2019-10-24T13:23:15.8326182Z ==============================================================================
2019-10-24T13:23:15.8326434Z Task         : Bash
2019-10-24T13:23:15.8326495Z Description  : Run a Bash script on macOS, Linux, or Windows
