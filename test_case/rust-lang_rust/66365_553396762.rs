plain
2019-11-13T13:07:25.3403235Z [RUSTC-TIMING] tidy test:false 0.581
2019-11-13T13:07:25.3415977Z     Finished release [optimized] target(s) in 1m 17s
2019-11-13T13:07:25.3515420Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 77.817
2019-11-13T13:07:25.3518434Z tidy check
2019-11-13T13:07:26.1108805Z tidy error: /checkout/src/libcore/array/iter.rs:100: undocumented unsafe
2019-11-13T13:07:26.1108996Z tidy error: /checkout/src/libcore/array/iter.rs:196: undocumented unsafe
2019-11-13T13:07:27.7288846Z Found 485 error codes
2019-11-13T13:07:27.7289267Z Found 0 error codes with no tests
2019-11-13T13:07:27.7289385Z Done!
2019-11-13T13:07:27.7289461Z some tidy checks failed
2019-11-13T13:07:27.7289461Z some tidy checks failed
2019-11-13T13:07:27.7289506Z 
2019-11-13T13:07:27.7289540Z 
2019-11-13T13:07:27.7290783Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-13T13:07:27.7290964Z 
2019-11-13T13:07:27.7290997Z 
2019-11-13T13:07:27.7295774Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-13T13:07:27.7295866Z Build completed unsuccessfully in 0:01:21
2019-11-13T13:07:27.7295866Z Build completed unsuccessfully in 0:01:21
2019-11-13T13:07:27.7342506Z == clock drift check ==
2019-11-13T13:07:27.7357044Z   local time: Wed Nov 13 13:07:27 UTC 2019
2019-11-13T13:07:27.8072462Z   network time: Wed, 13 Nov 2019 13:07:27 GMT
2019-11-13T13:07:27.8072807Z == end clock drift check ==
2019-11-13T13:07:29.1221563Z 
2019-11-13T13:07:29.1322722Z ##[error]Bash exited with code '1'.
2019-11-13T13:07:29.1350885Z ##[section]Starting: Checkout
2019-11-13T13:07:29.1353066Z ==============================================================================
2019-11-13T13:07:29.1353141Z Task         : Get sources
2019-11-13T13:07:29.1353227Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
