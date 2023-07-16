plain
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unexpected closing delimiter: `}`
   --> src/librustdoc/clean/blanket_impl.rs:126:9
    |
74  |                                 Err(traits::OverflowError) => {}
    |                                                               -- block is empty, you might have not meant to close it
126 |         }
    |         ^ unexpected closing delimiter

error: mismatched closing delimiter: `)`
error: mismatched closing delimiter: `)`
   --> src/librustdoc/clean/blanket_impl.rs:101:75
    |
100 |                     kind: Box::new(ImplItem(Impl {
    |                                                  - unclosed delimiter
101 |                         span: Span::new(self.cx.tcx.def_span(impl_def_id))),

error: mismatched closing delimiter: `}`
error: mismatched closing delimiter: `}`
   --> src/librustdoc/clean/blanket_impl.rs:122:21
    |
100 |                     kind: Box::new(ImplItem(Impl {
...
122 |                     })),
    |                     ^ mismatched closing delimiter


error: mismatched closing delimiter: `)`
   --> src/librustdoc/clean/blanket_impl.rs:122:23
    |
30  |             for &impl_def_id in trait_impls.blanket_impls() {
...
122 |                     })),
    |                       ^ mismatched closing delimiter


error: mismatched closing delimiter: `)`
   --> src/librustdoc/clean/blanket_impl.rs:124:18
    |
16  |     crate fn get_blanket_impls(&mut self, item_def_id: DefId) -> Vec<Item> {
...
124 |                 });
    |                  ^ mismatched closing delimiter

