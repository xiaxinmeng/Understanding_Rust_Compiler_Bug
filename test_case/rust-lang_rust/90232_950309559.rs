plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0433]: failed to resolve: could not find `noto_sans_kr` in `static_files`
  --> src/librustdoc/html/render/write_shared.rs:42:55
   |
42 |         "noto-sans-kr-regular.woff2" => static_files::noto_sans_kr::REGULAR2,
   |                                                       ^^^^^^^^^^^^ could not find `noto_sans_kr` in `static_files`

error[E0433]: failed to resolve: could not find `noto_sans_kr` in `static_files`
  --> src/librustdoc/html/render/write_shared.rs:43:54
   |
43 |         "noto-sans-kr-regular.woff" => static_files::noto_sans_kr::REGULAR,
   |                                                      ^^^^^^^^^^^^ could not find `noto_sans_kr` in `static_files`

error[E0433]: failed to resolve: could not find `noto_sans_kr` in `static_files`
  --> src/librustdoc/html/render/write_shared.rs:44:53
   |
44 |         "noto-sans-kr-LICENSE.txt" => static_files::noto_sans_kr::LICENSE,
   |                                                     ^^^^^^^^^^^^ could not find `noto_sans_kr` in `static_files`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustdoc` due to 3 previous errors
Build completed unsuccessfully in 0:03:04
