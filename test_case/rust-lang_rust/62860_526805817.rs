plain
2019-08-31T06:37:20.3733120Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 105.615
2019-08-31T06:37:20.3733450Z tidy check
2019-08-31T06:37:30.0986210Z * 578 error codes
2019-08-31T06:37:30.0986880Z * highest error code: E0733
2019-08-31T06:37:30.4603770Z tidy error: /Users/vsts/agent/2.155.1/work/1/s/src/tools/tidy/src/features/tests.rs:5: mismatches the `issue` in previous
2019-08-31T06:37:32.3240330Z some tidy checks failed
2019-08-31T06:37:32.3247220Z 
2019-08-31T06:37:32.3247220Z 
2019-08-31T06:37:32.3249110Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/tidy" "/Users/vsts/agent/2.155.1/work/1/s/src" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage0/bin/cargo" "--no-vendor"
2019-08-31T06:37:32.3250340Z 
2019-08-31T06:37:32.3250810Z 
2019-08-31T06:37:32.3261160Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-31T06:37:32.3261370Z Build completed unsuccessfully in 0:02:01
2019-08-31T06:37:32.3261370Z Build completed unsuccessfully in 0:02:01
2019-08-31T06:37:32.3331030Z == clock drift check ==
2019-08-31T06:37:32.3383380Z   local time: Sat Aug 31 06:37:32 UTC 2019
2019-08-31T06:37:32.4065750Z   network time: Sat, 31 Aug 2019 06:37:32 GMT
2019-08-31T06:37:32.4067640Z == end clock drift check ==
2019-08-31T06:37:32.4244920Z ##[error]Bash exited with code '1'.
2019-08-31T06:37:32.4290980Z ##[section]Starting: Upload CPU usage statistics
2019-08-31T06:37:32.4296210Z ==============================================================================
2019-08-31T06:37:32.4296320Z Task         : Bash
2019-08-31T06:37:32.4296560Z Description  : Run a Bash script on macOS, Linux, or Windows
