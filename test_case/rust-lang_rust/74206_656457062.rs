
2020-07-10T03:07:16.5904761Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "expand-yaml-anchors", path: "src/tools/expand-yaml-anchors", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 10.855
2020-07-10T03:07:16.5952981Z error: .github/workflows/ci.yml is not up to date
2020-07-10T03:07:16.5953142Z 
2020-07-10T03:07:16.5953882Z caused by: src/ci/github-actions/ci.yml and .github/workflows/ci.yml are different
2020-07-10T03:07:16.5954012Z 
2020-07-10T03:07:16.5954565Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/expand-yaml-anchors" "check" "/checkout"
2020-07-10T03:07:16.5954795Z expected success, got: exit code: 1
2020-07-10T03:07:16.5954910Z 
2020-07-10T03:07:16.5955004Z 
2020-07-10T03:07:16.5959361Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/expand-yaml-anchors

