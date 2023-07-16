
Building rustdoc for stage2 (x86_64-pc-windows-msvc)
error: failed to run `rustc` to learn about target-specific information
Caused by:
  process didn't exit successfully: `C:\projects\rust\build\bootstrap/debug/rustc - --crate-name ___ --print=file-names --crate-type bin --crate-type rlib --target x86_64-pc-windows-msvc` (exit code: 3221225477)
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "-j" "2" "--target" "x86_64-pc-windows-msvc" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools\\rustdoc\\Cargo.toml"
