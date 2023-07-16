
[01:40:22] Building stage1 compiler artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
[01:40:22]    Compiling graphviz v0.0.0 (file:///C:/projects/rust/src/libgraphviz)
<snip>
[01:41:43]    Compiling rustc_errors v0.0.0 (file:///C:/projects/rust/src/librustc_errors)
[01:41:49] error: Could not compile `rustc_errors`.
[01:41:49] 
[01:41:49] Caused by:
[01:41:49]   process didn't exit successfully: `C:\projects\rust\build\bootstrap/debug/rustc --crate-name rustc_errors librustc_errors\lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=65687244e9f663b1 -C extra-filename=-65687244e9f663b1 --out-dir C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\deps --extern rustc_data_structures=C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\rustc_data_structures-fe1c5452c3c89758.dll --extern serialize=C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\serialize-e9133ab5aa0e0076.dll --extern serialize=C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libserialize-e9133ab5aa0e0076.rlib --extern unicode_width=C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libunicode_width-850dabec54650530.rlib --extern syntax_pos=C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\syntax_pos-f4ba1a45fc002844.dll` (exit code: 3221225477)
[01:41:49] thread 'main' panicked at 'command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "C:\\projects\\rust\\src/rustc/Cargo.toml" "--message-format" "json"
[01:41:49] expected success, got: exit code: 101', bootstrap\compile.rs:1062:9
[01:41:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:41:49] failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap dist
