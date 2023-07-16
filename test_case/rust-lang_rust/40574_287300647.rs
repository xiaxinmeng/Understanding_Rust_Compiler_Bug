rust
#[derive(MyProcMacro)]
struct S(#[cfg(any())] i32);
//^ The input to `MyProcMacro` here is `struct S();`
