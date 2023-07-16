plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 5 arguments but 6 arguments were supplied
    --> src/librustdoc/clean/mod.rs:1998:18
     |
1998 |             vec![Item::from_def_id_and_attrs_and_parts(
...
2003 |                 cx,
     |                 -- argument of type `&mut DocContext<'_>` unexpected
     |
---
478  |         name: Option<Symbol>,
     |         --------------------
479  |         kind: ItemKind,
     |         --------------
480  |         attrs: Box<Attributes>,
     |         ----------------------
481  |         cfg: Option<Arc<Cfg>>,
help: remove the extra argument
     |
     |
1998 |             vec![Item::from_def_id_and_attrs_and_parts(def_id, Some(name), kind, Box::new(attrs), cfg)]

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:04:15
