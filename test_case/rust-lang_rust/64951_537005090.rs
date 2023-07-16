plain
2019-10-01T12:02:24.0620536Z [RUSTC-TIMING] tidy test:false 0.700
2019-10-01T12:02:24.0642156Z     Finished release [optimized] target(s) in 1m 33s
2019-10-01T12:02:24.0745748Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 93.711
2019-10-01T12:02:24.0745970Z tidy check
2019-10-01T12:02:24.7832394Z tidy error: /checkout/src/librustc/ty/context.rs: too many lines (3016) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-01T12:02:26.1152820Z some tidy checks failed
2019-10-01T12:02:26.1155742Z 
2019-10-01T12:02:26.1155742Z 
2019-10-01T12:02:26.1156807Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-01T12:02:26.1157058Z 
2019-10-01T12:02:26.1157112Z 
2019-10-01T12:02:26.1166724Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-01T12:02:26.1167733Z Build completed unsuccessfully in 0:01:37
2019-10-01T12:02:26.1167733Z Build completed unsuccessfully in 0:01:37
2019-10-01T12:02:26.1219644Z == clock drift check ==
2019-10-01T12:02:26.1235887Z   local time: Tue Oct  1 12:02:26 UTC 2019
2019-10-01T12:02:26.2724083Z   network time: Tue, 01 Oct 2019 12:02:26 GMT
2019-10-01T12:02:26.2729484Z == end clock drift check ==
2019-10-01T12:02:27.5802551Z ##[error]Bash exited with code '1'.
2019-10-01T12:02:27.5844708Z ##[section]Starting: Upload CPU usage statistics
2019-10-01T12:02:27.5849049Z ==============================================================================
2019-10-01T12:02:27.5849183Z Task         : Bash
2019-10-01T12:02:27.5849629Z Description  : Run a Bash script on macOS, Linux, or Windows
