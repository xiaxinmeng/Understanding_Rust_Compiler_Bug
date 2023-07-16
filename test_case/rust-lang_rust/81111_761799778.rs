plain
    Finished release [optimized] target(s) in 21.53s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 21.548
[TIMING] Tidy { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.000
tidy check
tidy error: following path contains more than 2669 entries, you should move the test to some relevant subdirectory (current: 2671): /checkout/src/test/ui/issues
Found 435 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
