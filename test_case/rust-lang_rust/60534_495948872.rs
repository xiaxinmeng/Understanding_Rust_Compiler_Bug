
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] racer test:false 29.976=============================> ] 260/264: racer                                         
error: build failed
command did not execute successfully: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "12" "-v" "--release" "--manifest-path" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rls/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
[TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 30.273
      < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] }
    < Rls { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
thread 'main' panicked at 'Unable to build RLS', src/bootstrap/dist.rs:65:9
