plain
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 0, host: aarch64-unknown-linux-gnu }, target: aarch64-unknown-linux-gnu, tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 21.721
[TIMING] tool::Tidy { compiler: Compiler { stage: 0, host: aarch64-unknown-linux-gnu }, target: aarch64-unknown-linux-gnu } -- 0.000
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: following path contains more than 2147 entries, you should move the test to some relevant subdirectory (current: 2148): /checkout/src/test/ui/issues
* highest error code: E0790
Checking which error codes lack tests...
Found 506 error codes
Found 0 error(s) in error codes
