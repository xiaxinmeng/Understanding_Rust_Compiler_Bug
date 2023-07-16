plain
    Checking clippy_lints v0.1.62 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/undocumented_unsafe_blocks.rs:120:27
    |
120 |             && Rc::ptr_eq(&unsafe_line.sf, &macro_line.sf)
    |
    = note: expected reference `&Rc<_>`
               found reference `&Arc<SourceFile>`


error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/undocumented_unsafe_blocks.rs:120:44
    |
120 |             && Rc::ptr_eq(&unsafe_line.sf, &macro_line.sf)
    |
    = note: expected reference `&Rc<_>`
               found reference `&Arc<SourceFile>`


error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/undocumented_unsafe_blocks.rs:136:23
    |
136 |         && Rc::ptr_eq(&unsafe_line.sf, &body_line.sf)
    |
    = note: expected reference `&Rc<_>`
               found reference `&Arc<SourceFile>`


error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/undocumented_unsafe_blocks.rs:136:40
    |
136 |         && Rc::ptr_eq(&unsafe_line.sf, &body_line.sf)
    |
    = note: expected reference `&Rc<_>`
               found reference `&Arc<SourceFile>`

