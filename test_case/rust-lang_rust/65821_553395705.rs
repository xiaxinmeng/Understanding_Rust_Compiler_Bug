plain
2019-11-13T12:59:48.5966248Z [RUSTC-TIMING] tidy test:false 0.709
2019-11-13T12:59:48.5981317Z     Finished release [optimized] target(s) in 1m 28s
2019-11-13T12:59:48.6091992Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 88.083
2019-11-13T12:59:48.6092258Z tidy check
2019-11-13T12:59:49.4639579Z tidy error: /checkout/src/libcore/array/iter.rs:100: undocumented unsafe
2019-11-13T12:59:49.4639811Z tidy error: /checkout/src/libcore/array/iter.rs:196: undocumented unsafe
2019-11-13T12:59:51.3273113Z Found 485 error codes
2019-11-13T12:59:51.3273854Z Found 0 error codes with no tests
2019-11-13T12:59:51.3274072Z Done!
2019-11-13T12:59:51.3274298Z some tidy checks failed
2019-11-13T12:59:51.3274298Z some tidy checks failed
2019-11-13T12:59:51.3274789Z 
2019-11-13T12:59:51.3274960Z 
2019-11-13T12:59:51.3276182Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-13T12:59:51.3277454Z 
2019-11-13T12:59:51.3277593Z 
2019-11-13T12:59:51.3277876Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-13T12:59:51.3278146Z Build completed unsuccessfully in 0:01:32
2019-11-13T12:59:51.3278146Z Build completed unsuccessfully in 0:01:32
2019-11-13T12:59:51.3328903Z == clock drift check ==
2019-11-13T12:59:51.3338838Z   local time: Wed Nov 13 12:59:51 UTC 2019
2019-11-13T12:59:51.4176767Z   network time: Wed, 13 Nov 2019 12:59:51 GMT
2019-11-13T12:59:51.4177824Z == end clock drift check ==
2019-11-13T12:59:52.7778489Z 
2019-11-13T12:59:52.7886089Z ##[error]Bash exited with code '1'.
2019-11-13T12:59:52.7918766Z ##[section]Starting: Checkout
2019-11-13T12:59:52.7920716Z ==============================================================================
2019-11-13T12:59:52.7920832Z Task         : Get sources
2019-11-13T12:59:52.7920937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
