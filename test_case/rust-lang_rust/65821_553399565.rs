plain
2019-11-13T13:15:27.4676662Z [RUSTC-TIMING] tidy test:false 0.657
2019-11-13T13:15:27.4693864Z     Finished release [optimized] target(s) in 1m 26s
2019-11-13T13:15:27.4964476Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 86.122
2019-11-13T13:15:27.4968720Z tidy check
2019-11-13T13:15:28.3628288Z tidy error: /checkout/src/libcore/array/iter.rs:100: undocumented unsafe
2019-11-13T13:15:28.3628544Z tidy error: /checkout/src/libcore/array/iter.rs:196: undocumented unsafe
2019-11-13T13:15:30.1814600Z some tidy checks failed
2019-11-13T13:15:30.1814761Z Found 485 error codes
2019-11-13T13:15:30.1814859Z Found 0 error codes with no tests
2019-11-13T13:15:30.1814939Z Done!
2019-11-13T13:15:30.1814939Z Done!
2019-11-13T13:15:30.1814983Z 
2019-11-13T13:15:30.1815044Z 
2019-11-13T13:15:30.1816315Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-13T13:15:30.1816873Z 
2019-11-13T13:15:30.1816930Z 
2019-11-13T13:15:30.1821665Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-13T13:15:30.1821809Z Build completed unsuccessfully in 0:01:30
2019-11-13T13:15:30.1821809Z Build completed unsuccessfully in 0:01:30
2019-11-13T13:15:30.1873984Z == clock drift check ==
2019-11-13T13:15:30.1901375Z   local time: Wed Nov 13 13:15:30 UTC 2019
2019-11-13T13:15:30.2592628Z   network time: Wed, 13 Nov 2019 13:15:30 GMT
2019-11-13T13:15:30.2594298Z == end clock drift check ==
2019-11-13T13:15:31.5866540Z 
2019-11-13T13:15:31.5963682Z ##[error]Bash exited with code '1'.
2019-11-13T13:15:31.5996753Z ##[section]Starting: Checkout
2019-11-13T13:15:31.5998681Z ==============================================================================
2019-11-13T13:15:31.5998803Z Task         : Get sources
2019-11-13T13:15:31.5998900Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
