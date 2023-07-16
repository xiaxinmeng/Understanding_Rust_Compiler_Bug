plain
    Checking unicode-normalization v0.1.13
    Checking semver-parser v0.10.2
   Compiling clippy v0.1.57 (/checkout/src/tools/clippy)
    Checking bstr v0.2.13
error[E0532]: expected tuple struct or tuple variant, found unit variant `UpvarCapture::ByValue`
    |
    |
958 | ...                   UpvarCapture::ByValue(_) => CaptureKind::Value,
    |
   ::: /checkout/compiler/rustc_middle/src/ty/closure.rs:59:5
    |
59  |     ByValue,
59  |     ByValue,
    |     ------- `UpvarCapture::ByValue` defined here
help: use this syntax instead
    |
    |
958 |                                 UpvarCapture::ByValue => CaptureKind::Value,
help: consider importing this tuple variant instead
    |
58  | use rustc_ast::BindingMode::ByValue;
    |
---
    Checking dirs-next v2.0.0
    Checking term v0.7.0
    Checking rand_core v0.6.2
    Checking rand_core v0.5.1
error[E0609]: no field `kind` on type `rustc_middle::ty::BorrowKind`
    |
    |
959 | ...                   UpvarCapture::ByRef(borrow) => match borrow.kind {

Some errors have detailed explanations: E0532, E0609.
For more information about an error, try `rustc --explain E0532`.
error: could not compile `clippy_utils` due to 2 previous errors
