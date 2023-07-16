plain
2019-10-24T07:25:14.6853063Z [RUSTC-TIMING] tidy test:false 0.698
2019-10-24T07:25:14.6861812Z     Finished release [optimized] target(s) in 1m 30s
2019-10-24T07:25:14.6949919Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 90.556
2019-10-24T07:25:14.6953721Z tidy check
2019-10-24T07:25:15.3835184Z tidy error: /checkout/src/librustc/ty/context.rs: too many lines (3012) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-24T07:25:16.7153771Z some tidy checks failed
2019-10-24T07:25:16.7155263Z 
2019-10-24T07:25:16.7155263Z 
2019-10-24T07:25:16.7156610Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-24T07:25:16.7157295Z 
2019-10-24T07:25:16.7157513Z 
2019-10-24T07:25:16.7162074Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-24T07:25:16.7162487Z Build completed unsuccessfully in 0:01:33
2019-10-24T07:25:16.7162487Z Build completed unsuccessfully in 0:01:33
2019-10-24T07:25:16.7216447Z == clock drift check ==
2019-10-24T07:25:16.7232366Z   local time: Thu Oct 24 07:25:16 UTC 2019
2019-10-24T07:25:16.8756948Z   network time: Thu, 24 Oct 2019 07:25:16 GMT
2019-10-24T07:25:16.8761953Z == end clock drift check ==
2019-10-24T07:25:18.3668418Z 
2019-10-24T07:25:18.3815196Z ##[error]Bash exited with code '1'.
2019-10-24T07:25:18.3856472Z ##[section]Starting: Upload CPU usage statistics
2019-10-24T07:25:18.3859890Z ==============================================================================
2019-10-24T07:25:18.3859997Z Task         : Bash
2019-10-24T07:25:18.3860073Z Description  : Run a Bash script on macOS, Linux, or Windows
