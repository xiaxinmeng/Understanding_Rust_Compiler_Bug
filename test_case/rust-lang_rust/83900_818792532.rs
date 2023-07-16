plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0422]: cannot find struct, variant or union type `Item` in this scope
   --> src/librustdoc/html/render/print_item.rs:268:39
    |
268 |                     let import_item = Item { def_id, ..myitem.clone() };
    |
help: consider importing one of these items
    |
1   | use crate::clean::Item;
1   | use crate::clean::Item;
    |
1   | use crate::html::render::pprust::AnnNode::Item;
    |
1   | use pulldown_cmark::Tag::Item;
1   | use rustc_ast::Item;
    |
      and 11 other candidates

