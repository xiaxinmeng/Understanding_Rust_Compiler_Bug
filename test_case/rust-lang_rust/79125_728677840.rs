
error[E0308]: mismatched types
    --> src/librustdoc/html/render/mod.rs:1771:92
     |
1769 |     match item.kind {
     |           --------- this expression has type `types::ItemKind`
1770 |         clean::ModuleItem(ref m) => item_module(buf, cx, item, &m.items),
1771 |         clean::FunctionItem(ref f) | clean::ForeignFunctionItem(ref f) | clean::MethodItem(ref f) | clean::TyMethodItem(ref f) => {
     |                             ----- first introduced with type `&types::Function` here       ^^^^^ expected struct `types::Function`, found struct `types::Method`
     |
     = note: in the same arm, a binding must have the same type in all alternatives
