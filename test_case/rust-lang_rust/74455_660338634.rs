
2020-07-17T20:40:36.9472277Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 23.054
2020-07-17T20:40:36.9474901Z tidy check
2020-07-17T20:40:38.0640507Z tidy error: duplicate error code: 767
2020-07-17T20:40:38.0641721Z tidy error: /checkout/src/librustc_error_codes/error_codes.rs:452: E0767: include_str!("./error_codes/E0767.md"),
2020-07-17T20:40:38.0643363Z tidy error: /checkout/src/librustc_error_codes/error_codes.rs:635:     E0767, // `'static' obligation coming from `impl dyn Trait {}` or `impl Foo for dyn Bar {}`.
2020-07-17T20:40:41.1402204Z Checking which error codes lack tests...
2020-07-17T20:40:41.3199217Z some tidy checks failed
2020-07-17T20:40:41.3199568Z Found 509 error codes
2020-07-17T20:40:41.3199840Z Found 0 error codes with no tests\
