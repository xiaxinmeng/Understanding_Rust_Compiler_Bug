plain
[01:52:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-netbsd" "-Zdual-proc-macros" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:52:59] expected success, got: exit code: 101
[01:52:59] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-netbsd", tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 18.310
[01:52:59] Unable to build miri, skipping dist
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
