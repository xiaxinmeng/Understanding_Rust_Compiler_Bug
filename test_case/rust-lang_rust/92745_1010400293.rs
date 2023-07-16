plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
    --> src/librustdoc/html/render/mod.rs:1678:62
     |
1678 |         Some(ref t) => get_id_for_impl(&i.inner_impl().for_, t, cx),
     |                                                              ^ expected enum `std::option::Option`, found `&types::Path`
     |
     = note:   expected enum `std::option::Option<&types::Path>`
             found reference `&types::Path`
help: try wrapping the expression in `doctest::_::_serde::__private::Some`
     |
1678 |         Some(ref t) => get_id_for_impl(&i.inner_impl().for_, doctest::_::_serde::__private::Some(t), cx),
     |                                                              ++++++++++++++++++++++++++++++++++++ +
error[E0308]: mismatched types
    --> src/librustdoc/html/render/mod.rs:2199:78
     |
     |
2199 |                 (format!("{:#}", i.for_.print(cx)), get_id_for_impl(&i.for_, trait_, cx))
     |                                                                              ^^^^^^ expected enum `std::option::Option`, found `&types::Path`
     |
     = note:   expected enum `std::option::Option<&types::Path>`
             found reference `&types::Path`
help: try wrapping the expression in `doctest::_::_serde::__private::Some`
     |
2199 |                 (format!("{:#}", i.for_.print(cx)), get_id_for_impl(&i.for_, doctest::_::_serde::__private::Some(trait_), cx))
     |                                                                              ++++++++++++++++++++++++++++++++++++      +
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:02:59
