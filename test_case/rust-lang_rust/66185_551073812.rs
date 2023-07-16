plain
2019-11-07T13:12:47.6869864Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2019-11-07T13:12:49.5289182Z [RUSTC-TIMING] linkchecker test:false 1.840
2019-11-07T13:12:49.5300373Z     Finished release [optimized] target(s) in 1.97s
2019-11-07T13:12:49.5355400Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "linkchecker", path: "src/tools/linkchecker", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 1.986
2019-11-07T13:12:54.5755492Z rustc/command-line-arguments.html:164: id is not unique: `option-l-search-path`
2019-11-07T13:12:54.5770117Z rustc/print.html:212: id is not unique: `option-l-search-path`
2019-11-07T13:12:56.3229972Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
2019-11-07T13:12:56.3259766Z 
2019-11-07T13:12:56.3259902Z 
2019-11-07T13:12:56.3260448Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2019-11-07T13:12:56.3260578Z expected success, got: exit code: 101
---
2019-11-07T13:12:56.3318398Z   local time: Thu Nov  7 13:12:56 UTC 2019
2019-11-07T13:12:56.5968169Z   network time: Thu, 07 Nov 2019 13:12:56 GMT
2019-11-07T13:12:56.5972463Z == end clock drift check ==
2019-11-07T13:13:01.8842474Z 
2019-11-07T13:13:01.8977190Z ##[error]Bash exited with code '1'.
2019-11-07T13:13:01.9010789Z ##[section]Starting: Checkout
2019-11-07T13:13:01.9012686Z ==============================================================================
2019-11-07T13:13:01.9012783Z Task         : Get sources
2019-11-07T13:13:01.9012852Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
