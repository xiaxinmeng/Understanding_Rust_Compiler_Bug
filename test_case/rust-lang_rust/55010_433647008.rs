
[00:24:14] [RUSTC-TIMING] core test:false 42.082
[00:24:14] error: Could not compile `core`.
[00:24:14] 
[00:24:14] To learn more, run the command again with --verbose.
[00:24:14] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "C:\\projects\\rust\\src/libstd/Cargo.toml" "--message-format" "json"
[00:24:14] expected success, got: exit code: 101
[00:24:14] thread 'main' panicked at 'cargo must succeed', bootstrap\compile.rs:1115:9
[00:24:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:24:14] failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap dist
[00:24:14] Build completed unsuccessfully in 0:18:25
[00:24:14] Command exited with code 1
[00:24:14] set PATH=%PATH%;"C:\Program Files (x86)\Windows Kits\10\Debuggers\X64"
[00:24:14] if exist %LOCALAPPDATA%\CrashDumps for %%f in (%LOCALAPPDATA%\CrashDumps\*) do cdb -c "k;q" -G -z "%%f"
