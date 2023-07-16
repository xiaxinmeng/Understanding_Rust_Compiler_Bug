plain
2020-01-23T23:43:59.5041680Z [RUSTC-TIMING] tidy test:false 0.597
2020-01-23T23:43:59.5257250Z     Finished release [optimized] target(s) in 1m 37s
2020-01-23T23:43:59.5411750Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-apple-darwin" }, target: "i686-apple-darwin", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 97.722
2020-01-23T23:43:59.5412850Z tidy check
2020-01-23T23:44:01.1264800Z tidy error: /Users/runner/runners/2.164.6/work/1/s/src/librustc_errors/emitter.rs:128: line longer than 100 chars
2020-01-23T23:44:01.1267630Z tidy error: /Users/runner/runners/2.164.6/work/1/s/src/librustc_errors/emitter.rs:586: line longer than 100 chars
2020-01-23T23:44:06.0016710Z Found 485 error codes
2020-01-23T23:44:06.0017680Z Found 0 error codes with no tests
2020-01-23T23:44:06.0018120Z Done!
2020-01-23T23:44:06.0024340Z some tidy checks failed
2020-01-23T23:44:06.0024340Z some tidy checks failed
2020-01-23T23:44:06.0024890Z 
2020-01-23T23:44:06.0025310Z 
2020-01-23T23:44:06.0026830Z command did not execute successfully: "/Users/runner/runners/2.164.6/work/1/s/build/i686-apple-darwin/stage0-tools-bin/tidy" "/Users/runner/runners/2.164.6/work/1/s/src" "/Users/runner/runners/2.164.6/work/1/s/build/i686-apple-darwin/stage0/bin/cargo" "--no-vendor"
2020-01-23T23:44:06.0028050Z 
2020-01-23T23:44:06.0028450Z 
2020-01-23T23:44:06.0035330Z failed to run: /Users/runner/runners/2.164.6/work/1/s/build/bootstrap/debug/bootstrap test
2020-01-23T23:44:06.0035860Z Build completed unsuccessfully in 0:01:48
2020-01-23T23:44:06.0035860Z Build completed unsuccessfully in 0:01:48
2020-01-23T23:44:06.0098760Z == clock drift check ==
2020-01-23T23:44:06.0153300Z   local time: Thu Jan 23 23:44:06 UTC 2020
2020-01-23T23:44:06.0574520Z   network time: Thu, 23 Jan 2020 23:44:06 GMT
2020-01-23T23:44:06.0576010Z == end clock drift check ==
2020-01-23T23:44:06.0617530Z 
2020-01-23T23:44:06.0733470Z ##[error]Bash exited with code '1'.
2020-01-23T23:44:06.0843780Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-23T23:44:06.0846700Z ==============================================================================
2020-01-23T23:44:06.0846990Z Task         : Get sources
2020-01-23T23:44:06.0847090Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
