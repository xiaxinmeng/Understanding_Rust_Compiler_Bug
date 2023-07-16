
2020-08-21T08:54:27.4115722Z [0m[1m[38;5;10mnote[0m[0m[1m[38;5;15m: `link.exe` returned an unexpected error[0m
2020-08-21T08:54:27.4116018Z 
2020-08-21T08:54:27.4116240Z [0m[1m[38;5;10mnote[0m[0m[1m[38;5;15m: the Visual Studio build tools may need to be repaired using the Visual Studio installer[0m
2020-08-21T08:54:27.4116388Z 
2020-08-21T08:54:27.4116584Z [0m[1m[38;5;10mnote[0m[0m[1m[38;5;15m: or a necessary component may be missing from the "C++ build tools" workload[0m
2020-08-21T08:54:27.4116722Z 
2020-08-21T08:54:27.4128039Z [0m[1m[38;5;9merror[0m[0m[1m[38;5;15m: aborting due to previous error[0m
2020-08-21T08:54:27.4128217Z 
2020-08-21T08:54:27.4220384Z [RUSTC-TIMING] build_script_build test:false 2.622
2020-08-21T08:54:27.4233457Z [0m[0m[1m[31merror[0m[1m:[0m could not compile `stacker`.
2020-08-21T08:54:27.4233664Z 
2020-08-21T08:54:27.4233931Z To learn more, run the command again with --verbose.
2020-08-21T08:54:27.4234190Z [0m[0m[1m[33mwarning[0m[1m:[0m build failed, waiting for other jobs to finish...
2020-08-21T08:54:28.2375366Z [RUSTC-TIMING] build_script_build test:false 3.434
2020-08-21T08:54:31.5274931Z [0m[0m[1m[31merror[0m[1m:[0m build failed
2020-08-21T08:54:31.5345517Z command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\rust\\rust\\src/tools/rustfmt\\Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
2020-08-21T08:54:31.5346495Z expected success, got: exit code: 101
2020-08-21T08:54:31.5368358Z [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None }, tool: "rustfmt", path: "src/tools/rustfmt", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 15.722
2020-08-21T08:54:31.5368933Z failed to test rustfmt: could not build
