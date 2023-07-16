
Building stage1 std artifacts (i686-pc-windows-msvc -> i686-pc-windows-msvc)
   Compiling libc v0.0.0 (file:///C:/projects/rust/src/rustc/libc_shim)
   Compiling gcc v0.3.51
   Compiling unwind v0.0.0 (file:///C:/projects/rust/src/libunwind)
   Compiling core v0.0.0 (file:///C:/projects/rust/src/libcore)
   Compiling libc v0.2.30
   Compiling filetime v0.1.10
   Compiling build_helper v0.1.0 (file:///C:/projects/rust/src/build_helper)
   Compiling std v0.0.0 (file:///C:/projects/rust/src/libstd)
   Compiling compiler_builtins v0.0.0 (file:///C:/projects/rust/src/rustc/compiler_builtins_shim)
   Compiling std_unicode v0.0.0 (file:///C:/projects/rust/src/libstd_unicode)
   Compiling rand v0.0.0 (file:///C:/projects/rust/src/librand)
   Compiling panic_abort v0.0.0 (file:///C:/projects/rust/src/libpanic_abort)
   Compiling alloc v0.0.0 (file:///C:/projects/rust/src/liballoc)
   Compiling collections v0.0.0 (file:///C:/projects/rust/src/libcollections)
   Compiling alloc_system v0.0.0 (file:///C:/projects/rust/src/liballoc_system)
   Compiling panic_unwind v0.0.0 (file:///C:/projects/rust/src/libpanic_unwind)
error: Could not compile `std`.
Caused by:
  failed to parse process output: `C:\projects\rust\build\bootstrap/debug/rustc --crate-name std src\libstd\lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="backtrace" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=22fc13253edadc9b -C extra-filename=-22fc13253edadc9b --out-dir C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps --target i686-pc-windows-msvc -L dependency=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps -L dependency=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\release\deps --extern libc=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\liblibc-57f9cc46233141d9.rlib --extern compiler_builtins=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\libcompiler_builtins-af47b91f5665c739.rlib --extern alloc=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\liballoc-8edc0052224e78e2.rlib --extern panic_unwind=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\libpanic_unwind-f30e8935180a24f8.rlib --extern rand=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\librand-1dd743f4fd629578.rlib --extern collections=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\libcollections-fab767d670e933ef.rlib --extern alloc_system=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\liballoc_system-adc2cf430de98b54.rlib --extern std_unicode=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\libstd_unicode-77636529a2640872.rlib --extern panic_abort=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\libpanic_abort-ba285cc200027a78.rlib --extern unwind=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\libunwind-dea68fc02821d416.rlib --extern core=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\deps\libcore-4b30724c9b3622b8.rlib -l advapi32 -l ws2_32 -l userenv -l shell32 -L native=C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\atlmfc\lib\ -L native=C:\projects\rust\build\i686-pc-windows-msvc\stage1-std\i686-pc-windows-msvc\release\build\compiler_builtins-9759ac0b288d4496\out` (exit code: 0)
To learn more, run the command again with --verbose.
thread 'main' panicked at 'command did not execute successfully: "C:\\projects\\rust\\build\\i686-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "-j" "4" "--target" "i686-pc-windows-msvc" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "C:\\projects\\rust\\src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', src\bootstrap\compile.rs:883:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap test --host i686-pc-windows-msvc --target i686-pc-windows-msvc
Build completed unsuccessfully in 0:18:46
