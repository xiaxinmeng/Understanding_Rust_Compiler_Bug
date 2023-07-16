plain
[RUSTC-TIMING] expand_yaml_anchors test:false 0.738
    Finished release [optimized] target(s) in 5.13s
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, tool: "expand-yaml-anchors", path: "src/tools/expand-yaml-anchors", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 5.155
[TIMING] tool::ExpandYamlAnchors { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
error: .github/workflows/ci.yml is not up to date; please run `x.py run src/tools/expand-yaml-anchors`.
caused by: src/ci/github-actions/ci.yml and .github/workflows/ci.yml are different
