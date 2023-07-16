bash
~/test_rustdoc_json/bar$ cargo rustdoc -- -Z unstable-options --output-format json
 Documenting bar v0.1.0 (/home/kpp/test_rustdoc_json/bar)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Item { id: Id("20:3"), crate_id: 20, name: Some("Foo"), span: Some(Span { filename: "/home/kpp/test_rustdoc_json/foo/src/lib.rs", begin: (1, 0), end: (1, 15) }), visibility: Public, docs: None, links: {}, attrs: [], deprecation: None, inner: Struct(Struct { struct_type: Unit, generics: Generics { params: [], where_predicates: [] }, fields_stripped: false, fields: [], impls: [] }) }`,
 right: `Item { id: Id("20:3"), crate_id: 20, name: Some("Foo"), span: Some(Span { filename: "/home/kpp/test_rustdoc_json/foo/src/lib.rs", begin: (1, 0), end: (1, 15) }), visibility: Public, docs: Some("Bar"), links: {}, attrs: [], deprecation: None, inner: Struct(Struct { struct_type: Unit, generics: Generics { params: [], where_predicates: [] }, fields_stripped: false, fields: [], impls: [] }) }`', src/librustdoc/json/mod.rs:179:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'

error: could not document `bar`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2021 --crate-type lib --crate-name bar bar/src/lib.rs -o /home/kpp/test_rustdoc_json/target/doc --error-format=json --json=diagnostic-rendered-ansi -Z unstable-options --output-format json -L dependency=/home/kpp/test_rustdoc_json/target/debug/deps --extern foo=/home/kpp/test_rustdoc_json/target/debug/deps/libfoo-234c6169b2e7c94b.rmeta --crate-version 0.1.0` (exit status: 1)
