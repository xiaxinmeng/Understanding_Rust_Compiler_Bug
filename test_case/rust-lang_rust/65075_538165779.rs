plain
2019-10-03T23:20:03.3540103Z [RUSTC-TIMING] tidy test:false 0.683
2019-10-03T23:20:03.3544447Z     Finished release [optimized] target(s) in 1m 29s
2019-10-03T23:20:03.3614356Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 89.831
2019-10-03T23:20:03.3619812Z tidy check
2019-10-03T23:20:04.2472946Z tidy error: duplicate error code: 735
2019-10-03T23:20:04.2473986Z tidy error: /checkout/src/librustc_resolve/error_codes.rs:1709: E0735: r##"
2019-10-03T23:20:04.2474351Z tidy error: /checkout/src/librustc/error_codes.rs:2308:     E0735, // invalid track_caller application/syntax
2019-10-03T23:20:05.3348334Z some tidy checks failed
2019-10-03T23:20:05.3348509Z 
2019-10-03T23:20:05.3348509Z 
2019-10-03T23:20:05.3350334Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-03T23:20:05.3350566Z 
2019-10-03T23:20:05.3350602Z 
2019-10-03T23:20:05.3405063Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-03T23:20:05.3406145Z Build completed unsuccessfully in 0:01:33
2019-10-03T23:20:05.3406145Z Build completed unsuccessfully in 0:01:33
2019-10-03T23:20:05.3417289Z == clock drift check ==
2019-10-03T23:20:05.3457353Z   local time: Thu Oct  3 23:20:05 UTC 2019
2019-10-03T23:20:05.4289728Z   network time: Thu, 03 Oct 2019 23:20:05 GMT
2019-10-03T23:20:05.4296999Z == end clock drift check ==
2019-10-03T23:20:06.8508506Z ##[error]Bash exited with code '1'.
2019-10-03T23:20:06.8540403Z ##[section]Starting: Upload CPU usage statistics
2019-10-03T23:20:06.8544106Z ==============================================================================
2019-10-03T23:20:06.8544227Z Task         : Bash
2019-10-03T23:20:06.8544322Z Description  : Run a Bash script on macOS, Linux, or Windows
