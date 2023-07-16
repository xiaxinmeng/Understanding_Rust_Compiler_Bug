
[01:56:11] [RUSTC-TIMING] rustfmt_nightly test:false 59.350
[01:56:15] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:56:15]    --> src\tools\rls\src\build\rustc.rs:214:12
[01:56:15]     |
[01:56:15] 214 |         ls.register_early_pass(Some(sess), true, pass);
[01:56:15]     |            ^^^^^^^^^^^^^^^^^^^ expected 4 parameters
[01:56:15] 
[01:56:15] error: aborting due to previous error
[01:56:15] 
[01:56:15] For more information about this error, try `rustc --explain E0061`.
[01:56:15] [RUSTC-TIMING] rls test:false 4.517
[01:56:16] error: Could not compile `rls`.
[01:56:16] 
[01:56:16] To learn more, run the command again with --verbose.
[01:56:16] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools/rls\\Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json"
[01:56:16] expected success, got: exit code: 101
[01:56:16] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: ["clippy"] } -- 310.115
[01:56:16] Unable to build RLS, skipping dist
