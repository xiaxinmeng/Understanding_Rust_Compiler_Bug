
$ rustdoc +stage1 src/test/rustdoc/intra-doc-link-enum-struct-field.rs
warning: unresolved link to `Foo::X::y`
  --> src/test/rustdoc/intra-doc-link-enum-struct-field.rs:11:13
   |
11 | /// I want [Foo::X::y].
   |             ^^^^^^^^^ the enum `Foo` has no variant or associated item named `y`
