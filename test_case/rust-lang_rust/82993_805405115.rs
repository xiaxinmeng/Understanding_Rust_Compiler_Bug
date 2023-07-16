plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0609]: no field `source` on type `types::Item`
  --> src/librustdoc/html/sources.rs:58:30
   |
58 |                         item.source.span(),
   |
   |
   = note: available fields are: `span`, `name`, `attrs`, `visibility`, `kind`, `def_id`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustdoc`
