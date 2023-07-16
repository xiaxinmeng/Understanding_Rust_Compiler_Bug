
error[E0599]: no method named `stable_since` found for struct `types::Impl` in the current scope
    --> src/librustdoc/html/render/print_item.rs:613:43
     |
613  |                     implementor.impl_item.stable_since(cx.tcx()).as_deref(),
     |                                           ^^^^^^^^^^^^ method not found in `types::Impl`
     | 
    ::: src/librustdoc/clean/types.rs:2034:1
     |
2034 | crate struct Impl {
     | ----------------- method `stable_since` not found for this

error[E0599]: no method named `const_stable_since` found for struct `types::Impl` in the current scope
    --> src/librustdoc/html/render/print_item.rs:614:43
     |
614  |                     implementor.impl_item.const_stable_since(cx.tcx()).as_deref(),
     |                                           ^^^^^^^^^^^^^^^^^^ method not found in `types::Impl`
