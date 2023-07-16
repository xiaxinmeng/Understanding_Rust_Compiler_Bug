`
[RUSTC-TIMING] rls test:false 74.396
    Finished release [optimized] target(s) in 4m 19s
[TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: ["clippy"] } -- 259.688
[TIMING] Rls { stage: 2, target: "x86_64-pc-windows-msvc" } -- 12.375
Dist LlvmTools stage2 (x86_64-pc-windows-msvc)
[TIMING] LlvmTools { stage: 2, target: "x86_64-pc-windows-msvc" } -- 21.311
Dist clippy stage2 (x86_64-pc-windows-msvc)
Building stage2 tool cargo-clippy (x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.74s
[TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", tool: "cargo-clippy", path: "src/tools/clippy", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 0.893
thread 'main' panicked at 'fs::File::open(&src) failed with The system cannot find the file specified. (os error 2)', bootstrap\lib.rs:1291:25
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 1:13:05
Command exited with code 1
set PATH=%PATH%;"C:\Program Files (x86)\Windows Kits\10\Debuggers\X64"
if exist %LOCALAPPDATA%\CrashDumps for %%f in (%LOCALAPPDATA%\CrashDumps\*) do cdb -c "k;q" -G -z "%%f"
