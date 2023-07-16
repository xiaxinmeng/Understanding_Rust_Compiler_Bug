
error[E0107]: wrong number of type arguments: expected 3, found 2
  --> src/librustc_macros/src/symbols.rs:50:17
   |
50 | struct Keywords(IndexMap<Option<Ident>, Vec<Keyword>>);
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 type arguments
