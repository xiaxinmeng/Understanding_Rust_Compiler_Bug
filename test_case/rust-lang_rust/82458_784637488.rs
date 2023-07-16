plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/types.rs:175:34
    |
175 |             source: source.clean(cx),
    |                                  ^^ types differ in mutability
    |
    = note: expected mutable reference `&mut DocContext<'_>`
                       found reference `&DocContext<'_>`
error[E0308]: mismatched types
   --> src/librustdoc/clean/types.rs:177:57
    |
    |
177 |             visibility: cx.tcx.visibility(def_id).clean(cx),
    |                                                         ^^ types differ in mutability
    |
    = note: expected mutable reference `&mut DocContext<'_>`
                       found reference `&DocContext<'_>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`
