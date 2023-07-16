
failures:

---- clean::clean_git stdout ----
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe clean -p dep`
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
thread 'clean::clean_git' panicked at '
Expected: execs
    but: exited with exit code: 101
--- stdout

--- stderr
   Compiling dep v0.5.0 (file:///D:/a/rust/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t478/dep#800dc269)
   Compiling foo v0.0.1 (D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t478\foo)
error: linking with `link.exe` failed: exit code: 0xc0000374
  |
