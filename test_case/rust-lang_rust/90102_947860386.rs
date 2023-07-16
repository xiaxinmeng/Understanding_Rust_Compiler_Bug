plain
   Compiling clippy v0.1.57 (/checkout/src/tools/clippy)
    Checking bstr v0.2.13
    Checking quote v1.0.7
   Compiling libz-sys v1.1.3
error[E0599]: no variant or associated item named `Box` found for enum `NullOp` in the current scope
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:196:35
    |
196 |         Rvalue::NullaryOp(NullOp::Box, _) => Err((span, "heap allocations are not allowed in const fn".into())),
    |                                   ^^^ variant or associated item not found in `NullOp`
    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
    Checking num_cpus v1.13.0
    Checking filetime v0.2.14
