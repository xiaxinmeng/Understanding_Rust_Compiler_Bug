plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `crate::html::render::search_index::get_function_type_for_search`
  --> src/librustdoc/formats/cache.rs:15:5
   |
15 | use crate::html::render::search_index::get_function_type_for_search;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `get_function_type_for_search` in `html::render::search_index`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:03
