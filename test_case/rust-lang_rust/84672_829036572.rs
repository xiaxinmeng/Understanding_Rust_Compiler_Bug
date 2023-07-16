plain

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'Unable to build miri', src/bootstrap/dist.rs:44:9
[TIMING] ToolBuild { compiler: Compiler { stage: 1, host: TargetSelection { triple: "i686-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "i686-unknown-linux-gnu", file: None }, tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 2.726
[TIMING] Miri { compiler: Compiler { stage: 1, host: TargetSelection { triple: "i686-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "i686-unknown-linux-gnu", file: None }, extra_features: [] } -- 0.000
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build i686-unknown-linux-gnu --host i686-unknown-linux-gnu --target i686-unknown-linux-gnu
Build completed unsuccessfully in 0:32:02
