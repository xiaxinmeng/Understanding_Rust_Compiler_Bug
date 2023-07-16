plain
2019-10-01T12:17:41.5748603Z [RUSTC-TIMING] tidy test:false 0.663
2019-10-01T12:17:41.5768336Z     Finished release [optimized] target(s) in 1m 26s
2019-10-01T12:17:41.5894709Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 86.459
2019-10-01T12:17:41.5897238Z tidy check
2019-10-01T12:17:42.2159260Z tidy error: /checkout/src/librustc/ty/context.rs: too many lines (3016) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-01T12:17:43.4862929Z some tidy checks failed
2019-10-01T12:17:43.4864846Z 
2019-10-01T12:17:43.4864846Z 
2019-10-01T12:17:43.4867677Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-01T12:17:43.4867916Z 
2019-10-01T12:17:43.4867952Z 
2019-10-01T12:17:43.4880878Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-01T12:17:43.4881182Z Build completed unsuccessfully in 0:01:29
2019-10-01T12:17:43.4881182Z Build completed unsuccessfully in 0:01:29
2019-10-01T12:17:43.4930233Z == clock drift check ==
2019-10-01T12:17:43.4943728Z   local time: Tue Oct  1 12:17:43 UTC 2019
2019-10-01T12:17:44.3649034Z   network time: Tue, 01 Oct 2019 12:17:44 GMT
2019-10-01T12:17:44.3651949Z == end clock drift check ==
2019-10-01T12:17:45.7407749Z ##[error]Bash exited with code '1'.
2019-10-01T12:17:45.7461774Z ##[section]Starting: Upload CPU usage statistics
2019-10-01T12:17:45.7465063Z ==============================================================================
2019-10-01T12:17:45.7465138Z Task         : Bash
2019-10-01T12:17:45.7465214Z Description  : Run a Bash script on macOS, Linux, or Windows
