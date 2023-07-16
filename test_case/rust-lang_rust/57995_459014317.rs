
testing https://github.com/servo/webrender
Initialized empty Git repository in /c/projects/rust/build/ct/webrender/.git/
fatal: Could not parse object '57250b2b8fa63934f80e5376a29f7dcb3f759ad6'.
fatal: unable to access 'https://github.com/servo/webrender/': Failed to connect to github.com port 443: Connection timed out
thread 'main' panicked at 'assertion failed: status.success()', src\tools\cargotest\main.rs:114:13
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\cargotest.exe" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "C:\\projects\\rust\\build\\ct"
expected success, got: exit code: 101
failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap test src/tools/cargotest src/tools/cargo
Build completed unsuccessfully in 1:12:55
script exited with 1
Command exited with code 1
