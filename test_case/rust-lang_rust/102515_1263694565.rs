plain
  |
2 | #![feature(map_first_last)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `-D stable-features` implied by `-D warnings`
[RUSTC-TIMING] miri test:false 7.495
error: could not compile `miri` due to previous error
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: InTree, extra_features: [] } -- 17.660
[TIMING] tool::Miri { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, extra_features: [] } -- 0.000
[TIMING] tool::Miri { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, extra_features: [] } -- 0.000
thread 'main' panicked at 'in-tree tool', test.rs:489:14
Build completed unsuccessfully in 0:00:19
