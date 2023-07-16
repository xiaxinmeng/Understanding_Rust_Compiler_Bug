plain
2020-03-18T18:39:39.0976656Z [RUSTC-TIMING] tidy test:false 0.715
2020-03-18T18:39:39.1040467Z     Finished release [optimized] target(s) in 1m 40s
2020-03-18T18:39:39.1088269Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-unknown-linux-gnu" }, target: "i686-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 100.917
2020-03-18T18:39:39.1089074Z tidy check
2020-03-18T18:39:39.8705603Z tidy error: /checkout/src/ci/scripts/install-msys2-packages.sh:16: line longer than 100 chars
2020-03-18T18:39:43.0141977Z Found 489 error codes
2020-03-18T18:39:43.0142779Z Found 0 error codes with no tests
2020-03-18T18:39:43.0143193Z Done!
2020-03-18T18:39:43.0143535Z some tidy checks failed
2020-03-18T18:39:43.0143535Z some tidy checks failed
2020-03-18T18:39:43.0143965Z 
2020-03-18T18:39:43.0144241Z 
2020-03-18T18:39:43.0145824Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-18T18:39:43.0147110Z 
2020-03-18T18:39:43.0147353Z 
2020-03-18T18:39:43.0148498Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/bootstrap --exclude src/test/rustdoc-js --exclude src/tools/error_index_generator --exclude src/tools/linkchecker
2020-03-18T18:39:43.0149346Z Build completed unsuccessfully in 0:01:54
2020-03-18T18:39:43.0149346Z Build completed unsuccessfully in 0:01:54
2020-03-18T18:39:43.0149805Z == clock drift check ==
2020-03-18T18:39:43.0150261Z   local time: Wed Mar 18 18:39:42 UTC 2020
2020-03-18T18:39:43.0150783Z   network time: Wed, 18 Mar 2020 18:39:42 GMT
2020-03-18T18:39:43.2714926Z == end clock drift check ==
2020-03-18T18:39:43.2958938Z 
2020-03-18T18:39:43.3002923Z ##[error]Bash exited with code '1'.
2020-03-18T18:39:43.3078269Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-18T18:39:43.3083777Z ==============================================================================
2020-03-18T18:39:43.3084239Z Task         : Get sources
2020-03-18T18:39:43.3084688Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
