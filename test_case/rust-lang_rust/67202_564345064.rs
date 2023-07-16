plain
2019-12-11T01:22:19.3961808Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-12-11T01:22:24.2012464Z [RUSTC-TIMING] rustbook test:false 4.795
2019-12-11T01:22:24.2047041Z     Finished release [optimized] target(s) in 10m 48s
2019-12-11T01:22:24.2272503Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: ["linkcheck"] } -- 648.213
2019-12-11T01:24:01.0029110Z error: The server responded with 404 Not Found for "https://discord.gg/rust-lang"
2019-12-11T01:24:01.0030926Z     ┌── compiler-team.md:15:33 ───
2019-12-11T01:24:01.0031531Z     │
2019-12-11T01:24:01.0031531Z     │
2019-12-11T01:24:01.0032333Z  15 │ - The `compiler` channel on the [rust-lang discord](https://discord.gg/rust-lang)
2019-12-11T01:24:01.0034356Z     │
2019-12-11T01:24:01.0034592Z 
2019-12-11T01:24:01.0040000Z Error: One or more incorrect links
2019-12-11T01:24:01.0044617Z 
---
2019-12-11T02:08:14.3447696Z Verifying status of clippy-driver...
2019-12-11T02:08:14.3447781Z Verifying status of miri...
2019-12-11T02:08:14.3448069Z Verifying status of embedded-book...
2019-12-11T02:08:14.3448340Z Verifying status of rustc-guide...
2019-12-11T02:08:14.3448686Z error: Tool `rls` should be test-pass but is build-fail during beta week.
2019-12-11T02:08:14.3453551Z Build completed unsuccessfully in 0:00:01
2019-12-11T02:08:14.3508055Z == clock drift check ==
2019-12-11T02:08:14.3524888Z   local time: Wed Dec 11 02:08:14 UTC 2019
2019-12-11T02:08:14.5208758Z   network time: Wed, 11 Dec 2019 02:08:14 GMT
2019-12-11T02:08:14.5208758Z   network time: Wed, 11 Dec 2019 02:08:14 GMT
2019-12-11T02:08:14.5213255Z == end clock drift check ==
2019-12-11T02:08:15.0662302Z 
2019-12-11T02:08:15.0766289Z ##[error]Bash exited with code '1'.
2019-12-11T02:08:15.0820199Z ##[section]Starting: Checkout
2019-12-11T02:08:15.0822417Z ==============================================================================
2019-12-11T02:08:15.0822520Z Task         : Get sources
2019-12-11T02:08:15.0822634Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
