plain
---- html/render/mod.rs - html::render::get_filtered_impls_for_reference (line 2361) stdout ----
error[E0405]: cannot find trait `Trait` in this scope
 --> html/render/mod.rs:2362:9
  |
3 | impl<T> Trait for &T {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0405`.
