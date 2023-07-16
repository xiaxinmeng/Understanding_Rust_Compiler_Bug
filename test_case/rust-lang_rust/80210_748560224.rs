plain
[RUSTC-TIMING] lint_docs test:false 0.457
    Finished release [optimized] target(s) in 8.03s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "lint-docs", path: "src/tools/lint-docs", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 8.047
[TIMING] LintDocs { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.000
error: failed to test example in lint docs for `missing_fragment_specifier` in /checkout/compiler/rustc_lint_defs/src/builtin.rs:1231: lint docs should contain the line `### Example`


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/lint-docs" "--src" "/checkout/compiler" "--out" "/checkout/obj/build/aarch64-unknown-linux-gnu/md-doc/rustc/src/lints" "--rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustc-target" "aarch64-unknown-linux-gnu"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu
Build completed unsuccessfully in 0:18:20
