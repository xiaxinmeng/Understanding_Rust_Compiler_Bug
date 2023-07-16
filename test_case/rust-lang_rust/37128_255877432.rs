
---- [compile-fail] compile-fail\feature-gate-no-debug-2.rs stdout ----

error: C:/bot/slave/auto-win-gnu-32-opt-rustbuild/build/src/test/compile-fail/feature-gate-no-debug-2.rs:14: unexpected "error": '14:1: 14:12: use of deprecated attribute `no_debug`: the `#[no_debug]` attribute is an experimental feature. See https://github.com/rust-lang/rust/issues/29721'

error: C:/bot/slave/auto-win-gnu-32-opt-rustbuild/build/src/test/compile-fail/feature-gate-no-debug-2.rs:14: expected error not found: use of deprecated attribute: no_debug

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: PATH="C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2\bin;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-tools\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-rustc\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-test\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-std\i686-pc-windows-gnu\release\deps;C:\mingw-w64\i686-4.9.1-win32-dwarf-rt_v3-rev1\mingw32\bin;C:\Python27;C:\msys64\mingw32\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin;C:\Program Files (x86)\Windows Kits\8.1\Windows Performance Toolkit;C:\Program Files\Microsoft SQL Server\110\Tools\Binn;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.0;C:\Program Files\CMake\bin" C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2\bin\rustc.exe C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\compile-fail\feature-gate-no-debug-2.rs -L C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\compile-fail --target=i686-pc-windows-gnu --error-format json -L C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\compile-fail\feature-gate-no-debug-2.stage2-i686-pc-windows-gnu.compile-fail.libaux -C prefer-dynamic -o C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\compile-fail\feature-gate-no-debug-2.stage2-i686-pc-windows-gnu.exe -Crpath -O -Lnative=C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 14,
        kind: Some(
            Error
        ),
        msg: "14:1: 14:12: use of deprecated attribute `no_debug`: the `#[no_debug]` attribute is an experimental feature. See https://github.com/rust-lang/rust/issues/29721"
    }
]

not found errors (from test file): [
    Error {
        line_num: 14,
        kind: Some(
            Error
        ),
        msg: "use of deprecated attribute: no_debug"
    }
]

thread '[compile-fail] compile-fail\feature-gate-no-debug-2.rs' panicked at 'explicit panic', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\tools\compiletest\src\runtest.rs:1097
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [compile-fail] compile-fail\feature-gate-no-debug-2.rs
