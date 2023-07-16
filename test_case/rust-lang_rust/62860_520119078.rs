plain
2019-08-10T05:08:41.6131770Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-apple-darwin" }, target: "i686-apple-darwin", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 101.711
2019-08-10T05:08:41.6132800Z tidy check
2019-08-10T05:08:52.1732190Z * 578 error codes
2019-08-10T05:08:52.1732670Z * highest error code: E0733
2019-08-10T05:08:52.5661040Z tidy error: /Users/vsts/agent/2.155.1/work/1/s/src/tools/tidy/src/features/tests.rs:5: mismatches the `issue` in previous
2019-08-10T05:08:54.4722900Z some tidy checks failed
2019-08-10T05:08:54.4729450Z 
2019-08-10T05:08:54.4729450Z 
2019-08-10T05:08:54.4731430Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/tidy" "/Users/vsts/agent/2.155.1/work/1/s/src" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0/bin/cargo" "--no-vendor"
2019-08-10T05:08:54.4732760Z 
2019-08-10T05:08:54.4733310Z 
2019-08-10T05:08:54.4741490Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-10T05:08:54.4742160Z Build completed unsuccessfully in 0:01:58
2019-08-10T05:08:54.4742160Z Build completed unsuccessfully in 0:01:58
2019-08-10T05:08:54.4989600Z ##[error]Bash exited with code '1'.
2019-08-10T05:08:54.5041490Z ##[section]Starting: Upload CPU usage statistics
2019-08-10T05:08:54.5046340Z ==============================================================================
2019-08-10T05:08:54.5046450Z Task         : Bash
2019-08-10T05:08:54.5046540Z Description  : Run a Bash script on macOS, Linux, or Windows
