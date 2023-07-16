
src/libstd/sys/windows/c.rs:1090:5: 1093:58 warning: foreign function is never used: `RaiseException`, #[warn(dead_code)] on by default
src/libstd/sys/windows/c.rs:1090     pub fn RaiseException(dwExceptionCode: DWORD,
src/libstd/sys/windows/c.rs:1091                           dwExceptionFlags: DWORD,
src/libstd/sys/windows/c.rs:1092                           nNumberOfArguments: DWORD,
src/libstd/sys/windows/c.rs:1093                           lpArguments: *const ULONG_PTR);
