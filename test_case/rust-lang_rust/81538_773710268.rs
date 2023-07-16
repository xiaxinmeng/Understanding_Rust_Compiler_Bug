plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
  --> src/librustdoc/formats/renderer.rs:69:60
   |
69 |         .extra_verbose_generic_activity("create_renderer", T::descr())
   |                                                            ^^^^^^^^^^ expected slice, found `str`
   |
   = note: expected reference `&[_]`
              found reference `&'static str`
error[E0308]: mismatched types
   --> src/librustdoc/formats/renderer.rs:109:65
    |
    |
109 |     prof.extra_verbose_generic_activity("renderer_after_krate", T::descr())
    |                                                                 ^^^^^^^^^^ expected slice, found `str`
    |
    = note: expected reference `&[_]`
               found reference `&'static str`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`
