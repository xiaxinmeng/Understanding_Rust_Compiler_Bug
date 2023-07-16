plain
    Checking idna v0.2.0
   Compiling tokio v1.8.4
   Compiling libnghttp2-sys v0.1.4+1.41.0
   Compiling libz-sys v1.1.3
error[E0277]: `&&[rustc_span::symbol::Ident]` is not an iterator
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:715:30
715 |                 for field in fields {
715 |                 for field in fields {
    |                              ^^^^^^ `&&[rustc_span::symbol::Ident]` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `&&[rustc_span::symbol::Ident]`
    = help: the following other types implement trait `IntoIterator`:
              &'a [T; N]
              &'a [T]
              &'a mut [T; N]
              &'a mut [T]
              [T; N]
    = note: required for `&&[rustc_span::symbol::Ident]` to implement `IntoIterator`
   Compiling curl-sys v0.4.59+curl-7.86.0
   Compiling compiletest_rs v0.9.0
    Checking textwrap v0.15.0
    Checking textwrap v0.15.0
error[E0599]: no variant or associated item named `OffsetOf` found for enum `Rvalue` in the current scope
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:208:17
    |
208 |         Rvalue::OffsetOf(_, _) => Ok(()),
    |                 ^^^^^^^^ variant or associated item not found in `Rvalue<'_>`
    Checking regex-automata v0.1.10
    Checking rand v0.8.5
    Checking clap_lex v0.3.0
    Checking bytes v1.0.1
