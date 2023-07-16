
error[E0061]: this function takes 4 parameters but 3 parameters were supplied
   --> src\tools\rls\src\build\rustc.rs:214:12
    |
214 |         ls.register_early_pass(Some(sess), true, pass);
    |            ^^^^^^^^^^^^^^^^^^^ expected 4 parameters
error: aborting due to previous error
For more information about this error, try `rustc --explain E0061`.
[RUSTC-TIMING] rls test:false 3.535
error: Could not compile `rls`.
To learn more, run the command again with --verbose.
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools/rls\\Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json"
expected success, got: exit code: 101
[TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: ["clippy"] } -- 247.135
Unable to build RLS, skipping dist
