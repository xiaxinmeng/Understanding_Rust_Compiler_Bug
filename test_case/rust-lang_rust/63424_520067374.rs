plain
2019-08-09T21:18:58.7921380Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-apple-darwin" }, target: "i686-apple-darwin", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 104.415
2019-08-09T21:18:58.7922680Z tidy check
2019-08-09T21:19:09.6954750Z * 578 error codes
2019-08-09T21:19:09.6955130Z * highest error code: E0733
2019-08-09T21:19:10.1406460Z tidy error: /Users/vsts/agent/2.155.1/work/1/s/src/tools/tidy/src/features/tests.rs:5: mismatches the `issue` in previous
2019-08-09T21:19:12.4116810Z some tidy checks failed
2019-08-09T21:19:12.4123280Z 
2019-08-09T21:19:12.4123280Z 
2019-08-09T21:19:12.4125210Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/tidy" "/Users/vsts/agent/2.155.1/work/1/s/src" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0/bin/cargo" "--no-vendor"
2019-08-09T21:19:12.4126480Z 
2019-08-09T21:19:12.4127000Z 
2019-08-09T21:19:12.4136450Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-09T21:19:12.4136880Z Build completed unsuccessfully in 0:02:03
2019-08-09T21:19:12.4136880Z Build completed unsuccessfully in 0:02:03
2019-08-09T21:19:12.4402290Z ##[error]Bash exited with code '1'.
2019-08-09T21:19:12.4447970Z ##[section]Starting: Upload CPU usage statistics
2019-08-09T21:19:12.4452800Z ==============================================================================
2019-08-09T21:19:12.4452920Z Task         : Bash
2019-08-09T21:19:12.4453000Z Description  : Run a Bash script on macOS, Linux, or Windows
