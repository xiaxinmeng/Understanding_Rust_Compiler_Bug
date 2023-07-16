
---- [compile-fail] compile-fail\svh-change-type-static.rs stdout ----

error: C:/msys64/home/we/rust/src/test/compile-fail/svh-change-type-static.rs:19: unexpected "note": '19:1: 19:16: crate `a` path #1: \\?\C:\msys64\home\we\rust\build0\x86_64-pc-windows-gnu\stage1\lib\rustlib\x86_64-pc-windows-gnu\lib\libarena-fdb5dc8c.rlib compiled by "rustc 1.15.0-dev (dcad4c53f 2016-12-16)"'

error: C:/msys64/home/we/rust/src/test/compile-fail/svh-change-type-static.rs:19: unexpected "note": '19:1: 19:16: crate `a` path #2: \\?\C:\msys64\home\we\rust\build0\x86_64-pc-windows-gnu\stage1\lib\rustlib\x86_64-pc-windows-gnu\lib\arena-fdb5dc8c.dll compiled by "rustc 1.15.0-dev (dcad4c53f 2016-12-16)"'

error: 2 unexpected errors found, 0 expected errors not found
status: exit code: 101
command: PATH="blahblahblah" x86_64-pc-windows-gnu/stage1/bin/rustc.exe C:/msys64/home/we/rust/src/test/compile-fail/svh-change-type-static.rs -L x86_64-pc-windows-gnu/test/compile-fail/ --target=x86_64-pc-windows-gnu --error-format json -L x86_64-pc-windows-gnu/test/compile-fail\svh-change-type-static.stage1-x86_64-pc-windows-gnu.compile-fail.libaux -C prefer-dynamic -o x86_64-pc-windows-gnu/test/compile-fail\svh-change-type-static.stage1-x86_64-pc-windows-gnu.exe --cfg rtopt -C rpath -O -L x86_64-pc-windows-gnu/rt
unexpected errors (from JSON output): [
    Error {
        line_num: 19,
        kind: Some(
            Note
        ),
        msg: "19:1: 19:16: crate `a` path #1: \\\\?\\C:\\msys64\\home\\we\\rust\\build0\\x86_64-pc-windows-gnu\\stage1\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libarena-fdb5dc8c.rlib compiled by \"rustc 1.15.0-dev (dcad4c53f 2016-12-16)\""
    },
    Error {
        line_num: 19,
        kind: Some(
            Note
        ),
        msg: "19:1: 19:16: crate `a` path #2: \\\\?\\C:\\msys64\\home\\we\\rust\\build0\\x86_64-pc-windows-gnu\\stage1\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\arena-fdb5dc8c.dll compiled by \"rustc 1.15.0-dev (dcad4c53f 2016-12-16)\""
    }
]

thread '[compile-fail] compile-fail\svh-change-type-static.rs' panicked at 'explicit panic', C:/msys64/home/we/rust/src/tools/compiletest/src\runtest.rs:1114
