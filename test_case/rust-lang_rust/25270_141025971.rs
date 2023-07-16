
---- [run-pass] run-pass/sepcomp-lib-lto.rs stdout ----

error: compilation failed!
status: exit code: -1073740791
command: PATH="x86_64-pc-windows-msvc/stage2/bin;D:\Sources\Rust\x86_64-pc-windows-msvc\stage2\bin;C:\MSYS2\mingw64\bin;C:\MSYS2\usr\local\bin;C:\MSYS2\usr\bin;C:\MSYS2\usr\bin;C:\Program Files\Python 3;C:\Program Files\Python 3\Scripts;C:\WINDOWS\system32;C:\WINDOWS;C:\WINDOWS\System32\Wbem;C:\WINDOWS\System32\WindowsPowerShell\v1.0;C:\Program Files (x86)\Windows Kits\8.1\Windows Performance Toolkit;C:\Program Files\SlikSvn\bin;C:\Program Files\System Tools;C:\Program Files (x86)\System Tools;C:\Program Files\Vim\vim74;C:\Program Files\Rust\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\MiKTeX\miktex\bin\x64;C:\Program Files (x86)\Pandoc;C:\Program Files\LLVM\bin;C:\Program Files\KDiff3;C:\Program Files\Git\cmd;C:\Users\Vitali\AppData\Local\atom\bin;C:\MSYS2\usr\bin\site_perl;C:\MSYS2\usr\bin\vendor_perl;C:\MSYS2\usr\bin\core_perl" x86_64-pc-windows-msvc/stage2/bin/rustc.exe D:/Sources/Rust/src/test/run-pass/sepcomp-lib-lto.rs -L x86_64-pc-windows-msvc/test/run-pass/ --target=x86_64-pc-windows-msvc -L x86_64-pc-windows-msvc/test/run-pass\sepcomp-lib-lto.stage2-x86_64-pc-windows-msvc.run-pass.libaux -o x86_64-pc-windows-msvc/test/run-pass\sepcomp-lib-lto.stage2-x86_64-pc-windows-msvc.exe -O -L x86_64-pc-windows-msvc/rt -C lto
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
Assertion failed: Ty == resolve(Ty->getRef()) && "type was not uniqued, possible ODR violation.", file D:\Sources\Rust\src\llvm\lib\CodeGen\AsmPrinter\DwarfUnit.cpp, line 713

------------------------------------------

thread '[run-pass] run-pass/sepcomp-lib-lto.rs' panicked at 'explicit panic', D:/Sources/Rust/src/compiletest\runtest.rs:1501
